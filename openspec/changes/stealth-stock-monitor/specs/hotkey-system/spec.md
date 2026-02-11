## ADDED Requirements

### Requirement: 全局显示/隐藏切换 (Global Visible Toggle)
系统必须注册一个全局快捷键（默认：`Ctrl+Shift+S` 或 `Cmd+Shift+S`），用于切换悬浮窗口的可见性，无论当前焦点位于哪个应用程序。

#### Scenario: 显示窗口
- **WHEN** 悬浮窗口处于隐藏状态 **且** 用户按下切换快捷键
- **THEN** 窗口必须立即在其上次位置变为可见

#### Scenario: 隐藏窗口
- **WHEN** 悬浮窗口处于可见状态 **且** 用户按下切换快捷键
- **THEN** 窗口必须立即隐藏

### Requirement: 快速股票切换 (Quick Stock Navigation)
系统必须允许用户使用全局快捷键在自选股列表中循环切换显示，而无需将焦点切换到此应用。

#### Scenario: 下一只股票
- **WHEN** 用户按下“下一只股票”快捷键（默认：`Alt+J`）
- **THEN** 显示必须更新为自选列表中的下一只股票

#### Scenario: 上一只股票
- **WHEN** 用户按下“上一只股票”快捷键（默认：`Alt+K`）
- **THEN** 显示必须更新为自选列表中的上一只股票

#### Scenario: 列表循环
- **WHEN** 当前显示的是自选列表中的最后一只股票 **且** 用户按下"下一只股票"快捷键
- **THEN** 显示必须循环回到列表中的第一只股票

### Requirement: 自定义快捷键配置 (Custom Shortcut Configuration)
系统必须允许用户修改所有全局功能的快捷键组合。

#### Scenario: 更改切换快捷键
- **WHEN** 用户在设置中将“切换可见性”快捷键更新为新组合（例如 `Ctrl+Space`）
- **THEN** 旧的快捷键必须失效 **且** 新的快捷键必须触发切换动作
