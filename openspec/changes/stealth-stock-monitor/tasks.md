## 1. 项目初始化与环境搭建

- [x] 1.1 使用 `create-tauri-app` 创建 Tauri 2.x + React + TypeScript 项目
- [x] 1.2 配置 `tauri.conf.json`：设置应用名称、窗口属性（无边框、置顶）
- [x] 1.3 安装必要的 Tauri 插件：`tauri-plugin-global-shortcut`、`tauri-plugin-store`、`tauri-plugin-autostart`
- [x] 1.4 配置 Rust 依赖：添加 `tokio`、`serde`、`reqwest`、`anyhow` 到 `Cargo.toml`
- [x] 1.5 创建项目目录结构（按 design.md 中的 Project Structure）
- [x] 1.6 配置 `tauri-plugin-autostart`：实现开机自启动支持

## 2. 数据模型与配置管理 (Rust)

- [x] 2.1 定义 `models.rs`：创建 `Stock`、`Config`、`PriceUpdate` 等核心数据结构
- [x] 2.2 实现 `config.rs`：配置文件读取、写入、默认值生成
- [x] 2.3 实现配置热加载机制（使用 `RwLock` 保护共享状态）
- [x] 2.4 添加配置文件路径解析（跨平台支持：`~/.config/stealth-stock/` 和 `%APPDATA%`）

## 3. 数据源适配器 (Rust)

- [x] 3.1 创建 `sources/mod.rs`：定义 `DataSource` trait（统一接口）
- [x] 3.2 实现 `sources/sina.rs`：新浪财经 API 适配器（HTTP 请求 + CSV 解析）
- [x] 3.3 实现 `sources/tencent.rs`：腾讯证券 API 适配器
- [x] 3.4 实现 `sources/eastmoney.rs`：东方财富 API 适配器
- [x] 3.5 实现股票代码自动识别逻辑（sh/sz 前缀判断）
- [x] 3.6 实现 `SourceManager`：管理数据源优先级和故障转移

## 4. 数据轮询器 (Rust)

- [x] 4.1 创建 `poller.rs`：实现基于 Tokio 的异步轮询任务
- [x] 4.2 实现可配置的轮询间隔（从 `config.refresh_interval_ms` 读取）
- [x] 4.3 实现批量股票数据请求（一次请求多只股票）
- [x] 4.4 实现数据解析和标准化（转换为 `PriceUpdate` 结构）
- [x] 4.5 实现错误处理：超时重试、失败计数、数据源切换
- [x] 4.6 实现 `pause_when_hidden` 逻辑（窗口隐藏时暂停轮询）
- [x] 4.7 通过 Tauri Event 推送 `price-update` 事件到前端

## 5. 窗口管理 (Rust)

- [x] 5.1 创建 `window.rs`：实现 `WindowManager` 模块
- [x] 5.2 配置主窗口（悬浮窗）：无边框、置顶、隐藏任务栏图标
- [x] 5.3 配置设置窗口：标准窗口样式
- [x] 5.4 实现窗口显示/隐藏切换逻辑
- [x] 5.5 实现窗口位置和大小的持久化（保存到配置文件）
- [x] 5.6 实现平台特定逻辑：macOS `LSUIElement`、Windows `WS_EX_TOOLWINDOW`

## 6. 全局快捷键系统 (Rust)

- [x] 6.1 创建 `hotkey.rs`：封装 `tauri-plugin-global-shortcut`
- [x] 6.2 实现快捷键注册逻辑（从配置读取组合键）
- [x] 6.3 实现 `toggle_visible` 快捷键处理（显示/隐藏窗口）
- [x] 6.4 实现 `next_stock` 和 `prev_stock` 快捷键处理
- [x] 6.5 实现快捷键动态更新（配置变更时重新注册）
- [x] 6.6 添加快捷键冲突检测和错误提示

## 7. 系统托盘 (Rust)

- [x] 7.1 配置系统托盘图标（使用 Tauri 内置托盘功能）
- [x] 7.2 实现托盘左键点击：切换窗口显示/隐藏
- [x] 7.3 实现托盘右键菜单：`Settings`、`Quit`
- [ ] 7.4 实现数据源切换时托盘图标变色提示（可选）

## 8. Tauri Commands (Rust)

- [x] 8.1 实现 `get_config` 命令：返回完整配置
- [x] 8.2 实现 `update_config` 命令：更新配置并保存
- [x] 8.3 实现 `add_stock` 命令：添加股票到自选列表
- [x] 8.4 实现 `remove_stock` 命令：移除股票
- [x] 8.5 实现 `reorder_stocks` 命令：重新排序自选列表
- [x] 8.6 实现 `set_window_visible` 命令：控制窗口显隐
- [x] 8.7 实现 `open_settings` 命令：打开/激活设置窗口
- [x] 8.8 实现 `force_refresh` 命令：立即刷新行情数据

## 9. 前端类型定义 (TypeScript)

- [x] 9.1 创建 `src/types/index.ts`：定义 `Config`、`Stock`、`PriceUpdate` 等接口
- [x] 9.2 定义 Tauri Command 的类型签名
- [x] 9.3 定义 Tauri Event 的类型签名

## 10. 前端状态管理 (React)

- [x] 10.1 创建 `src/hooks/useConfig.ts`：管理配置状态
- [x] 10.2 创建 `src/hooks/useStockData.ts`：订阅 `price-update` 事件
- [x] 10.3 实现 React Context 用于全局状态共享
- [x] 10.4 实现配置变更时的自动同步（监听 `config-changed` 事件）

## 11. 悬浮窗口 UI (React)

- [x] 11.1 创建 `src/windows/FloatingWindow.tsx`：主悬浮窗组件
- [x] 11.2 创建 `src/components/StockItem.tsx`：单个股票显示组件
- [x] 11.3 实现股票信息布局：名称/代码、价格、涨跌幅
- [x] 11.4 实现涨跌颜色逻辑（根据配置的 `up_color`/`down_color`）
- [x] 11.5 实现多行显示模式（根据 `display_rows` 配置）
- [x] 11.6 实现窗口拖拽功能（使用 Tauri 的 `data-tauri-drag-region`）
- [x] 11.7 实现右键菜单：Refresh、Settings、Hide、Quit
- [ ] 11.8 实现鼠标穿透模式（根据 `click_through` 配置）

## 12. 设置窗口 UI (React)

- [x] 12.1 创建 `src/windows/SettingsWindow.tsx`：设置窗口组件
- [x] 12.2 实现股票管理面板：添加、删除、排序
- [x] 12.3 实现股票搜索输入框（支持代码输入）
- [x] 12.4 实现外观配置：透明度滑块、字体大小选择、颜色选择器
- [x] 12.5 实现快捷键录制功能（Input Box + 键盘事件监听）
- [x] 12.6 实现数据源选择和优先级配置
- [x] 12.7 实现刷新间隔配置（滑块或输入框）
- [x] 12.8 实现配置保存按钮（调用 `update_config` 命令）
- [x] 12.9 实现开机自启动开关（调用 `tauri-plugin-autostart` API）

## 13. 样式与主题 (CSS)

- [x] 13.1 创建悬浮窗样式：极简、紧凑、无边框
- [x] 13.2 创建设置窗口样式：清晰的表单布局
- [x] 13.3 实现主题切换逻辑（auto/dark/light）
- [x] 13.4 实现响应式字体大小（small/medium/large）
- [x] 13.5 实现动态颜色应用（从配置读取）

## 14. 错误处理与用户反馈

- [x] 14.1 实现前端错误边界（React Error Boundary）
- [x] 14.2 实现 Toast 通知组件（用于显示错误和提示）
- [x] 14.3 监听 `error` 事件并显示用户友好的错误信息
- [x] 14.4 监听 `source-switched` 事件并提示数据源切换
- [x] 14.5 实现添加无效股票代码时的错误提示

## 15. 平台特定适配

- [x] 15.1 配置 macOS `Info.plist`：添加 `LSUIElement` 支持
- [x] 15.2 配置 Windows 窗口样式：`WS_EX_TOOLWINDOW` 隐藏任务栏
- [x] 15.3 测试 macOS 全局快捷键权限请求流程
- [x] 15.4 测试 Windows 杀毒软件兼容性

## 16. 测试与优化

- [x] 16.1 测试数据源故障转移逻辑（模拟 API 失败）
- [x] 16.2 测试配置文件损坏时的恢复机制
- [x] 16.3 测试窗口位置记忆功能（跨重启）
- [x] 16.4 测试多显示器场景下的窗口行为
- [x] 16.5 性能测试：内存占用、CPU 使用率
- [x] 16.6 测试快捷键在不同应用下的响应

## 17. 打包与发布

- [x] 17.1 配置应用图标（macOS `.icns` 和 Windows `.ico`）
- [x] 17.2 配置应用签名（如有条件）
- [x] 17.3 生成 Windows 安装包（`.msi` 和 `.exe`）
- [x] 17.4 生成 macOS 安装包（`.dmg` 和 `.app`）
- [x] 17.5 编写用户文档：使用说明、快捷键表、配置指南
- [x] 17.6 测试安装包在干净系统上的安装和运行
