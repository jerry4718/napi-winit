import {Application, EventLoop, Extra, Window, WindowAttributes} from '@ylcc/napi-winit';
import {drawText} from 'examples.util/text-renderer';

console.log('⌨️  Keyboard and Mouse Events Example');
console.log('📝 Demonstrates complete input event handling');
console.log('');
console.log('🖱️  Mouse Operations:');
console.log('   - Move mouse to see coordinates');
console.log('   - Click to draw points');
console.log('   - Scroll to zoom brush');
console.log('');
console.log('⌨️  Keyboard Operations:');
console.log('   - Press any key to see keycode');
console.log('   - C: Clear canvas');
console.log('   - H: Hide/show cursor');
console.log('   - N: Next color');
console.log('   - Shift/Ctrl/Alt: Check modifier keys');
console.log('   - ESC: Exit');
console.log('');

const eventLoop = new EventLoop();

const attrs = new WindowAttributes()
    .withSurfaceSize({type: 'Logical', width: 800, height: 600})
    .withTitle('Keyboard Mouse Example - C:Clear  H:Cursor  ESC:Exit');

let window: Window;
let surface: Extra.BufferSurface;

// 绘制状态
const points: Array<{ x: number, y: number, color: number }> = [];
let cursorX = 0;
let cursorY = 0;
let cursorInWindow = false;
let cursorVisible = true;
let brushSize = 5;

// 修饰键状态
let modifiers = {
    shift: false,
    ctrl: false,
    alt: false,
    meta: false
};

// Random colors
const colors = [
    0xFFFF4444, // Red
    0xFF44FF44, // Green
    0xFF4444FF, // Blue
    0xFFFFFF44, // Yellow
    0xFFFF44FF, // Magenta
    0xFF44FFFF, // Cyan
];
let currentColorIndex = 0;

const app = Application.withOptions({
    onCanCreateSurfaces: (eventLoop) => {
        window = eventLoop.createWindow(attrs);
        surface = new Extra.BufferSurface(window);
        window.requestRedraw();
        console.log('✅ Window created, start drawing!\n');
    },

    onWindowEvent: (eventLoop, _windowId, event) => {
        if (event.type === 'CloseRequested') {
            eventLoop.exit();
            return;
        }

        // Keyboard events
        if (event.type === 'KeyboardInput') {
            const {logicalKey, physicalKey, state, text, repeat} = event.event;

            if (state === 'Pressed' && !repeat) {
                let keyInfo = '';

                if (logicalKey.type === 'Character') {
                    keyInfo = `Character '${logicalKey.ch}'`;
                } else if (logicalKey.type === 'Named') {
                    keyInfo = `Named '${logicalKey.name}'`;
                }

                console.log(`⌨️  ${state}: ${keyInfo}${text ? ` (text: "${text}")` : ''}`);
            }

            if (state === 'Released' && logicalKey.type === 'Character') {
                const ch = logicalKey.ch.toLowerCase();

                if (ch === 'c') {
                    points.length = 0;
                    window.requestRedraw();
                    console.log('🧹 Canvas cleared');
                } else if (ch === 'h') {
                    cursorVisible = !cursorVisible;
                    window.setCursorVisible(cursorVisible);
                    console.log(`👁️  Cursor: ${cursorVisible ? 'visible' : 'hidden'}`);
                } else if (ch === 'n') {
                    currentColorIndex = (currentColorIndex + 1) % colors.length;
                    console.log(`🎨 Color changed`);
                }
            }

            if (state === 'Released' && logicalKey.type === 'Named' && logicalKey.name === 'Escape') {
                eventLoop.exit();
            }
        }

        // Modifier keys state changed
        if (event.type === 'ModifiersChanged') {
            const mods = event.modifiers.state;
            const newModifiers = {
                shift: mods.hasShift(),
                ctrl: mods.hasControl(),
                alt: mods.hasAlt(),
                meta: mods.hasMeta()
            };

            // Detect changes
            const changes: string[] = [];
            if (newModifiers.shift !== modifiers.shift) {
                changes.push(`Shift: ${newModifiers.shift ? 'pressed' : 'released'}`);
            }
            if (newModifiers.ctrl !== modifiers.ctrl) {
                changes.push(`Ctrl: ${newModifiers.ctrl ? 'pressed' : 'released'}`);
            }
            if (newModifiers.alt !== modifiers.alt) {
                changes.push(`Alt: ${newModifiers.alt ? 'pressed' : 'released'}`);
            }
            if (newModifiers.meta !== modifiers.meta) {
                changes.push(`Meta: ${newModifiers.meta ? 'pressed' : 'released'}`);
            }

            if (changes.length > 0) {
                console.log(`🎮 Modifiers: ${changes.join(', ')}`);
            }

            modifiers = newModifiers;

            // Adjust brush size based on modifiers
            if (modifiers.shift) {
                brushSize = 10;
            } else if (modifiers.ctrl) {
                brushSize = 2;
            }
            window.requestRedraw();
        }

        // Mouse move
        if (event.type === 'PointerMoved') {
            const pos = event.position;
            cursorX = Math.floor(pos.x);
            cursorY = Math.floor(pos.y);
            window.requestRedraw();
        }

        // Mouse enter/leave window
        if (event.type === 'PointerEntered') {
            cursorInWindow = true;
            console.log('🖱️  Mouse entered window');
        }

        if (event.type === 'PointerLeft') {
            cursorInWindow = false;
            console.log('🖱️  Mouse left window');
        }

        // Mouse button
        if (event.type === 'PointerButton') {
            const {state: btnState, button} = event;
            const mouseButton = button.type === 'Mouse' ? button.button : undefined;

            if (btnState === 'Pressed' && mouseButton) {
                console.log(`🖱️  Mouse ${mouseButton} at (${cursorX}, ${cursorY})`);

                if (mouseButton?.type === 'Left') {
                    // Add draw point
                    points.push({
                        x: cursorX,
                        y: cursorY,
                        color: colors[currentColorIndex]
                    });
                    window.requestRedraw();
                } else if (mouseButton?.type === 'Right') {
                    // Right click to clear nearby points
                    const threshold = 20;
                    const before = points.length;
                    for (let i = points.length - 1; i >= 0; i--) {
                        const dx = points[i].x - cursorX;
                        const dy = points[i].y - cursorY;
                        if (Math.sqrt(dx * dx + dy * dy) < threshold) {
                            points.splice(i, 1);
                        }
                    }
                    if (points.length < before) {
                        window.requestRedraw();
                        console.log(`🧹 Cleared ${before - points.length} points`);
                    }
                }
            }
        }

        // Mouse wheel
        if (event.type === 'MouseWheel') {
            const delta = event.delta;
            const deltaY = delta.type === 'LineDelta' ? delta.y : delta.type === 'PixelDelta' ? delta.delta.y / 10 : 0;

            brushSize = Math.max(2, Math.min(20, brushSize + Math.sign(deltaY)));
            console.log(`🎨 Brush size: ${brushSize}`);
            window.requestRedraw();
        }

        // Redraw
        if (event.type === 'RedrawRequested') {
            window.prePresentNotify();

            surface.presentWithWriter((view, width, height) => {
                // Clear background
                view.fill(0xFF2a2a3e);

                // Draw all points
                for (const point of points) {
                    drawCircle(view, width, height, point.x, point.y, brushSize, point.color);
                }

                // Draw cursor preview circle (if cursor in window)
                if (cursorInWindow) {
                    drawCircleOutline(view, width, height, cursorX, cursorY, brushSize, 0xFFFFFFFF);
                }

                // Draw info
                drawText(view, width, `Pos: (${cursorX}, ${cursorY})`, 10, 10);
                drawText(view, width, `Size: ${brushSize}`, 10, 25);
                drawText(view, width, `Points: ${points.length}`, 10, 40);

                // Show modifier keys state
                let modStr = '';
                if (modifiers.shift) modStr += 'Shift ';
                if (modifiers.ctrl) modStr += 'Ctrl ';
                if (modifiers.alt) modStr += 'Alt ';
                if (modifiers.meta) modStr += 'Meta ';
                if (modStr) {
                    drawText(view, width, `Mods: ${modStr}`, 10, 55);
                }
            });
        }
    },

    onAboutToWait: (eventLoop) => {
        eventLoop.setControlFlow({type: 'Wait'});
    }
});

// draw circle
function drawCircle(view: Uint32Array, width: number, height: number, cx: number, cy: number, radius: number, color: number) {
    for (let dy = -radius; dy <= radius; dy++) {
        for (let dx = -radius; dx <= radius; dx++) {
            if (dx * dx + dy * dy <= radius * radius) {
                const px = cx + dx;
                const py = cy + dy;

                if (px >= 0 && px < width && py >= 0 && py < height) {
                    const index = py * width + px;
                    view[index] = color;
                }
            }
        }
    }
}

// draw circle outline
function drawCircleOutline(view: Uint32Array, width: number, height: number, cx: number, cy: number, radius: number, color: number) {
    for (let angle = 0; angle < 360; angle += 5) {
        const rad = angle * Math.PI / 180;
        const px = Math.floor(cx + radius * Math.cos(rad));
        const py = Math.floor(cy + radius * Math.sin(rad));

        if (px >= 0 && px < width && py >= 0 && py < height) {
            const index = py * width + px;
            view[index] = color;
        }
    }
}

async function run() {
    while (true) {
        const status = eventLoop.pumpAppEvents(null, app);
        if (status.type === 'Exit') {
            console.log(`\n✨ Application exited, drew ${points.length} points`);
            break;
        }
        await new Promise(resolve => setTimeout(resolve, 1000 / 60));
    }
}

run().catch(console.error);
