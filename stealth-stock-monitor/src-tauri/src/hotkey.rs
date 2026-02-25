use tauri::{AppHandle, Manager, Emitter};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};
use anyhow::Result;
use crate::config::ConfigStore;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// 全局快捷键管理器
pub struct HotkeyManager {
    app: AppHandle,
    pub config_store: Arc<ConfigStore>,
}

impl HotkeyManager {
    pub fn new(app: AppHandle, config_store: Arc<ConfigStore>) -> Self {
        Self { app, config_store }
    }

    /// 注册所有快捷键
    pub fn register_all(&self) -> Result<()> {
        let config = self.config_store.get();
        
        // 注册显示/隐藏快捷键
        self.register_toggle_visible(&config.shortcuts.toggle_visible)?;
        
        // 注册股票切换快捷键
        self.register_next_stock(&config.shortcuts.next_stock)?;
        self.register_prev_stock(&config.shortcuts.prev_stock)?;

        // 注册打开设置快捷键
        self.register_open_settings(&config.shortcuts.open_settings)?;

        log::info!("全局快捷键已注册");
        Ok(())
    }

    /// 注销所有快捷键
    pub fn unregister_all(&self) -> Result<()> {
        self.app.global_shortcut().unregister_all()?;
        log::info!("全局快捷键已注销");
        Ok(())
    }

    /// 重新注册快捷键（配置变更时调用）
    pub fn reload(&self) -> Result<()> {
        self.unregister_all()?;
        self.register_all()?;
        Ok(())
    }

    fn register_toggle_visible(&self, shortcut_str: &str) -> Result<()> {
        let app = self.app.clone();
        let shortcut: Shortcut = shortcut_str.parse()?;
        
        let last_trigger = Arc::new(Mutex::new(Instant::now()));
        
        self.app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, event| {
            if event.state() == ShortcutState::Pressed {
                let mut last = last_trigger.lock().unwrap();
                if last.elapsed() < Duration::from_millis(300) {
                    return;
                }
                *last = Instant::now();

                log::info!("快捷键 triggered: toggle_visible");
                if let Some(window) = app.get_webview_window("monitor") {
                    let is_visible = window.is_visible().unwrap_or(false);
                    log::info!("当前窗口可见性: {}", is_visible);
                    if is_visible {
                        let _ = window.hide();
                        log::info!("执行 hide()");
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                        log::info!("执行 show()");
                    }
                } else {
                    log::error!("找不到 monitor 窗口");
                }
            }
        })?;
        
        Ok(())
    }

    fn register_next_stock(&self, shortcut_str: &str) -> Result<()> {
        let config_store = self.config_store.clone();
        let app = self.app.clone();
        let shortcut: Shortcut = shortcut_str.parse()?;
        
        let last_trigger = Arc::new(Mutex::new(Instant::now()));

        self.app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, event| {
            if event.state() == ShortcutState::Pressed {
                let mut last = last_trigger.lock().unwrap();
                if last.elapsed() < Duration::from_millis(300) {
                    return;
                }
                *last = Instant::now();
                
                log::info!("快捷键 triggered: next_stock");
                let config = config_store.get();
                let visible_stocks: Vec<_> = config.stocks.iter().filter(|s| s.visible).collect();
                
                log::info!("Visible stocks count: {}", visible_stocks.len());

                if visible_stocks.len() <= 1 {
                    log::info!("股票数量不足，无需切换");
                    return; // 只有一只或没有股票，无需切换
                }
                
                // 简化实现：通过前端事件通知切换
                let _ = app.emit("hotkey-next-stock", ());
                log::info!("已发送 hotkey-next-stock 事件");
            }
        })?;
        
        Ok(())
    }

    fn register_prev_stock(&self, shortcut_str: &str) -> Result<()> {
        let config_store = self.config_store.clone();
        let app = self.app.clone();
        let shortcut: Shortcut = shortcut_str.parse()?;

        let last_trigger = Arc::new(Mutex::new(Instant::now()));

        self.app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, event| {
            if event.state() == ShortcutState::Pressed {
                let mut last = last_trigger.lock().unwrap();
                if last.elapsed() < Duration::from_millis(300) {
                    return;
                }
                *last = Instant::now();

                let config = config_store.get();
                let visible_stocks: Vec<_> = config.stocks.iter().filter(|s| s.visible).collect();

                log::info!("Visible stocks count (prev): {}", visible_stocks.len());

                if visible_stocks.len() <= 1 {
                    return;
                }

                let _ = app.emit("hotkey-prev-stock", ());
            }
        })?;

        Ok(())
    }

    /// 注册打开设置快捷键（用于鼠标穿透后无法通过右键菜单进入设置的情况）
    fn register_open_settings(&self, shortcut_str: &str) -> Result<()> {
        let app = self.app.clone();
        let shortcut: Shortcut = shortcut_str.parse()?;

        let last_trigger = Arc::new(Mutex::new(Instant::now()));

        self.app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, event| {
            if event.state() == ShortcutState::Pressed {
                let mut last = last_trigger.lock().unwrap();
                if last.elapsed() < Duration::from_millis(300) {
                    return;
                }
                *last = Instant::now();

                log::info!("快捷键 triggered: open_settings");
                if let Some(window) = app.get_webview_window("settings") {
                    let _ = window.show();
                    let _ = window.set_focus();
                    log::info!("已打开设置窗口");
                } else {
                    log::error!("找不到 settings 窗口");
                }
            }
        })?;

        Ok(())
    }
}
