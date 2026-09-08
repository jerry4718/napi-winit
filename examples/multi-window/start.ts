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
    id: string;
    window: Window;
    surface: Extra.BufferSurface;
    color: number;
    order: number;
}

const windows: WindowInfo[] = [];
let countWindowCreated = 0;
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
    const color = colors[windows.length % colors.length];
    const order = ++countWindowCreated;

    const attrs = new WindowAttributes()
        .withSurfaceSize({type: 'Logical', width: 400, height: 300})
        // .withPosition({type: 'Logical', x: 100 + (index - 1) * 50, y: 100 + (index - 1) * 50})
        .withTitle(`Window order: ${order}`);

    const window = activeEventLoop.createWindow(attrs);
    const surface = new Extra.BufferSurface(window);
    const windowId = window.id().rawString();

    const windowInfo: WindowInfo = {
        id: windowId,
        window,
        surface,
        color,
        order
    };

    windows.push(windowInfo);

    // Request redraw
    redrawWindows();

    console.log(`✅ Created window order: ${order}, total: ${windows.length} windows`);

    return windowInfo;
}

function closeWindow(windowId: WindowId) {
    const windowIdString = windowId.rawString();
    const windowInfo = windows.find((info) => info.id === windowIdString);
    if (windowInfo) {
        console.log(`🚪 Closing window order: ${windowInfo.order}`);
        windows.splice(windows.indexOf(windowInfo), 1);
        redrawWindows();

        if (windows.length === 0) {
            console.log('⚠️  All windows closed, exiting application');
        } else {
            console.log(`📊 Remaining ${windows.length} windows`);
        }
    }
}

function redrawWindows() {
    for (const info of windows.values()) {
        info.window.requestRedraw();
    }
}

const app = Application.withOptions({
    onCanCreateSurfaces: (eventLoop) => {
        // Create initial window
        createNewWindow(eventLoop);
        console.log('');
    },

    onWindowEvent: (eventLoop, windowId, event) => {
        const windowInfo = windows.find((info) => info.id === windowId.rawString());

        if (!windowInfo) return;

        if (event.type === 'CloseRequested') {
            closeWindow(windowId);
            if (windows.length === 0) {
                eventLoop.exit();
            }
            return;
        }

        if (event.type === 'Focused') {
            if (event.focused) {
                focusedWindowId = windowId;
                console.log(`🎯 Window gained focus order: ${windowInfo.order}`);
            }
        }

        if (event.type === 'KeyboardInput') {
            const {logicalKey, state} = event.event;

            if (state === 'Released' && logicalKey.type === 'Character') {
                const ch = logicalKey.ch.toLowerCase();

                // N key creates new window
                if (ch === 'n') {
                    if (windows.length < 9) {
                        createNewWindow(eventLoop);
                    } else {
                        console.log('⚠️  Maximum 9 windows supported');
                    }
                }
                // C key closes current window
                else if (ch === 'c') {
                    if (focusedWindowId) {
                        closeWindow(focusedWindowId);
                        if (windows.length === 0) {
                            eventLoop.exit();
                        }
                    }
                }
                // Number keys switch windows
                else if (ch >= '1' && ch <= '9') {
                    const windowInfo = windows[parseInt(ch)];
                    windowInfo?.window?.focusWindow()
                }
            }

            if (logicalKey.type === 'Named' && logicalKey.name === 'Tab') {
            }

            if (logicalKey.type === 'Named' && logicalKey.name === 'Escape') {
                console.log('🚪 Exiting application, closing all windows');
                eventLoop.exit();
            }
        }

        if (event.type === 'RedrawRequested') {
            windowInfo.window.prePresentNotify();

            windowInfo.surface.presentWithWriter((view, width, height) => {
                // Fill window color
                view.fill(windowInfo.color);

                // Draw window info
                const centerX = Math.floor(width / 2);
                const centerY = Math.floor(height / 2);

                // Draw large number
                const centerText = `Index: ${windows.findIndex(info => info.id === windowId.rawString())}`;
                const centerTextX = Math.floor(centerX - (centerText.length * 6 * 5) / 2);
                drawLargeText(view, width, height, centerText, centerTextX, centerY - 30, 5, 0xFFFFFFFF);

                // Draw window title
                const titleText = `Window id: ${windowInfo.id}`;
                const titleX = Math.floor(centerX - titleText.length * 3);
                drawText(view, width, titleText, titleX, centerY + 40);

                // Draw controls hint
                drawText(view, width, 'N: New  C: Close  ESC: Exit', 10, height - 20);

                // Draw window count
                drawText(view, width, `Window Count: ${windows.length}, Current Order: ${windowInfo.order}`, 10, 10);
            });
        }

        if (event.type === 'SurfaceResized') {
            windowInfo.window.requestRedraw();
        }
    },

    onAboutToWait: (eventLoop) => {
        eventLoop.setControlFlow({type: 'Wait'});
    }
});

async function run() {
    while (true) {
        const status = eventLoop.pumpAppEvents(null, app);
        if (status.type === 'Exit') {
            console.log(`\n✨ Application exited, created ${countWindowCreated} windows total`);
            break;
        }
        await new Promise(resolve => setTimeout(resolve, 1000 / 60));
    }
}

run().catch(console.error);
