import { useState, useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import type { PriceUpdate } from '../types';

/**
 * 订阅股票行情数据更新的 Hook
 * - 监听后端推送的 price-update 事件
 * - 维护一个 Map<id, PriceUpdate> 作为最新数据快照
 */
export function useStockData() {
    const [stockData, setStockData] = useState<Map<string, PriceUpdate>>(new Map());

    useEffect(() => {
        const unlisten = listen<PriceUpdate[]>('price-update', (event) => {
            setStockData((prev) => {
                const next = new Map(prev);
                for (const update of event.payload) {
                    next.set(update.id, update);
                }
                return next;
            });
        });

        return () => {
            unlisten.then((fn) => fn());
        };
    }, []);

    /** 根据 ID 获取单只股票数据 */
    const getStock = (id: string): PriceUpdate | undefined => {
        return stockData.get(id);
    };

    /** 获取所有股票数据列表 */
    const getAllStocks = (): PriceUpdate[] => {
        return Array.from(stockData.values());
    };

    return { stockData, getStock, getAllStocks };
}
