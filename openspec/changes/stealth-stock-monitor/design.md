## Context

当前市场上主流的股票软件窗口庞大、信息繁杂，在办公场景下使用极易暴露。用户需要一款"隐形"工具，仅在需要时通过快捷键唤起，展示最核心的价格信息。

本项目是从零开始构建的跨平台桌面应用 **Stealth Stock Monitor**。核心约束是"轻量"与"隐蔽"。应用必须支持 Windows 和 macOS，且安装包尽可能小，运行时内存占用低。

## Goals / Non-Goals

**Goals:**
- **极致轻量**：安装包大小 < 10MB，运行时内存占用 < 50MB。
- **瞬时响应**：全局快捷键隐藏/显示窗口需无感知延迟 (<100ms)。
- **深度隐蔽**：
    - 主窗口无边框、无标题栏。
    - **支持在任务栏/Dock 中隐藏图标**（仅保留托盘图标或完全通过快捷键唤起）。
    - 支持窗口透明度调节。
- **高可用数据**：支持多数据源（新浪/腾讯/东财）自动故障转移，确保行情不断连。

**Non-Goals:**
- **不支持交易功能**：仅作为行情查看器，不涉及账户登录与交易。
- **不支持复杂图表**：不提供 K 线图、分时图等复杂可视化，仅展示文字/数字。
- **不提供移动端**：仅专注于桌面端（Windows/macOS）。
- **不支持港股/美股实时行情**：MVP 阶段仅支持 A 股，港美股为延迟数据。

## UI/UX Design

### 1. 悬浮窗 (Floating Window)
*   **布局**: 极简单行或多行文本列表。
    *   *左侧*: 股票名称/代码 (可配置显示哪一个)
    *   *中间*: 当前价格
    *   *右侧*: 涨跌幅 (颜色可自定义：红涨绿跌/红跌绿涨/无色)
*   **交互**:
    *   **拖拽**: 窗口任意位置按住鼠标左键可拖动窗口。
    *   **右键菜单**:
        *   `Refresh`: 立即刷新
        *   `Settings`: 打开设置窗口
        *   `Hide`: 隐藏当前窗口 (等同于快捷键)
        *   `Quit`: 退出应用
    *   **鼠标穿透 (可选)**: 在设置中开启后，鼠标点击将穿透窗口（仅通过快捷键控制显隐），防止误触。

### 2. 设置窗口 (Settings Window)
*   **股票管理**: List + Add/Remove/Sort。支持搜索股票代码。
*   **外观配置**: 透明度滑块 (0.1 - 1.0)、字体大小、颜色方案。
*   **快捷键录制**: 这是一个 Input Box，按下键盘组合键即记录。

### 3. 系统托盘 (System Tray)
*   **Click / Left Click**: 触发 `Show/Hide` 切换。
*   **Right Click Menu**: `Settings`, `Quit`.

## 项目结构 (Project Structure)

```
stealth-stock-monitor/
├── src-tauri/              # Rust 后端
│   ├── src/
│   │   ├── main.rs         # 应用入口
│   │   ├── window.rs       # 窗口管理模块
│   │   ├── hotkey.rs       # 全局快捷键
│   │   ├── poller.rs       # 数据轮询器
│   │   ├── config.rs       # 配置管理
│   │   ├── sources/        # 数据源适配器
│   │   │   ├── mod.rs
│   │   │   ├── sina.rs
│   │   │   ├── tencent.rs
│   │   │   └── eastmoney.rs
│   │   └── models.rs       # 数据模型
│   ├── Cargo.toml
│   └── tauri.conf.json     # Tauri 配置
├── src/                    # React 前端
│   ├── App.tsx
│   ├── windows/
│   │   ├── FloatingWindow.tsx
│   │   └── SettingsWindow.tsx
│   ├── components/
│   │   ├── StockItem.tsx
│   │   └── StockList.tsx
│   ├── hooks/
│   │   ├── useStockData.ts
│   │   └── useConfig.ts
│   └── types/
│       └── index.ts
└── package.json
```

## 技术栈细节 (Tech Stack Details)

### 前端状态管理
- **React Context + useReducer**: 管理全局配置和股票数据
- **Zustand** (可选): 如果状态复杂度增加，可引入轻量级状态库
- **事件驱动**: 通过 Tauri Event 监听后端推送，单向数据流

### 数据源 API 详细说明

#### 1. 新浪财经 API (主数据源)
```
GET https://hq.sinajs.cn/list={stock_codes}
示例: https://hq.sinajs.cn/list=sh600519,sz000001

返回格式 (CSV):
var hq_str_sh600519="贵州茅台,1750.00,1740.00,1755.00,1760.00,1745.00,...";
                    [0]名称 [1]今开 [2]昨收 [3]现价 [4]最高 [5]最低 ...

解析字段:
- name: [0]
- current_price: [3]
- prev_close: [2]
- percent: ((current - prev) / prev) * 100
```

#### 2. 腾讯证券 API (备用源)
```
GET http://qt.gtimg.cn/q={stock_codes}
示例: http://qt.gtimg.cn/q=sh600519

返回格式:
v_sh600519="51~贵州茅台~600519~1755.00~1740.00~...";
           [3]现价 [4]昨收
```

#### 3. 东方财富 API (备用源)
```
GET http://push2.eastmoney.com/api/qt/stock/get?secid={market}.{code}
示例: http://push2.eastmoney.com/api/qt/stock/get?secid=1.600519

返回格式 (JSON):
{
  "data": {
    "f43": 1755.00,  // 现价
    "f60": 1740.00   // 昨收
  }
}
```

## Architecture & Data Design

### 1. 数据存储结构 (Data Persistence)
使用 `config.json` (或 `.toml`) 存储在用户配置目录下 (`~/.config/stealth-stock/` or `%APPDATA%`).

```json
{
  "version": 1,
  "window": {
    "always_on_top": true,
    "opacity": 0.8,
    "position": { "x": 100, "y": 100 },
    "size": { "width": 300, "height": 80 },
    "hide_in_taskbar": true,
    "click_through": false,
    "display_rows": 1
  },
  "app": {
    "theme": "auto",
    "up_color": "#ff0000",
    "down_color": "#00ff00",
    "neutral_color": "#888888",
    "font_size": "medium",
    "refresh_interval_ms": 3000,
    "pause_when_hidden": false,
    "autostart": false,
    "data_sources": ["sina", "tencent", "eastmoney"]
  },
  "shortcuts": {
    "toggle_visible": "CommandOrControl+Shift+S",
    "next_stock": "Alt+J",
    "prev_stock": "Alt+K"
  },
  "stocks": [
    {
      "id": "sh600519",
      "code": "600519",
      "market": "sh",
      "alias": "茅台",
      "visible": true
    },
    {
      "id": "sz000001",
      "code": "000001",
      "market": "sz",
      "alias": "平安银行",
      "visible": true
    }
  ]
}
```

### 2. 接口定义 (Rust <--> Frontend IPC)

#### Commands (Frontend invokes Rust)

| Command Name | Payload | Response | Description |
| :--- | :--- | :--- | :--- |
| `get_config` | `null` | `Config` object | 获取完整配置 |
| `update_config` | `Partial<Config>` | `Result<()>` | 更新配置并热生效 |
| `add_stock` | `{ code: string }` | `Result<Stock>` | 添加股票 (Rust端会自动识别市场) |
| `remove_stock` | `{ id: string }` | `Result<()>` | 移除股票 |
| `reorder_stocks` | `{ ids: string[] }` | `Result<()>` | 排序 |
| `set_window_visible` | `{ visible: boolean }` | `Result<()>` | 显隐控制 |
| `open_settings` | `null` | `Result<()>` | 打开/激活设置窗口 |
| `force_refresh` | `null` | `Result<()>` | 立即刷新行情数据 |

#### Events (Rust pushes to Frontend)

| Event Name | Payload | Description |
| :--- | :--- | :--- |
| `price-update` | `PriceUpdate[]` | 股票行情更新 (全量或增量) |
| `config-changed` | `Config` | 配置变更通知 (多窗口同步用) |
| `source-switched` | `{ from: string, to: string }` | 数据源切换通知 |
| `error` | `{ code: string, message: string }` | 错误通知 |

```typescript
// PriceUpdate 完整数据结构
interface PriceUpdate {
  id: string;          // e.g. "sh600519"
  code: string;        // "600519"
  market: string;      // "sh"
  name: string;        // "贵州茅台"
  price: number;       // 1750.00 (当前价)
  prev_close: number;  // 1740.00 (昨收)
  change: number;      // 10.00 (涨跌额)
  percent: number;     // 0.0575 (5.75% 涨跌幅)
  high: number;        // 1760.00 (今日最高)
  low: number;         // 1745.00 (今日最低)
  timestamp: number;   // Unix timestamp (ms)
  source: string;      // "sina" | "tencent" | "eastmoney"
}
```

### 3. 后端模块 (Rust)

*   `WindowManager`:
    *   `main_window`: 开启 `skip_taskbar` (Windows/Linux) 或 `set_excluded_from_windows_menu` (macOS)。
    *   `settings_window`: 普通窗口行为。
*   `Poller`: 单独 Tokio Task，`interval` 循环。
*   `Store`: 基于 `Mutex` 或 `RwLock` 保护 `Config` struct，读写需加锁。
*   `SourceManager`: 管理数据源优先级和故障转移逻辑。

## 错误处理策略 (Error Handling)

### 网络异常
- **超时**: 单次请求超时设为 5 秒
- **重试**: 失败后立即重试 1 次，若仍失败则计入失败计数
- **降级**: 连续 3 次失败触发数据源切换

### API 解析异常
- **格式错误**: 记录日志，跳过该股票的本次更新
- **字段缺失**: 使用默认值或上次缓存值

### 配置文件异常
- **文件不存在**: 创建默认配置
- **JSON 解析失败**: 备份损坏文件，使用默认配置
- **字段缺失**: 使用默认值补全

### 用户操作异常
- **添加无效股票代码**: 返回错误提示 "股票代码无效"
- **快捷键冲突**: 提示用户更换组合键


## Implementation Details (Platform Specific)

- **macOS**: 
  - 在 `Info.plist` 中添加 keys 以隐藏 Dock 图标（如果用户配置开启）。
  - 使用 `NSWindowLevel` 设置极高的置顶层级 (kCGFloatingWindowLevelKey)。
- **Windows**:
  - `SetWindowLong` 设置 `WS_EX_LAYERED | WS_EX_TRANSPARENT` 实现鼠标穿透。

## Risks / Trade-offs

- **[Risk] 免费 API 稳定性**
  - **Mitigation**:
    - 实现**适配器模式** (Adapter Pattern)，轻松扩展新源。
    - **自动降级**：若首选源连续 3 次失败，自动切换至次选源。

- **[Risk] 系统权限与杀毒软件**
  - 全局快捷键和开机自启可能被安全软件拦截。
  - **Mitigation**: 应用签名，提示用户添加白名单。

## Open Questions

- **深色/浅色模式跟随**：前端是否需要自动跟随系统主题切换配色？（暂定：是，Tauri 可获取系统主题）。
- **股票代码兼容性**：不同 API 对代码前缀（sh/sz/hk/us）处理不一致，后端需统一标准化为内部格式。
