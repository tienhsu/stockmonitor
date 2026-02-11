import type { PriceUpdate, Config } from '../types';
import './StockItem.css';

interface StockItemProps {
    data: PriceUpdate;
    config: Config;
    rowHeight: number;
}

/**
 * 单只股票显示组件
 * 布局: [名称/代码] [当前价格] [涨跌幅]
 */
export function StockItem({ data, config, rowHeight }: StockItemProps) {
    const { app } = config;

    // 根据涨跌确定颜色
    const getColor = () => {
        if (data.change > 0) return app.up_color;
        if (data.change < 0) return app.down_color;
        return app.neutral_color;
    };

    // 格式化涨跌幅
    const formatPercent = (percent: number) => {
        const pct = (percent * 100).toFixed(2);
        return percent >= 0 ? `+${pct}%` : `${pct}%`;
    };

    const color = getColor();

    // 显示名称：优先使用 alias，如果为空则使用 name
    const stock = config.stocks.find(s => s.id === data.id);
    const displayName = stock?.alias || data.name || data.code;

    return (
        <div className="stock-item" style={{ color, height: `${rowHeight}px` }} data-tauri-drag-region>
            <span className="stock-name" data-tauri-drag-region>{displayName}</span>
            <span className="stock-price" data-tauri-drag-region>{data.price.toFixed(2)}</span>
            <span className="stock-change" data-tauri-drag-region>
                {formatPercent(data.percent)}
            </span>
        </div>
    );
}
