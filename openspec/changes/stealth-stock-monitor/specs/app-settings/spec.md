## ADDED Requirements

### Requirement: 持久化配置 (Persistent Configuration)
系统必须将用户设置（窗口位置、股票列表、快捷键、主题）存储在本地配置文件（JSON/TOML）中，并在应用重启后持久保留。

#### Scenario: 变更即保存
- **WHEN** 用户在设置面板中更新设置（例如更改窗口透明度）
- **THEN** 新值必须立即写入配置文件

#### Scenario: 启动加载
- **WHEN** 应用程序启动时
- **THEN** 它必须从配置文件读取并应用所有设置

### Requirement: 主题自定义 (Theme Customization)
系统必须允许用户自定义应用程序的视觉外观，包括涨跌颜色方案。

#### Scenario: 自定义涨/跌颜色
- **WHEN** 用户设置“上涨颜色”为红 **且** “下跌颜色”为绿
- **THEN** 具有正变化的股票必须显示为红色
- **AND** 具有负变化的股票必须显示为绿色

#### Scenario: 字体大小调整
- **WHEN** 用户更改字体大小设置（小/中/大）
- **THEN** 悬浮窗口中的文本必须立即更新为所选大小

### Requirement: 透明度控制 (Transparency Control)
系统必须提供对悬浮窗口透明度的精细控制。

#### Scenario: 不透明度滑块
- **WHEN** 用户将不透明度滑块调整为 50%
- **THEN** 悬浮窗口必须立即变为半透明

### Requirement: 开机自启动 (Auto Start)
系统必须提供开机自启动选项，使应用程序可以在用户登录后自动启动。

#### Scenario: 启用自启动
- **WHEN** 用户在设置中开启"开机自启动"选项
- **THEN** 应用程序必须在下次操作系统启动并登录后自动运行

#### Scenario: 关闭自启动
- **WHEN** 用户在设置中关闭"开机自启动"选项
- **THEN** 应用程序在下次操作系统启动后不得自动运行

### Requirement: 数据源配置 (Data Source Configuration)
系统必须允许用户选择和排列数据提供商的优先级顺序。

#### Scenario: 更改数据源优先级
- **WHEN** 用户将数据源优先级从"新浪 > 腾讯"调整为"腾讯 > 新浪"
- **THEN** 系统必须优先使用腾讯 API 获取数据，新浪作为备用源

### Requirement: 刷新频率配置 (Refresh Interval Configuration)
系统必须允许用户自定义行情数据的刷新间隔（范围：1 秒 ~ 60 秒）。

#### Scenario: 调整刷新频率
- **WHEN** 用户在设置中将刷新间隔从 3 秒更改为 10 秒
- **THEN** 后端数据轮询频率必须立即更新为 10 秒

