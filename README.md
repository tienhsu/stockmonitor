# 隐秘看盘 (Stealth Stock Monitor)

<div align="center">

一款轻量级跨平台悬浮股票监控工具，支持实时行情显示、鼠标穿透、快捷键控制等丰富的功能。

[![Version](https://img.shields.io/badge/version-0.1.0-blue)](https://github.com/wolf/stealth-stock-monitor)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey)](https://tauri.app/)

</div>

## ✨ 功能特性

### 📊 行行情监控
- 实时获取股票行情数据
- 支持多只股票同时监控
- 显示当前价、涨跌幅、今开、最高、最低等关键信息
- 可自定义刷新间隔

### 🖥️ 悬浮窗口
- 始终置顶悬浮显示，不占用屏幕空间
- 支持拖拽移动窗口位置
- 窗口透明度可调节（10% - 100%）
- **鼠标穿透模式**：开启后点击不会中断鼠标事件，完全"隐身"于背景之上
- 隐藏任务栏图标

### ⌨️ 全局快捷键
- 显示/隐藏窗口
- 切换上一只/下一只股票
- 快速打开设置窗口（即使鼠标穿透模式下也能使用）
- 快捷键在任何情况下都能触发

### 🎨 外观自定义
- 多种主题模式：跟随系统、深色、浅色
- 自定义上涨/下跌颜色
- 三种字体大小可选（小/中/大）
- 可调节显示行数（1-10 行）
- 自动轮播功能：当股票数量超过显示行数时自动轮播展示

### 🔧 系统集成
- 系统托盘图标
- 左键托盘图标切换窗口显隐
- 右键托盘菜单快速操作
- 支持开机自启动
- 隐藏窗口时自动暂停数据刷新节省资源

## 🌐 数据源

应用支持多个数据来源，自动切换确保数据稳定可靠。主要支持：
- A 股市场（上海/深圳）
- 自动识别股票代码所属市场

## 🏗️ 技术栈

| 技术层 | 技术选型 |
|--------|----------|
| 前端框架 | React 19 + TypeScript |
| 构建工具 | Vite 7 |
| 跨平台框架 | Tauri 2 |
| 后端语言 | Rust |
| 状态管理 | React Hooks + Tauri Store |
| 全局快捷键 | tauri-plugin-global-shortcut |
| 自启动 | tauri-plugin-autostart |

## 📦 安装与运行

### 前置要求

- **Rust** 1.70+
- **Node.js** 18+
- **pnpm** / **npm** / **yarn**

### 开发环境

```bash
# 安装前端依赖
cd stealth-stock-monitor
npm install

# 运行开发环境
npm run tauri dev
```

### 构建生产版本

```bash
# 构建应用
npm run tauri build
```

构建产物位于 `src-tauri/target/release/bundle/` 目录下。

## 📖 使用指南

### 添加股票

1. 右键悬浮窗口选择「设置」
2. 在「自选股」标签页输入股票代码
3. 支持格式：
   - A 股代码：`600519`（自动识别为上海市场）
   - 市场前缀：`sh600519`、`sz000001`

### 鼠标穿透模式

开启鼠标穿透模式后，窗口将完全"隐身"：
- 无法点击悬浮窗口
- 无法通过右键菜单打开设置
- 此时需要**通过快捷键**「打开设置」来关闭穿透模式

### 快捷键设置

1. 进入「设置」→「快捷键」标签页
2. 点击快捷键输入框
3. 按下要设置的组合键
4. 快捷键会自动保存并注册

### 自动轮播

当股票数量超过设置的显示行数时，可以启用自动轮播功能：
- 在「外观」标签页勾选「启用自动轮播」
- 设置轮播间隔时间（1-60 秒）
- 系统会自动循环展示所有股票

## 📂 项目结构

```
 stealth-stock-monitor/
 ├── src/                 # 前端代码
 │   ├── components/      # React 组件
 │   │   └── StockItem.tsx
 │   ├── hooks/           # 自定义 Hooks
 │   │   ├── useConfig.ts
 │   │   └── useStockData.ts
 │   ├── windows/         # 窗口组件
 │   │   ├── FloatingWindow.tsx   # 悬浮监控窗口
 │   │   └── SettingsWindow.tsx   # 设置窗口
 │   ├── types/           # TypeScript 类型定义
 │   │   └── index.ts
 │   └── App.tsx
 ├── src-tauri/          # Rust 后端代码
 │   ├── src/
 │   │   ├── lib.rs       # 主入口
 │   │   ├── models.rs    # 数据模型
 │   │   ├── config.rs    # 配置管理
 │   │   ├── sources.rs   # 数据源
 │   │   ├── poller.rs    # 数据轮询
 │   │   └── hotkey.rs    # 全局快捷键
 │   └── Cargo.toml
 └── package.json
```

## 🤝 贡献指南

欢迎提交 Issue 和 Pull Request！

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📄 开源协议

本项目基于 [MIT License](LICENSE) 开源。

## 📞 联系方式

**Email:** wolf3057@163.com
