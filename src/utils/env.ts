/**
 * Detect if the app is running in a Tauri environment
 */
export const isTauri = () => {
    return typeof window !== 'undefined' &&
        (!!(window as any).__TAURI_INTERNALS__ || !!(window as any).__TAURI__);
};

/**
 * Detect if running on Linux
 */
export const isLinux = () => {
    return navigator.userAgent.toLowerCase().includes('linux');
};

/**
 * Detect if running on macOS
 */
export const isMacOS = () => {
    const ua = navigator.userAgent.toLowerCase();
    return ua.includes('mac os') || ua.includes('macintosh');
};
