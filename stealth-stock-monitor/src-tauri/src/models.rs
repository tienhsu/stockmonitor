use serde::{Deserialize, Serialize};

/// 股票基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stock {
    /// 唯一标识，如 "sh600519"
    pub id: String,
    /// 股票代码，如 "600519"
    pub code: String,
    /// 市场前缀，如 "sh" / "sz"
    pub market: String,
    /// 自定义别名，如 "茅台"
    pub alias: String,
    /// 是否在悬浮窗中显示
    pub visible: bool,
}

/// 实时行情数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceUpdate {
    /// 唯一标识，如 "sh600519"
    pub id: String,
    /// 股票代码
    pub code: String,
    /// 市场
    pub market: String,
    /// 股票名称
    pub name: String,
    /// 当前价格
    pub price: f64,
    /// 昨收价
    pub prev_close: f64,
    /// 涨跌额
    pub change: f64,
    /// 涨跌幅（小数，如 0.0575 表示 5.75%）
    pub percent: f64,
    /// 今日最高
    pub high: f64,
    /// 今日最低
    pub low: f64,
    /// 时间戳（毫秒）
    pub timestamp: u64,
    /// 数据源标识
    pub source: String,
}

/// 窗口配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub always_on_top: bool,
    pub opacity: f64,
    pub position: Position,
    pub size: Size,
    pub hide_in_taskbar: bool,
    pub click_through: bool,
    pub display_rows: u32,
    pub enable_carousel: bool,      // 是否启用自动轮播
    pub carousel_interval_ms: u64,  // 轮播间隔（毫秒）
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub theme: String,
    pub up_color: String,
    pub down_color: String,
    pub neutral_color: String,
    pub font_size: String,
    pub refresh_interval_ms: u64,
    pub pause_when_hidden: bool,
    pub autostart: bool,
    pub data_sources: Vec<String>,
}

/// 快捷键配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutConfig {
    pub toggle_visible: String,
    pub next_stock: String,
    pub prev_stock: String,
}

/// 完整应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: u32,
    pub window: WindowConfig,
    pub app: AppConfig,
    pub shortcuts: ShortcutConfig,
    pub stocks: Vec<Stock>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            window: WindowConfig {
                always_on_top: true,
                opacity: 0.85,
                position: Position { x: 100, y: 100 },
                size: Size {
                    width: 300,
                    height: 80,
                },
                hide_in_taskbar: true,
                click_through: false,
                display_rows: 1,
                enable_carousel: false,        // 默认禁用轮播
                carousel_interval_ms: 5000,    // 默认5秒
            },
            app: AppConfig {
                theme: "auto".to_string(),
                up_color: "#ff0000".to_string(),
                down_color: "#00b300".to_string(),
                neutral_color: "#888888".to_string(),
                font_size: "medium".to_string(),
                refresh_interval_ms: 3000,
                pause_when_hidden: false,
                autostart: false,
                data_sources: vec![
                    "sina".to_string(),
                    "tencent".to_string(),
                    "eastmoney".to_string(),
                ],
            },
            shortcuts: ShortcutConfig {
                toggle_visible: "CommandOrControl+Shift+S".to_string(),
                next_stock: "Alt+J".to_string(),
                prev_stock: "Alt+K".to_string(),
            },
            stocks: vec![
                Stock {
                    id: "sh600519".to_string(),
                    code: "600519".to_string(),
                    market: "sh".to_string(),
                    alias: "茅台".to_string(),
                    visible: true,
                },
            ],
        }
    }
}

/// 数据源切换通知
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceSwitchedEvent {
    pub from: String,
    pub to: String,
}

/// 错误通知
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorEvent {
    pub code: String,
    pub message: String,
}
