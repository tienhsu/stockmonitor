import { useConfig } from '../hooks/useConfig';
import { useStockData } from '../hooks/useStockData';
import { StockItem } from '../components/StockItem';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useState, useEffect } from 'react';
import './FloatingWindow.css';

/**
 * 字体大小 → 行高 / 窗宽 映射
 */
const FONT_CONFIG: Record<string, { fontSize: number; rowHeight: number; width: number }> = {
    small: { fontSize: 12, rowHeight: 22, width: 260 },
    medium: { fontSize: 14, rowHeight: 26, width: 295 },
    large: { fontSize: 16, rowHeight: 30, width: 330 },
};

/**
 * 悬浮窗口主组件
 * - 显示股票行情列表
 * - 支持拖拽移动窗口（data-tauri-drag-region）
 * - 右键弹出原生系统菜单（不会被窗口裁切）
 * - 快捷键切换股票
 * - 窗口大小根据显示行数 + 字体大小自适应
 */
export function FloatingWindow() {
    const { config } = useConfig();
    const { getStock } = useStockData();
    const [scrollOffset, setScrollOffset] = useState(0);

    // 获取当前字体配置
    const fc = config
        ? FONT_CONFIG[config.app.font_size] || FONT_CONFIG.medium
        : FONT_CONFIG.medium;

    // ==================== 窗口大小自适应 ====================
    useEffect(() => {
        if (!config) return;

        const visibleStocks = config.stocks.filter(s => s.visible);
        const displayCount = Math.min(config.window.display_rows, visibleStocks.length) || 1;
        const f = FONT_CONFIG[config.app.font_size] || FONT_CONFIG.medium;

        const width = f.width;
        const height = displayCount * f.rowHeight + 4;

        invoke('resize_monitor_window', { width, height }).catch(console.error);
    }, [config?.window.display_rows, config?.app.font_size, config?.stocks]);

    // ==================== 鼠标穿透 ====================
    useEffect(() => {
        if (!config) return;
        invoke('set_ignore_cursor_events', { ignore: config.window.click_through })
            .catch(console.error);
    }, [config?.window.click_through]);

    // ==================== 快捷键监听 ====================
    useEffect(() => {
        if (!config) return;

        const visibleCount = config.stocks.filter(s => s.visible).length;
        const displayCount = Math.min(config.window.display_rows, visibleCount);
        const maxOffset = Math.max(0, visibleCount - displayCount);

        const unlistenNext = listen('hotkey-next-stock', () => {
            console.log('Received: hotkey-next-stock');
            setScrollOffset(prev => {
                const next = prev + 1;
                return next > maxOffset ? 0 : next;
            });
        });
        const unlistenPrev = listen('hotkey-prev-stock', () => {
            setScrollOffset(prev => {
                const prevIndex = prev - 1;
                return prevIndex < 0 ? maxOffset : prevIndex;
            });
        });

        return () => {
            unlistenNext.then(fn => fn());
            unlistenPrev.then(fn => fn());
        };
    }, [config?.stocks, config?.window.display_rows]);

    // ==================== 自动轮播 ====================
    useEffect(() => {
        if (!config || !config.window.enable_carousel) return;

        const visibleCount = config.stocks.filter(s => s.visible).length;
        const displayCount = Math.min(config.window.display_rows, visibleCount);

        // 只有当股票数量 > 显示行数时才需要轮播
        if (visibleCount <= displayCount) return;

        const interval = setInterval(() => {
            setScrollOffset(prev => {
                const maxOffset = visibleCount - displayCount;
                // 循环轮播：到底后回到顶部
                return prev >= maxOffset ? 0 : prev + 1;
            });
        }, config.window.carousel_interval_ms);

        return () => clearInterval(interval);
    }, [config?.window.enable_carousel, config?.window.carousel_interval_ms, config?.stocks, config?.window.display_rows]);

    // 股票列表变化时校正偏移量
    useEffect(() => {
        if (!config) return;
        const visibleCount = config.stocks.filter(s => s.visible).length;
        setScrollOffset(prev => {
            const maxOffset = Math.max(0, visibleCount - config.window.display_rows);
            return Math.min(prev, maxOffset);
        });
    }, [config?.stocks, config?.window.display_rows]);

    if (!config) return null;

    // ==================== 计算要显示的股票 ====================
    const visibleStocks = config.stocks.filter(s => s.visible);
    const displayCount = Math.min(config.window.display_rows, visibleStocks.length) || visibleStocks.length;
    const startIndex = Math.min(scrollOffset, Math.max(0, visibleStocks.length - displayCount));
    const displayStocks = visibleStocks.slice(startIndex, startIndex + displayCount);

    // ==================== 原生右键菜单 ====================
    const handleContextMenu = (e: React.MouseEvent) => {
        e.preventDefault();
        console.log('[FloatingWindow] 右键菜单被触发');
        invoke('show_context_menu')
            .then(() => console.log('[FloatingWindow] 菜单显示成功'))
            .catch(err => console.error('[FloatingWindow] 菜单显示失败:', err));
    };

    const bgOpacity = config.window.opacity;

    return (
        <div
            className="floating-window"
            data-tauri-drag-region
            onContextMenu={handleContextMenu}
            style={{
                fontSize: `${fc.fontSize}px`,
                background: `rgba(30, 30, 30, ${bgOpacity})`,
            }}
        >
            {displayStocks.map((stock) => {
                const data = getStock(stock.id);
                if (!data) {
                    return (
                        <div
                            key={stock.id}
                            className="stock-item stock-loading"
                            style={{ height: `${fc.rowHeight}px` }}
                            data-tauri-drag-region
                        >
                            <span className="stock-name" data-tauri-drag-region>{stock.alias || stock.code}</span>
                            <span className="stock-price">--</span>
                            <span className="stock-change">--</span>
                        </div>
                    );
                }
                return <StockItem key={stock.id} data={data} config={config} rowHeight={fc.rowHeight} />;
            })}
        </div>
    );
}
