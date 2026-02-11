## ADDED Requirements

### Requirement: 添加自选股 (Add Stock to Watchlist)
系统必须允许用户通过搜索股票代码或名称将股票添加到其个人自选列表中。

#### Scenario: 按代码添加
- **WHEN** 用户在设置输入框中输入有效的股票代码（例如“600519”）并确认
- **THEN** 股票“茅台”必须添加到自选列表的末尾

#### Scenario: 重复检查
- **WHEN** 用户尝试添加已在自选列表中的股票
- **THEN** 必须显示错误或通知“已在列表中”，且不创建重复项

### Requirement: 管理自选列表 (Manage Watchlist)
系统必须允许用户对自选列表中的股票进行排序和移除。

#### Scenario: 重新排序
- **WHEN** 用户在设置列表中将股票项拖动到新位置
- **THEN** 配置文件中的顺序必须更新以反映新序列

#### Scenario: 移除股票
- **WHEN** 用户点击股票项旁边的删除图标
- **THEN** 股票必须从自选列表中移除 **且** 如果当前正显示该股票，显示应立即更新

### Requirement: 多股显示布局 (Multi-Stock Display Configuration)
系统必须允许用户指定在悬浮窗口中同时显示多少只股票（如果布局支持）。

#### Scenario: 单行显示
- **WHEN** 配置设置为“行数：1”
- **THEN** 悬浮窗口必须仅显示当前选中的那一只股票

#### Scenario: 列表显示
- **WHEN** 配置设置为“行数：3”
- **THEN** 悬浮窗口必须垂直显示当前股票以及列表中的后两只股票
