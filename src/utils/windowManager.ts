import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';
import { isMacOS, isTauri } from './env';

const sleep = (ms: number) => new Promise<void>((resolve) => setTimeout(resolve, ms));

const waitForVisibleWindow = async (retries: number = 10, delayMs: number = 100) => {
    const win = getCurrentWindow();
    for (let i = 0; i < retries; i++) {
        try {
            if (await win.isVisible()) return win;
        } catch {
            // Ignore transient errors during window init/teardown
        }
        await sleep(delayMs);
    }
    return null;
};

const safeSetDecorations = async (decorated: boolean) => {
    // Workaround: tao/wry on macOS can crash AppKit when toggling window style mask
    // (decorations/resizable). Prefer leaving decorations as-is.
    if (isMacOS()) return;
    const win = await waitForVisibleWindow();
    if (!win) return;
    try {
        const current = await win.isDecorated();
        if (current !== decorated) {
            await win.setDecorations(decorated);
        }
    } catch (error) {
        console.error('Failed to set window decorations:', error);
    }
};

const safeSetResizable = async (resizable: boolean) => {
    // Workaround: tao/wry on macOS can crash AppKit when toggling resizable
    // with certain first-responder states. Prefer keeping the window resizable.
    if (isMacOS()) return;
    const win = await waitForVisibleWindow();
    if (!win) return;
    try {
        const current = await win.isResizable();
        if (current !== resizable) {
            await win.setResizable(resizable);
        }
    } catch (error) {
        console.error('Failed to set window resizable:', error);
    }
};

let ensureFullViewStateInFlight: Promise<void> | null = null;

/**
 * Enter mini view mode
 * @param contentHeight The height of the content to fit
 * @param shouldCenter Whether to center the window (default: false)
 */
export const enterMiniMode = async (contentHeight: number, shouldCenter: boolean = false) => {
    if (!isTauri()) return;
    try {
        const win = await waitForVisibleWindow();
        if (!win) return;

        // Hide window decorations (title bar) first to ensure accurate sizing
        await safeSetDecorations(false);

        // Set window size: width 300, height = content height 
        await win.setSize(new LogicalSize(300, contentHeight+2));

        await win.setAlwaysOnTop(true);
        // Enable window shadow
        await win.setShadow(true);
        // Disable resizing in mini mode
        await safeSetResizable(false);

        // Center window only if requested (usually on first load)
        if (shouldCenter) {
            await win.center();
        }
    } catch (error) {
        console.error('Failed to enter mini mode:', error);
    }
};

/**
 * Exit mini view mode and restore default window state
 */
export const exitMiniMode = async () => {
    if (!isTauri()) return;
    try {
        const win = await waitForVisibleWindow();
        if (!win) return;
        // Restore to a reasonable default size
        await win.setSize(new LogicalSize(1200, 800));
        await win.setAlwaysOnTop(false);
        await win.center();
        // Restore window decorations (title bar)
        await safeSetDecorations(true);
        // Re-enable resizing
        await safeSetResizable(true);
    } catch (error) {
        console.error('Failed to exit mini mode:', error);
    }
};

/**
 * Ensure window is in valid full view state (Self-healing)
 * Used on app startup to recover from improper shutdown in mini mode
 */
export const ensureFullViewState = async () => {
    if (!isTauri()) return;
    if (ensureFullViewStateInFlight) return ensureFullViewStateInFlight;
    ensureFullViewStateInFlight = (async () => {
        try {
            const win = await waitForVisibleWindow();
            if (!win) return;
            const size = await win.outerSize();
            // If window is suspiciously narrow (likely leftover from Mini View), restore default size
            if (size.width < 500) {
                await win.setSize(new LogicalSize(1200, 800));
                await win.center();
            }
            // Always enforce standard window properties for Full View
            await safeSetDecorations(true);
            await safeSetResizable(true);
            await win.setAlwaysOnTop(false);
        } catch (error) {
            console.error('Failed to ensure full view state:', error);
        } finally {
            ensureFullViewStateInFlight = null;
        }
    })();
    return ensureFullViewStateInFlight;
};
