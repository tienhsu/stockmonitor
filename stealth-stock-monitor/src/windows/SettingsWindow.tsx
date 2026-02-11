import { useState } from 'react';
import { useConfig } from '../hooks/useConfig';
import { invoke } from '@tauri-apps/api/core';
import type { Config } from '../types';
import './SettingsWindow.css';

/**
 * 设置窗口组件
 * 包含：股票管理、外观配置、快捷键设置、数据源配置
 */
export function SettingsWindow() {
    // ==================== Hooks 必须在所有条件判断之前调用 ====================
    const { config, updateConfig, loading, error } = useConfig();
    const [newStockCode, setNewStockCode] = useState('');
    const [activeTab, setActiveTab] = useState<'stocks' | 'appearance' | 'shortcuts' | 'data' | 'about'>('stocks');
    const [message, setMessage] = useState<{ text: string; type: 'success' | 'error' } | null>(null);
    const [recordingKey, setRecordingKey] = useState<string | null>(null);

    // ==================== 辅助函数 ====================
    const showMessage = (text: string, type: 'success' | 'error' = 'success') => {
        setMessage({ text, type });
        setTimeout(() => setMessage(null), 3000);
    };

    // ==================== 股票管理 ====================
    const handleAddStock = async () => {
        const code = newStockCode.trim();
        if (!code) return;
        try {
            await invoke('add_stock', { code });
            setNewStockCode('');
            showMessage(`已添加股票 ${code}`);
        } catch (err) {
            showMessage(String(err), 'error');
        }
    };

    const handleRemoveStock = async (id: string) => {
        try {
            await invoke('remove_stock', { id });
            showMessage('已移除');
        } catch (err) {
            showMessage(String(err), 'error');
        }
    };

    // ==================== 配置更新 ====================
    const updateField = <K extends keyof Config>(
        section: K,
        field: string,
        value: unknown
    ) => {
        if (!config) return;
        const sectionValue = config[section];
        const newConfig = {
            ...config,
            [section]: typeof sectionValue === 'object' && sectionValue !== null
                ? { ...sectionValue, [field]: value }
                : value,
        };
        updateConfig(newConfig as Config);
    };

    // ==================== 快捷键录制 ====================
    const handleKeyRecord = (shortcutName: string) => {
        setRecordingKey(shortcutName);
    };

    const handleKeyDown = (e: React.KeyboardEvent, shortcutName: string) => {
        if (recordingKey !== shortcutName) return;
        e.preventDefault();

        const parts: string[] = [];
        if (e.ctrlKey || e.metaKey) parts.push('CommandOrControl');
        if (e.altKey) parts.push('Alt');
        if (e.shiftKey) parts.push('Shift');

        // 使用 code 而不是 key，避免 MacOS Option 键导致字符变化（如 Option+J -> ∆）
        let key = e.key;
        if (e.code.startsWith('Key')) {
            key = e.code.slice(3);
        } else if (e.code.startsWith('Digit')) {
            key = e.code.slice(5);
        } else if (e.code.startsWith('Arrow')) {
            key = e.code; // ArrowUp, ArrowDown etc is fine
        } else {
            // Fallback for other keys, uppercase if single char
            key = key.length === 1 ? key.toUpperCase() : key;
        }

        // 过滤掉单独按下的修饰键
        if (['Control', 'Alt', 'Shift', 'Meta', 'Command'].includes(key)) {
            return;
        }

        parts.push(key);
        const combo = parts.join('+');
        updateField('shortcuts', shortcutName, combo);
        setRecordingKey(null);
    };

    // ==================== 条件渲染 ====================
    if (loading) {
        return <div className="settings-loading">加载配置中...</div>;
    }

    if (error) {
        return <div className="settings-loading" style={{ color: '#ff5555' }}>配置加载失败: {error}</div>;
    }

    if (!config) {
        return <div className="settings-loading">配置为空，请检查后端</div>;
    }

    return (
        <div className="settings-window">
            <h1 className="settings-title">设置</h1>

            {/* 消息提示 */}
            {message && (
                <div className={`toast ${message.type}`}>{message.text}</div>
            )}

            {/* 标签页导航 */}
            <div className="tab-bar">
                <button
                    className={`tab ${activeTab === 'stocks' ? 'active' : ''}`}
                    onClick={() => setActiveTab('stocks')}
                >自选股</button>
                <button
                    className={`tab ${activeTab === 'appearance' ? 'active' : ''}`}
                    onClick={() => setActiveTab('appearance')}
                >外观</button>
                <button
                    className={`tab ${activeTab === 'shortcuts' ? 'active' : ''}`}
                    onClick={() => setActiveTab('shortcuts')}
                >快捷键</button>
                <button
                    className={`tab ${activeTab === 'data' ? 'active' : ''}`}
                    onClick={() => setActiveTab('data')}
                >数据源</button>
                <button
                    className={`tab ${activeTab === 'about' ? 'active' : ''}`}
                    onClick={() => setActiveTab('about')}
                >关于</button>
            </div>

            {/* 股票管理 */}
            {activeTab === 'stocks' && (
                <div className="tab-content">
                    <div className="add-stock-row">
                        <input
                            type="text"
                            placeholder="输入股票代码，如 600519"
                            value={newStockCode}
                            onChange={(e) => setNewStockCode(e.target.value)}
                            onKeyDown={(e) => e.key === 'Enter' && handleAddStock()}
                            className="input"
                        />
                        <button onClick={handleAddStock} className="btn btn-primary">添加</button>
                    </div>
                    <div className="stock-list">
                        {config.stocks.map((stock) => (
                            <div key={stock.id} className="stock-list-item">
                                <span className="stock-id">{stock.id}</span>
                                <span className="stock-alias">{stock.alias || stock.code}</span>
                                <button
                                    className="btn btn-danger btn-sm"
                                    onClick={() => handleRemoveStock(stock.id)}
                                >删除</button>
                            </div>
                        ))}
                        {config.stocks.length === 0 && (
                            <div className="empty-hint">暂无自选股，请添加</div>
                        )}
                    </div>
                </div>
            )}

            {/* 外观设置 */}
            {activeTab === 'appearance' && (
                <div className="tab-content">
                    <div className="form-group">
                        <label>窗口透明度</label>
                        <input
                            type="range"
                            min="0.1"
                            max="1"
                            step="0.05"
                            value={config.window.opacity}
                            onChange={(e) => updateField('window', 'opacity', parseFloat(e.target.value))}
                        />
                        <span className="value">{Math.round(config.window.opacity * 100)}%</span>
                    </div>
                    <div className="form-group">
                        <label className="checkbox-label">
                            <input
                                type="checkbox"
                                checked={config.window.click_through}
                                onChange={(e) => updateField('window', 'click_through', e.target.checked)}
                            />
                            鼠标穿透模式（开启后无法点击窗口，需通过设置关闭）
                        </label>
                    </div>
                    <div className="form-group">
                        <label>字体大小</label>
                        <select
                            value={config.app.font_size}
                            onChange={(e) => updateField('app', 'font_size', e.target.value)}
                            className="select"
                        >
                            <option value="small">小</option>
                            <option value="medium">中</option>
                            <option value="large">大</option>
                        </select>
                    </div>
                    <div className="form-group">
                        <label>上涨颜色</label>
                        <input
                            type="color"
                            value={config.app.up_color}
                            onChange={(e) => updateField('app', 'up_color', e.target.value)}
                        />
                    </div>
                    <div className="form-group">
                        <label>下跌颜色</label>
                        <input
                            type="color"
                            value={config.app.down_color}
                            onChange={(e) => updateField('app', 'down_color', e.target.value)}
                        />
                    </div>
                    <div className="form-group">
                        <label>显示行数</label>
                        <input
                            type="number"
                            min="1"
                            max="10"
                            value={config.window.display_rows}
                            onChange={(e) => updateField('window', 'display_rows', parseInt(e.target.value))}
                            className="input input-sm"
                        />
                    </div>
                    <div className="form-group">
                        <label className="checkbox-label">
                            <input
                                type="checkbox"
                                checked={config.window.enable_carousel}
                                onChange={(e) => updateField('window', 'enable_carousel', e.target.checked)}
                            />
                            启用自动轮播（股票数量 &gt; 显示行数时生效）
                        </label>
                    </div>
                    {config.window.enable_carousel && (
                        <div className="form-group">
                            <label>轮播间隔（秒）</label>
                            <input
                                type="number"
                                min="1"
                                max="60"
                                value={config.window.carousel_interval_ms / 1000}
                                onChange={(e) => updateField('window', 'carousel_interval_ms', parseInt(e.target.value) * 1000)}
                                className="input input-sm"
                            />
                            <span className="hint">每隔几秒自动切换到下一只股票</span>
                        </div>
                    )}
                    <div className="form-group">
                        <label>主题模式</label>
                        <select
                            value={config.app.theme}
                            onChange={(e) => updateField('app', 'theme', e.target.value)}
                            className="select"
                        >
                            <option value="auto">跟随系统</option>
                            <option value="dark">深色</option>
                            <option value="light">浅色</option>
                        </select>
                    </div>
                </div>
            )}

            {/* 快捷键 */}
            {activeTab === 'shortcuts' && (
                <div className="tab-content">
                    {[
                        { key: 'toggle_visible', label: '显示/隐藏窗口' },
                        { key: 'next_stock', label: '下一只股票' },
                        { key: 'prev_stock', label: '上一只股票' },
                    ].map(({ key, label }) => (
                        <div key={key} className="form-group">
                            <label>{label}</label>
                            <input
                                type="text"
                                readOnly
                                className={`input shortcut-input ${recordingKey === key ? 'recording' : ''}`}
                                value={
                                    recordingKey === key
                                        ? '按下组合键...'
                                        : (config.shortcuts as unknown as Record<string, string>)[key]
                                            .replace('CommandOrControl', 'Cmd/Ctrl')
                                            .replace('Control', 'Ctrl')
                                            .replace('Command', 'Cmd')
                                }
                                onClick={() => handleKeyRecord(key)}
                                onKeyDown={(e) => handleKeyDown(e, key)}
                                onBlur={() => setRecordingKey(null)}
                            />
                        </div>
                    ))}
                </div>
            )}

            {/* 数据源 */}
            {activeTab === 'data' && (
                <div className="tab-content">
                    <div className="form-group">
                        <label>刷新间隔</label>
                        <div className="range-row">
                            <input
                                type="range"
                                min="1000"
                                max="60000"
                                step="1000"
                                value={config.app.refresh_interval_ms}
                                onChange={(e) => updateField('app', 'refresh_interval_ms', parseInt(e.target.value))}
                            />
                            <span className="value">{(config.app.refresh_interval_ms / 1000).toFixed(0)}秒</span>
                        </div>
                    </div>

                    <div className="form-group">
                        <label className="checkbox-label">
                            <input
                                type="checkbox"
                                checked={config.app.pause_when_hidden}
                                onChange={(e) => updateField('app', 'pause_when_hidden', e.target.checked)}
                            />
                            隐藏窗口时暂停数据刷新
                        </label>
                    </div>
                    <div className="form-group">
                        <label className="checkbox-label">
                            <input
                                type="checkbox"
                                checked={config.app.autostart}
                                onChange={(e) => updateField('app', 'autostart', e.target.checked)}
                            />
                            开机自启动
                        </label>
                    </div>
                </div>
            )}

            {/* 关于 */}
            {activeTab === 'about' && (
                <div className="tab-content">
                    <div className="about-section" style={{ padding: '20px', textAlign: 'center', color: '#e0e0e0' }}>
                        <h2 style={{ fontSize: '18px', marginBottom: '16px', color: '#fff' }}>关于我们</h2>
                        <p style={{ lineHeight: '1.6', marginBottom: '24px', color: '#ccc' }}>
                            我们是一支专业的软件开发团队，致力于提供高质量的技术解决方案。
                            <br />
                            承接各类软件开发工作，包括但不限于：
                        </p>
                        <div style={{
                            display: 'grid',
                            gridTemplateColumns: 'repeat(2, 1fr)',
                            gap: '12px',
                            textAlign: 'left',
                            maxWidth: '400px',
                            margin: '0 auto 30px auto'
                        }}>
                            <div className="service-item">🚀 桌面应用开发</div>
                            <div className="service-item">🌐 Web 应用开发</div>
                            <div className="service-item">📱 移动端 App 开发</div>
                            <div className="service-item">🤖 自动化脚本定制</div>
                        </div>

                        <div style={{
                            background: 'rgba(74, 158, 255, 0.1)',
                            padding: '16px',
                            borderRadius: '8px',
                            display: 'inline-block'
                        }}>
                            <p style={{ marginBottom: '8px', fontSize: '14px', color: '#888' }}>业务合作请联系</p>
                            <a href="mailto:contact@example.com" style={{
                                color: '#4a9eff',
                                fontSize: '16px',
                                textDecoration: 'none',
                                fontWeight: '600'
                            }}>
                                contact@example.com
                            </a>
                        </div>
                        <p style={{ marginTop: '40px', fontSize: '12px', color: '#666' }}>
                            v0.1.0
                        </p>
                    </div>
                </div>
            )}
        </div>
    );
}
