use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use tokio::time::{interval, Duration};
use tauri::{AppHandle, Emitter};
use anyhow::Result;

use crate::config::ConfigStore;
use crate::models::PriceUpdate;
use crate::sources::{DataSource, sina::SinaSource, tencent::TencentSource, eastmoney::EastmoneySource};

/// 数据源管理器（含故障转移）
struct SourceManager {
    sources: Vec<Box<dyn DataSource>>,
    active_index: usize,
    fail_count: u32,
    max_failures: u32,
}

impl SourceManager {
    fn new(source_names: &[String]) -> Self {
        let mut sources: Vec<Box<dyn DataSource>> = Vec::new();
        for name in source_names {
            match name.as_str() {
                "sina" => sources.push(Box::new(SinaSource::new())),
                "tencent" => sources.push(Box::new(TencentSource::new())),
                "eastmoney" => sources.push(Box::new(EastmoneySource::new())),
                _ => log::warn!("未知数据源: {}", name),
            }
        }
        if sources.is_empty() {
            // 默认至少有一个
            sources.push(Box::new(SinaSource::new()));
        }
        Self {
            sources,
            active_index: 0,
            fail_count: 0,
            max_failures: 3,
        }
    }

    #[cfg(test)]
    fn new_with_sources(sources: Vec<Box<dyn DataSource>>) -> Self {
        Self {
            sources,
            active_index: 0,
            fail_count: 0,
            max_failures: 3,
        }
    }

    /// 获取当前活动数据源名称
    fn active_name(&self) -> &str {
        self.sources[self.active_index].name()
    }

    /// 尝试获取数据，失败时自动切换数据源
    async fn fetch(&mut self, stocks: &[(String, String)]) -> Result<Vec<PriceUpdate>> {
        let source = &self.sources[self.active_index];
        match source.fetch(stocks).await {
            Ok(data) => {
                self.fail_count = 0;
                Ok(data)
            }
            Err(e) => {
                self.fail_count += 1;
                log::warn!(
                    "数据源 {} 请求失败 ({}/{}): {}",
                    self.active_name(),
                    self.fail_count,
                    self.max_failures,
                    e
                );

                if self.fail_count >= self.max_failures && self.sources.len() > 1 {
                    let old_name = self.active_name().to_string();
                    self.active_index = (self.active_index + 1) % self.sources.len();
                    self.fail_count = 0;
                    let new_name = self.active_name().to_string();
                    log::info!("数据源切换: {} -> {}", old_name, new_name);
                    return Err(anyhow::anyhow!(
                        "source_switched:{}:{}",
                        old_name,
                        new_name
                    ));
                }

                Err(e)
            }
        }
    }
}

/// 轮询器状态
pub struct Poller {
    is_paused: Arc<AtomicBool>,
    is_running: Arc<AtomicBool>,
}

impl Poller {
    pub fn new() -> Self {
        Self {
            is_paused: Arc::new(AtomicBool::new(false)),
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    /// 设置暂停状态
    pub fn set_paused(&self, paused: bool) {
        self.is_paused.store(paused, Ordering::Relaxed);
    }

    /// 启动轮询任务
    pub fn start(&self, app_handle: AppHandle, config_store: Arc<ConfigStore>) {
        if self.is_running.load(Ordering::Relaxed) {
            return;
        }
        self.is_running.store(true, Ordering::Relaxed);

        let is_paused = self.is_paused.clone();
        let is_running = self.is_running.clone();

        tauri::async_runtime::spawn(async move {
            let mut config = config_store.get();
            let mut current_sources = config.app.data_sources.clone();
            let mut source_manager = SourceManager::new(&current_sources);
            let mut tick_interval = interval(Duration::from_millis(config.app.refresh_interval_ms));

            loop {
                tick_interval.tick().await;

                if !is_running.load(Ordering::Relaxed) {
                    break;
                }

                if is_paused.load(Ordering::Relaxed) {
                    continue;
                }

                config = config_store.get();

                // 检查数据源配置是否变更
                if config.app.data_sources != current_sources {
                    log::info!("数据源配置变更，重建 SourceManager");
                    current_sources = config.app.data_sources.clone();
                    source_manager = SourceManager::new(&current_sources);
                }

                // 收集要请求的股票列表
                let stocks: Vec<(String, String)> = config
                    .stocks
                    .iter()
                    .filter(|s| s.visible)
                    .map(|s| (s.market.clone(), s.code.clone()))
                    .collect();

                if stocks.is_empty() {
                    continue;
                }

                match source_manager.fetch(&stocks).await {
                    Ok(updates) => {
                        if !updates.is_empty() {
                            let _ = app_handle.emit("price-update", &updates);
                        }
                    }
                    Err(e) => {
                        let err_msg = e.to_string();
                        if err_msg.starts_with("source_switched:") {
                            // 解析数据源切换事件
                            let parts: Vec<&str> = err_msg.splitn(3, ':').collect();
                            if parts.len() == 3 {
                                let _ = app_handle.emit(
                                    "source-switched",
                                    serde_json::json!({
                                        "from": parts[1],
                                        "to": parts[2]
                                    }),
                                );
                            }
                        } else {
                            let _ = app_handle.emit(
                                "error",
                                serde_json::json!({
                                    "code": "FETCH_ERROR",
                                    "message": format!("数据获取失败: {}", e)
                                }),
                            );
                        }
                    }
                }

                // 检查刷新间隔是否变更
                let new_interval = config_store.get().app.refresh_interval_ms;
                if new_interval != config.app.refresh_interval_ms {
                    tick_interval = interval(Duration::from_millis(new_interval));
                }
            }
        });
    }

    /// 停止轮询
    pub fn stop(&self) {
        self.is_running.store(false, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockSource {
        name: String,
        should_fail: bool,
        call_count: Arc<AtomicUsize>,
    }

    #[async_trait::async_trait]
    impl DataSource for MockSource {
        fn name(&self) -> &str {
            &self.name
        }
        async fn fetch(&self, _stocks: &[(String, String)]) -> Result<Vec<PriceUpdate>> {
            self.call_count.fetch_add(1, Ordering::SeqCst);
            if self.should_fail {
                Err(anyhow::anyhow!("Mock failure"))
            } else {
                Ok(vec![])
            }
        }
    }

    #[tokio::test]
    async fn test_failover() {
        let calls_1 = Arc::new(AtomicUsize::new(0));
        let calls_2 = Arc::new(AtomicUsize::new(0));

        let s1 = MockSource {
            name: "s1".to_string(),
            should_fail: true,
            call_count: calls_1.clone(),
        };
        let s2 = MockSource {
            name: "s2".to_string(),
            should_fail: false,
            call_count: calls_2.clone(),
        };

        // Note: new_with_sources uses default max_failures = 3
        let mut manager = SourceManager::new_with_sources(vec![Box::new(s1), Box::new(s2)]);

        // 1. Fail 1
        let _ = manager.fetch(&[]).await;
        assert_eq!(manager.active_index, 0); 
        assert_eq!(manager.fail_count, 1);

        // 2. Fail 2
        let _ = manager.fetch(&[]).await;
        assert_eq!(manager.active_index, 0);
        assert_eq!(manager.fail_count, 2);

        // 3. Fail 3 -> Switch
        let result = manager.fetch(&[]).await;
        
        match result {
            Err(e) => {
                let msg = e.to_string();
                assert!(msg.contains("source_switched"));
                assert!(msg.contains("s1"));
                assert!(msg.contains("s2"));
            }
            _ => panic!("Expected switch error"),
        }
        
        // Assert switched
        assert_eq!(manager.active_index, 1);
        assert_eq!(manager.fail_count, 0); 

        // 4. Success on s2
        let _ = manager.fetch(&[]).await;
        assert_eq!(calls_2.load(Ordering::SeqCst), 1);
    }
}
