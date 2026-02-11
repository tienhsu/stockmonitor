import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { Config } from '../types';

/**
 * 管理应用配置状态的 Hook
 * - 启动时从后端加载配置
 * - 监听 config-changed 事件自动同步
 */
export function useConfig() {
    const [config, setConfig] = useState<Config | null>(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        console.log('[useConfig] 开始加载配置...');

        // 初始加载
        invoke<Config>('get_config')
            .then((cfg) => {
                console.log('[useConfig] 配置加载成功:', cfg);
                setConfig(cfg);
                setLoading(false);
            })
            .catch((err) => {
                console.error('[useConfig] 配置加载失败:', err);
                setError(String(err));
                setLoading(false);
            });

        // 监听配置变更事件
        const unlisten = listen<Config>('config-changed', (event) => {
            console.log('[useConfig] 配置已更新:', event.payload);
            setConfig(event.payload);
        });

        return () => {
            unlisten.then((fn) => fn());
        };
    }, []);

    /** 更新配置 */
    const updateConfig = async (newConfig: Config) => {
        try {
            console.log('[useConfig] 更新配置:', newConfig);
            await invoke('update_config', { config: newConfig });
            setConfig(newConfig);
        } catch (err) {
            console.error('[useConfig] 配置更新失败:', err);
            setError(String(err));
        }
    };

    return { config, loading, error, updateConfig };
}
