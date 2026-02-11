use std::path::PathBuf;
use std::sync::RwLock;
use anyhow::{Context, Result};
use crate::models::Config;

/// 获取配置文件路径
/// macOS: ~/Library/Application Support/com.wolf.stealth-stock-monitor/config.json
/// Windows: %APPDATA%/com.wolf.stealth-stock-monitor/config.json
pub fn get_config_dir() -> Result<PathBuf> {
    let dir = dirs_next()
        .ok_or_else(|| anyhow::anyhow!("无法确定应用配置目录"))?;
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// 使用平台特定路径
fn dirs_next() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        dirs::config_dir().map(|p| p.join("com.wolf.stealth-stock-monitor"))
    }
    #[cfg(target_os = "windows")]
    {
        dirs::config_dir().map(|p| p.join("com.wolf.stealth-stock-monitor"))
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        dirs::config_dir().map(|p| p.join("stealth-stock-monitor"))
    }
}

fn config_file_path() -> Result<PathBuf> {
    Ok(get_config_dir()?.join("config.json"))
}

/// 从文件加载配置
pub fn load_config() -> Result<Config> {
    let path = config_file_path()?;

    if !path.exists() {
        // 文件不存在时创建默认配置
        let config = Config::default();
        save_config(&config)?;
        return Ok(config);
    }

    let content = std::fs::read_to_string(&path)
        .context("读取配置文件失败")?;

    match serde_json::from_str::<Config>(&content) {
        Ok(config) => Ok(config),
        Err(e) => {
            // JSON 解析失败：备份损坏文件，使用默认配置
            log::warn!("配置文件解析失败，使用默认配置: {}", e);
            let backup_path = path.with_extension("json.bak");
            let _ = std::fs::copy(&path, &backup_path);
            let config = Config::default();
            save_config(&config)?;
            Ok(config)
        }
    }
}

/// 保存配置到文件
pub fn save_config(config: &Config) -> Result<()> {
    let path = config_file_path()?;
    let content = serde_json::to_string_pretty(config)
        .context("序列化配置失败")?;
    std::fs::write(&path, content)
        .context("写入配置文件失败")?;
    Ok(())
}

/// 全局配置状态（线程安全）
pub struct ConfigStore {
    config: RwLock<Config>,
}

impl ConfigStore {
    pub fn new() -> Result<Self> {
        let config = load_config()?;
        Ok(Self {
            config: RwLock::new(config),
        })
    }

    /// 读取当前配置的克隆副本
    pub fn get(&self) -> Config {
        self.config.read().unwrap().clone()
    }

    /// 更新配置并持久化
    pub fn update(&self, new_config: Config) -> Result<()> {
        save_config(&new_config)?;
        let mut config = self.config.write().unwrap();
        *config = new_config;
        Ok(())
    }

    /// 添加股票
    pub fn add_stock(&self, stock: crate::models::Stock) -> Result<()> {
        let mut config = self.config.write().unwrap();
        // 重复检查
        if config.stocks.iter().any(|s| s.id == stock.id) {
            return Err(anyhow::anyhow!("股票已在列表中: {}", stock.id));
        }
        config.stocks.push(stock);
        save_config(&config)?;
        Ok(())
    }

    /// 移除股票
    pub fn remove_stock(&self, id: &str) -> Result<()> {
        let mut config = self.config.write().unwrap();
        config.stocks.retain(|s| s.id != id);
        save_config(&config)?;
        Ok(())
    }

    /// 重新排序股票
    pub fn reorder_stocks(&self, ids: &[String]) -> Result<()> {
        let mut config = self.config.write().unwrap();
        let mut reordered = Vec::new();
        for id in ids {
            if let Some(stock) = config.stocks.iter().find(|s| &s.id == id) {
                reordered.push(stock.clone());
            }
        }
        config.stocks = reordered;
        save_config(&config)?;
        Ok(())
    }
}
