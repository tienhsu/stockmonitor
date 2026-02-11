import React, { useState, useEffect } from 'react';
import ReactDOM from 'react-dom/client';
import { listen } from '@tauri-apps/api/event';
import { FloatingWindow } from './windows/FloatingWindow';
import { SettingsWindow } from './windows/SettingsWindow';
import './App.css';
import type { ErrorEvent } from './types';

/**
 * 全局 Toast 组件
 */
function GlobalToast() {
  const [toast, setToast] = useState<{ text: string; type: 'info' | 'error' } | null>(null);

  useEffect(() => {
    let unlistenErrorRequest: Promise<() => void>;
    let unlistenSourceRequest: Promise<() => void>;

    // 监听 error 事件
    unlistenErrorRequest = listen<ErrorEvent>('error', (event) => {
      setToast({ text: event.payload.message || '未知错误', type: 'error' });
      setTimeout(() => setToast(null), 5000);
    });

    // 监听 source-switched 事件
    unlistenSourceRequest = listen<{ from: string; to: string }>('source-switched', (event) => {
      const { from, to } = event.payload;
      const getSourceName = (name: string) =>
        name === 'sina' ? '新浪' : name === 'tencent' ? '腾讯' : name === 'eastmoney' ? '东财' : name;

      setToast({
        text: `数据源故障，已从 ${getSourceName(from)} 切换到 ${getSourceName(to)}`,
        type: 'info'
      });
      setTimeout(() => setToast(null), 5000);
    });

    return () => {
      unlistenErrorRequest.then((fn) => fn());
      unlistenSourceRequest.then((fn) => fn());
    };
  }, []);

  if (!toast) return null;

  return (
    <div className={`global-toast ${toast.type}`}>
      {toast.text}
    </div>
  );
}

/**
 * 根据 URL path 决定渲染哪个窗口
 */
function App() {
  const path = window.location.pathname;

  return (
    <>
      {path === '/settings' ? <SettingsWindow /> : <FloatingWindow />}
      <GlobalToast />
    </>
  );
}

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
