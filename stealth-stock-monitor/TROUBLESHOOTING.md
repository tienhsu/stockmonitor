# 故障排查指南

## 设置窗口空白问题

如果设置窗口显示空白，请按以下步骤排查：

### 1. 检查浏览器控制台

**macOS 开发模式下**:
- 右键点击设置窗口 → 检查元素（Inspect Element）
- 或按 `Cmd + Option + I` 打开开发者工具
- 查看 Console 标签页

**查找以下日志**:
```
[useConfig] 开始加载配置...
[useConfig] 配置加载成功: {...}
```

**如果看到错误**:
```
[useConfig] 配置加载失败: ...
```
记录错误信息，可能的原因：
- 后端命令未注册
- 配置文件损坏
- 权限问题

### 2. 检查配置文件

**配置文件位置**:
- macOS: `~/Library/Application Support/com.wolf.stealth-stock-monitor/config.json`
- Windows: `%APPDATA%/com.wolf.stealth-stock-monitor/config.json`

**验证配置文件**:
```bash
# macOS
cat ~/Library/Application\ Support/com.wolf.stealth-stock-monitor/config.json

# 应该看到类似这样的 JSON 内容
{
  "version": 1,
  "window": {...},
  "app": {...},
  "shortcuts": {...},
  "stocks": [...]
}
```

**如果配置文件损坏**:
```bash
# 备份旧配置
mv ~/Library/Application\ Support/com.wolf.stealth-stock-monitor/config.json \
   ~/Library/Application\ Support/com.wolf.stealth-stock-monitor/config.json.bak

# 重启应用，会自动生成新的默认配置
```

### 3. 检查后端日志

**开发模式下查看终端输出**:
```bash
npm run tauri dev
```

查找错误信息，特别是：
- Rust panic 错误
- 配置加载失败
- 命令调用失败

### 4. 验证 Tauri 命令

**在浏览器控制台手动测试**:
```javascript
// 测试获取配置
await window.__TAURI__.core.invoke('get_config')
  .then(cfg => console.log('配置:', cfg))
  .catch(err => console.error('错误:', err));
```

### 5. 常见问题

#### 问题：设置窗口一直显示"加载配置中..."
**原因**: 配置加载超时或卡住
**解决**:
1. 检查网络（如果有远程配置）
2. 重启应用
3. 删除配置文件重新生成

#### 问题：显示"配置加载失败: ..."
**原因**: 后端返回错误
**解决**:
1. 查看具体错误信息
2. 检查配置文件格式
3. 查看后端日志

#### 问题：显示"配置为空，请检查后端"
**原因**: `get_config` 返回 null
**解决**:
1. 检查后端 `get_config` 命令实现
2. 验证 ConfigStore 初始化
3. 查看 Rust 编译错误

### 6. 重置应用

**完全重置（清除所有数据）**:
```bash
# macOS
rm -rf ~/Library/Application\ Support/com.wolf.stealth-stock-monitor/

# Windows
rmdir /s %APPDATA%\com.wolf.stealth-stock-monitor\
```

然后重启应用，会使用默认配置。

### 7. 开发者调试

**启用详细日志**:
```bash
# 设置 Rust 日志级别
RUST_LOG=debug npm run tauri dev
```

**检查编译状态**:
```bash
cd src-tauri
cargo check
```

**手动测试配置加载**:
```bash
cd src-tauri
cargo run --bin test-config  # 如果有测试二进制
```

## 其他常见问题

### 窗口不显示
- 检查是否被隐藏（使用快捷键 `Cmd/Ctrl + Shift + S`）
- 检查系统托盘图标
- 查看是否在屏幕外（重置配置）

### 快捷键不工作
- 检查是否与其他应用冲突
- 在设置中重新录制快捷键
- 查看后端日志确认快捷键已注册

### 数据不更新
- 检查网络连接
- 查看数据源切换提示
- 手动刷新（右键菜单 → 刷新）

## 获取帮助

如果以上方法都无法解决问题，请提供：
1. 浏览器控制台完整日志
2. 终端输出（开发模式）
3. 配置文件内容
4. 操作系统版本
5. 复现步骤
