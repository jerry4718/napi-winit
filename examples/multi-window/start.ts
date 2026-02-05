import {Application, EventLoop, Extra, Window, WindowAttributes, WindowId} from '@ylcc/napi-winit';
import {drawLargeText, drawText} from 'examples.util/text-renderer';

console.log('🪟  Multi-Window Management Example');
console.log('📝 Demonstrates creating and managing multiple windows');
console.log('');
console.log('💡 Controls:');
console.log('   - N: Create new window');
console.log('   - C: Close current focused window');
console.log('   - 1-9: Activate corresponding window');
console.log('   - ESC: Exit application');
console.log('');

const eventLoop = new EventLoop();

// Window management
interface WindowInfo {
    window: Window;
    surface: Extra.BufferSurface;
    id: WindowId;
    title: string;
    color: number;
    order: number;
}

const windows = new Map<string, WindowInfo>();
let windowCounter = 0;
let focusedWindowId: WindowId | null = null;

// Predefined colors
const colors = [
    0xFFe74c3c, // Red
    0xFF3498db, // Blue
    0xFF2ecc71, // Green
    0xFFf39c12, // Orange
    0xFF9b59b6, // Purple
    0xFF1abc9c, // Teal
    0xFFe67e22, // Dark Orange
    0xFF34495e, // Dark Gray Blue
    0xFFd35400, // Pumpkin
];

function createNewWindow(activeEventLoop: any) {
    const color = colors[windowCounter % colors.length];
    const order = ++windowCounter;

    const attrs = new WindowAttributes()
        .withInnerSize({type: 'Logical', width: 400, height: 300})
        // .withPosition({type: 'Logical', x: 100 + (index - 1) * 50, y: 100 + (index - 1) * 50})
        .withTitle(`Window ${order}`);

    const window = activeEventLoop.createWindow(attrs);
    const surface = new Extra.BufferSurface(window);
    const windowId = window.id();

    const windowInfo: WindowInfo = {
        window,
        surface,
        id: windowId,
        title: `Window ${order}`,
        color,
        order
    };

    windows.set(windowId.rawString(), windowInfo);

    // Request redraw
    window.requestRedraw();

    console.log(`✅ Created window ${order}, total: ${windows.size} windows`);

    return windowInfo;
}

function closeWindow(windowId: WindowId) {
    const windowIdString = windowId.rawString();
    const windowInfo = windows.get(windowIdString);
    if (windowInfo) {
        console.log(`🚪 Closing ${windowInfo.title}`);
        windows.delete(windowIdString);

        if (windows.size === 0) {
            console.log('⚠️  All windows closed, exiting application');
        } else {
            console.log(`📊 Remaining ${windows.size} windows`);
        }
    }
}

const app = Application.withSyncRef({
    onResumed: (eventLoop) => {
        // Create initial window
        createNewWindow(eventLoop);
        console.log('');
    },

    onWindowEvent: (eventLoop, windowId, event) => {
        const windowInfo = windows.get(windowId.rawString());

        if (!windowInfo) return;

        if (event.type === 'CloseRequested') {
            closeWindow(windowId);
            if (windows.size === 0) {
                eventLoop.exit();
            }
            return;
        }

        if (event.type === 'Focused') {
            if (event.focused) {
                focusedWindowId = windowId;
                console.log(`🎯 ${windowInfo.title} gained focus`);
            }
        }

        if (event.type === 'KeyboardInput') {
            const {logicalKey, state} = event.event;

            if (state === 'Released') {
                if (logicalKey.type === 'Character') {
                    const ch = logicalKey.ch.toLowerCase();

                    // N key creates new window
                    if (ch === 'n') {
                        if (windows.size < 9) {
                            createNewWindow(eventLoop);
                        } else {
                            console.log('⚠️  Maximum 9 windows supported');
                        }
                    }
                    // C key closes current window
                    else if (ch === 'c') {
                        if (focusedWindowId) {
                            closeWindow(focusedWindowId);
                            if (windows.size === 0) {
                                eventLoop.exit();
                            }
                        }
                    }
                    // Number keys switch windows
                    else if (ch >= '1' && ch <= '9') {
                        const targetIndex = parseInt(ch);
                        for (const [id, info] of windows.entries()) {
                            if (info.order === targetIndex) {
                                info.window.focusWindow();
                                console.log(`🔍 Switched to ${info.title}`);
                                break;
                            }
                        }
                    }
                }

                if (logicalKey.type === 'Named' && logicalKey.name === 'Escape') {
                    console.log('🚪 Exiting application, closing all windows');
                    eventLoop.exit();
                }
            }
        }

        if (event.type === 'RedrawRequested') {
            windowInfo.window.prePresentNotify();

            windowInfo.surface.presentWithWriter((width, height, view) => {
                // Fill window color
                view.fill(windowInfo.color);

                // Draw window info
                const centerX = Math.floor(width / 2);
                const centerY = Math.floor(height / 2);

                // Draw large number
                const numStr = windowInfo.order.toString();
                const largeNumX = Math.floor(centerX - (numStr.length * 6 * 5) / 2);
                drawLargeText(view, width, height, numStr, largeNumX, centerY - 30, 5, 0xFFFFFFFF);

                // Draw window title
                const titleText = windowInfo.title;
                const titleX = Math.floor(centerX - titleText.length * 3);
                drawText(view, width, titleText, titleX, centerY + 40);

                // Draw controls hint
                drawText(view, width, 'N: New  C: Close  ESC: Exit', 10, height - 20);

                // Draw window count
                drawText(view, width, `Windows: ${numStr}/${windowCounter}`, 10, 10);
            });
        }

        if (event.type === 'Resized') {
            windowInfo.window.requestRedraw();
        }
    },

    onAboutToWait: (eventLoop) => {
        eventLoop.setControlFlow({type: 'Wait'});
    }
});

async function run() {
    while (true) {
        const status = eventLoop.pumpAppEvents(0, app);
        if (status.type === 'Exit') {
            console.log(`\n✨ Application exited, created ${windowCounter} windows total`);
            break;
        }
        await new Promise(resolve => setTimeout(resolve, 1000 / 60));
    }
}

run().catch(console.error);
