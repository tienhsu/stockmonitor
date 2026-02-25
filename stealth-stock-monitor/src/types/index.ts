// ==================== 数据模型 ====================

/** 股票基本信息 */
export interface Stock {
  id: string;       // "sh600519"
  code: string;     // "600519"
  market: string;   // "sh"
  alias: string;    // "茅台"
  visible: boolean;
}

/** 实时行情数据 */
export interface PriceUpdate {
  id: string;
  code: string;
  market: string;
  name: string;
  price: number;
  prev_close: number;
  change: number;
  percent: number;  // 小数，如 0.0575
  high: number;
  low: number;
  timestamp: number;
  source: string;
}

// ==================== 配置结构 ====================

export interface Position {
  x: number;
  y: number;
}

export interface WindowSize {
  width: number;
  height: number;
}

export interface WindowConfig {
  always_on_top: boolean;
  opacity: number;
  position: Position;
  size: WindowSize;
  hide_in_taskbar: boolean;
  click_through: boolean;
  display_rows: number;
  enable_carousel: boolean;      // 是否启用自动轮播
  carousel_interval_ms: number;  // 轮播间隔（毫秒）
}

export interface AppConfig {
  theme: 'auto' | 'dark' | 'light';
  up_color: string;
  down_color: string;
  neutral_color: string;
  font_size: 'small' | 'medium' | 'large';
  refresh_interval_ms: number;
  pause_when_hidden: boolean;
  autostart: boolean;
  data_sources: string[];
}

export interface ShortcutConfig {
  toggle_visible: string;
  next_stock: string;
  prev_stock: string;
  open_settings: string;  // 打开设置窗口（用于鼠标穿透后无法通过右键菜单进入设置的情况）
}

export interface Config {
  version: number;
  window: WindowConfig;
  app: AppConfig;
  shortcuts: ShortcutConfig;
  stocks: Stock[];
}

// ==================== 事件 Payload ====================

export interface SourceSwitchedEvent {
  from: string;
  to: string;
}

export interface ErrorEvent {
  code: string;
  message: string;
}
