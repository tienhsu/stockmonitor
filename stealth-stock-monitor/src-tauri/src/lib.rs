pub mod models;
pub mod config;
pub mod sources;
pub mod poller;
pub mod hotkey;

use std::sync::Arc;
use tauri::{Manager, Emitter, AppHandle, State};
use tauri::tray::{TrayIconBuilder, MouseButton, MouseButtonState, TrayIconEvent};
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use models::{Config, Stock};
use config::ConfigStore;
use poller::Poller;

/// 应用状态
pub struct AppState {
    pub config_store: Arc<ConfigStore>,
    pub poller: Poller,
    pub hotkey_manager: Arc<hotkey::HotkeyManager>,
}

// ==================== Tauri Commands ====================

/// 获取完整配置
#[tauri::command]
fn get_config(state: State<AppState>) -> Result<Config, String> {
    Ok(state.config_store.get())
}

/// 更新配置
#[tauri::command]
fn update_config(
    state: State<AppState>,
    app: AppHandle,
    config: Config,
) -> Result<(), String> {
    // 检查快捷键是否有变更
    let old_config = state.config_store.get();
    let shortcuts_changed = old_config.shortcuts != config.shortcuts;

    state
        .config_store
        .update(config.clone())
        .map_err(|e| e.to_string())?;

    // 如果快捷键配置有变更，重新注册快捷键
    if shortcuts_changed {
        if let Err(e) = state.hotkey_manager.reload() {
            log::error!("重新注册快捷键失败: {}", e);
        }
    }

    let _ = app.emit("config-changed", &config);
    Ok(())
}

/// 添加股票
#[tauri::command]
fn add_stock(state: State<AppState>, app: AppHandle, code: String) -> Result<Stock, String> {
    let market = sources::detect_market(&code);
    let id = sources::make_stock_id(market, &code);

    let stock = Stock {
        id: id.clone(),
        code: code.clone(),
        market: market.to_string(),
        alias: String::new(), // 后续通过数据源获取名称
        visible: true,
    };

    state
        .config_store
        .add_stock(stock.clone())
        .map_err(|e| e.to_string())?;

    // 通知配置变更
    let _ = app.emit("config-changed", &state.config_store.get());
    Ok(stock)
}

/// 移除股票
#[tauri::command]
fn remove_stock(state: State<AppState>, app: AppHandle, id: String) -> Result<(), String> {
    state
        .config_store
        .remove_stock(&id)
        .map_err(|e| e.to_string())?;
    let _ = app.emit("config-changed", &state.config_store.get());
    Ok(())
}

/// 重新排序股票
#[tauri::command]
fn reorder_stocks(state: State<AppState>, app: AppHandle, ids: Vec<String>) -> Result<(), String> {
    state
        .config_store
        .reorder_stocks(&ids)
        .map_err(|e| e.to_string())?;
    let _ = app.emit("config-changed", &state.config_store.get());
    Ok(())
}

/// 控制窗口显隐
#[tauri::command]
fn set_window_visible(app: AppHandle, visible: bool) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("monitor") {
        if visible {
            window.show().map_err(|e| e.to_string())?;
        } else {
            window.hide().map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

/// 设置鼠标穿透
#[tauri::command]
fn set_ignore_cursor_events(app: AppHandle, ignore: bool) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("monitor") {
        window.set_ignore_cursor_events(ignore).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 打开设置窗口
#[tauri::command]
fn open_settings(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("settings") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 立即刷新数据
#[tauri::command]
fn force_refresh() -> Result<(), String> {
    // 轮询器会在下一个 tick 自动获取最新数据
    // 这里可以通过信号立即触发，暂时使用简化实现
    Ok(())
}

/// 调整悬浮窗口大小（由前端根据内容动态计算）
#[tauri::command]
fn resize_monitor_window(app: AppHandle, width: f64, height: f64) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("monitor") {
        let size = tauri::LogicalSize::new(width, height);
        window.set_size(size).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 在悬浮窗口上弹出原生右键菜单
#[tauri::command]
async fn show_context_menu(app: AppHandle) -> Result<(), String> {
    log::info!("显示右键菜单");
    
    let window = app.get_webview_window("monitor")
        .ok_or("找不到 monitor 窗口".to_string())?;

    let refresh = MenuItemBuilder::with_id("ctx_refresh", "刷新")
        .build(&app).map_err(|e| e.to_string())?;
    let settings = MenuItemBuilder::with_id("ctx_settings", "设置")
        .build(&app).map_err(|e| e.to_string())?;
    let hide = MenuItemBuilder::with_id("ctx_hide", "隐藏")
        .build(&app).map_err(|e| e.to_string())?;
    let quit = MenuItemBuilder::with_id("ctx_quit", "退出")
        .build(&app).map_err(|e| e.to_string())?;

    let menu = MenuBuilder::new(&app)
        .item(&refresh)
        .item(&settings)
        .separator()
        .item(&hide)
        .item(&quit)
        .build()
        .map_err(|e| e.to_string())?;
    
    // 使用 window.popup_menu 在鼠标位置弹出
    log::info!("调用 popup_menu");
    window.popup_menu(&menu).map_err(|e| {
        log::error!("popup_menu 失败: {}", e);
        e.to_string()
    })?;
    
    log::info!("菜单弹出成功");
    Ok(())
}

// ==================== 应用入口 ====================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            get_config,
            update_config,
            add_stock,
            remove_stock,
            reorder_stocks,
            set_window_visible,
            set_ignore_cursor_events,
            open_settings,
            force_refresh,
            resize_monitor_window,
            show_context_menu,
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "settings" {
                    // 阻止关闭，改为隐藏
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .setup(move |app| {
            let app_handle = app.handle().clone();
            let config_store = Arc::new(ConfigStore::new().expect("无法初始化配置"));

            // 创建并注册全局快捷键管理器
            let hotkey_manager = hotkey::HotkeyManager::new(
                app_handle.clone(),
                config_store.clone(),
            );
            if let Err(e) = hotkey_manager.register_all() {
                log::error!("注册全局快捷键失败: {}", e);
            }

            // 创建 AppState，包含 hotkey_manager
            app.manage(AppState {
                config_store: config_store.clone(),
                poller: Poller::new(),
                hotkey_manager: Arc::new(hotkey_manager),
            });

            // 启动数据轮询
            let state: State<AppState> = app.state();
            state.poller.start(app_handle.clone(), state.config_store.clone());

            // 设置应用菜单（系统菜单）
            setup_app_menu(app)?;

            // 设置系统托盘
            setup_tray(app)?;

            // 注册全局菜单事件处理器（用于右键菜单和系统菜单）
            app.on_menu_event(move |app, event| {
                log::info!("菜单事件: {:?}", event.id());
                match event.id().as_ref() {
                    // 系统菜单事件
                    "about" => {
                        log::info!("关于");
                        if let Some(win) = app.get_webview_window("settings") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                    "app_settings" => {
                        log::info!("打开设置（系统菜单）");
                        if let Some(win) = app.get_webview_window("settings") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                    "hide" => {
                        log::info!("隐藏应用");
                        if let Some(win) = app.get_webview_window("monitor") {
                            let _ = win.hide();
                        }
                    }
                    "hide_others" => {
                        log::info!("隐藏其他应用");
                        // TODO: 实现隐藏其他应用
                    }
                    "show_all" => {
                        log::info!("全部显示");
                        if let Some(win) = app.get_webview_window("monitor") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                    "app_quit" => {
                        log::info!("退出应用（系统菜单）");
                        app.exit(0);
                    }
                    // 右键菜单事件
                    "ctx_refresh" => {
                        log::info!("刷新数据");
                        // 轮询器会自动刷新
                    }
                    "ctx_settings" => {
                        log::info!("打开设置（右键菜单）");
                        if let Some(win) = app.get_webview_window("settings") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                    "ctx_hide" => {
                        log::info!("隐藏窗口");
                        if let Some(win) = app.get_webview_window("monitor") {
                            let _ = win.hide();
                        }
                    }
                    "ctx_quit" => {
                        log::info!("退出应用（右键菜单）");
                        app.exit(0);
                    }
                    // 系统托盘菜单事件
                    "settings" => {
                        log::info!("打开设置（托盘菜单）");
                        if let Some(win) = app.get_webview_window("settings") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                    "quit" => {
                        log::info!("退出应用（托盘菜单）");
                        app.exit(0);
                    }
                    _ => {}
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("启动应用失败");
}

/// 设置应用菜单（系统菜单，macOS 显示在左上角应用名称下）
fn setup_app_menu(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let app_name = "Stealth Stock Monitor";

    // 创建应用菜单项
    let about = MenuItemBuilder::with_id("about", format!("关于 {}", app_name))
        .build(app)?;
    let separator1 = tauri::menu::PredefinedMenuItem::separator(app)?;
    let settings = MenuItemBuilder::with_id("app_settings", "设置...")
        .build(app)?;
    let separator2 = tauri::menu::PredefinedMenuItem::separator(app)?;
    let hide = MenuItemBuilder::with_id("hide", format!("隐藏 {}", app_name))
        .accelerator("CommandOrControl+H")
        .build(app)?;
    let hide_others = MenuItemBuilder::with_id("hide_others", "隐藏其他")
        .accelerator("CommandOrControl+Shift+H")
        .build(app)?;
    let show_all = MenuItemBuilder::with_id("show_all", "全部显示")
        .build(app)?;
    let separator3 = tauri::menu::PredefinedMenuItem::separator(app)?;
    let quit = MenuItemBuilder::with_id("app_quit", format!("退出 {}", app_name))
        .accelerator("CommandOrControl+Q")
        .build(app)?;

    // 创建应用子菜单（macOS 左上角应用名称下的菜单）
    let app_menu = tauri::menu::SubmenuBuilder::with_id(app, "app_menu", app_name)
        .items(&[
            &about,
            &separator1,
            &settings,
            &separator2,
            &hide,
            &hide_others,
            &show_all,
            &separator3,
            &quit,
        ])
        .build()?;

    // 设置应用系统菜单
    app.set_menu(
        tauri::menu::MenuBuilder::new(app)
            .item(&app_menu)
            .build()?,
    )?;

    Ok(())
}

/// 设置系统托盘
fn setup_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let settings_item = MenuItemBuilder::with_id("settings", "设置").build(app)?;
    let quit_item = MenuItemBuilder::with_id("quit", "退出").build(app)?;
    let menu = MenuBuilder::new(app)
        .item(&settings_item)
        .separator()
        .item(&quit_item)
        .build()?;

    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .on_menu_event(|app, event| {
            match event.id().as_ref() {
                "settings" => {
                    if let Some(window) = app.get_webview_window("settings") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("monitor") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        })
        .build(app)?;

    Ok(())
}
