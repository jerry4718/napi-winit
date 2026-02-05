import {Application, EventLoop, Extra, Fullscreen, Window, WindowAttributes} from '@ylcc/napi-winit';
import {drawCenteredText, drawText} from 'examples.util/text-renderer';

console.log('🖥️  Fullscreen Mode Example');
console.log('📝 Demonstrates fullscreen toggle and window decorations');
console.log('');
console.log('💡 Controls:');
console.log('   - F: Toggle fullscreen/windowed mode');
console.log('   - D: Toggle window decorations (title bar)');
console.log('   - M: Maximize/restore window');
console.log('   - V: Toggle window visibility');
console.log('   - ESC: Exit fullscreen or application');
console.log('');

const eventLoop = new EventLoop();

const attrs = new WindowAttributes()
    .withInnerSize({type: 'Logical', width: 800, height: 600})
    .withTitle('Fullscreen Example - F:Fullscreen  D:Decorations  M:Maximize  ESC:Exit');

let window: Window;
let surface: Extra.BufferSurface;

// Status
let isFullscreen = false;
let hasDecorations = true;
let isMaximized = false;
let isVisible = true;
let frameCount = 0;

// Animation elements
const stars: Array<{ x: number, y: number, speed: number, brightness: number }> = [];

function initStars(count: number, width: number, height: number) {
    stars.length = 0;
    for (let i = 0; i < count; i++) {
        stars.push({
            x: Math.random() * width,
            y: Math.random() * height,
            speed: 0.2 + Math.random() * 1.5,
            brightness: 0.3 + Math.random() * 0.7
        });
    }
}

const app = Application.withSyncRef({
    onResumed: (eventLoop) => {
        window = eventLoop.createWindow(attrs);
        surface = new Extra.BufferSurface(window);

        const size = window.innerSize();
        const width = size.type === 'Logical' ? size.width : size.width;
        const height = size.type === 'Logical' ? size.height : size.height;
        initStars(200, width, height);

        window.requestRedraw();
        console.log('✅ Window created');
        console.log(`📐 Initial size: ${width}x${height}\n`);
    },

    onWindowEvent: (eventLoop, _windowId, event) => {
        if (event.type === 'CloseRequested') {
            console.log('👋 Closing window');
            eventLoop.exit();
            return;
        }

        if (event.type === 'KeyboardInput') {
            const {logicalKey, state} = event.event;

            if (state === 'Released' && logicalKey.type === 'Character') {
                const ch = logicalKey.ch.toLowerCase();

                // F key toggles fullscreen
                if (ch === 'f') {
                    isFullscreen = !isFullscreen;

                    if (isFullscreen) {
                        window.setFullscreen(Fullscreen.Borderless);
                        console.log('🖥️  Switched to fullscreen mode');
                    } else {
                        window.setFullscreen(null);
                        console.log('🪟  Switched to windowed mode');
                    }
                }
                // D key toggles decorations
                else if (ch === 'd') {
                    hasDecorations = !hasDecorations;
                    window.setDecorations(hasDecorations);
                    console.log(`🎨 Window decorations: ${hasDecorations ? 'shown' : 'hidden'}`);
                }
                // M key maximize/restore
                else if (ch === 'm') {
                    window.setMaximized(!isMaximized);
                    console.log(`📏 Window: ${isMaximized ? 'restore' : 'maximize'}`);
                }
                // V key toggle visibility
                else if (ch === 'v') {
                    isVisible = !isVisible;
                    window.setVisible(isVisible);
                    console.log(`👁️  Window: ${isVisible ? 'visible' : 'hidden'}`);
                }
            }

            if (state === 'Released' && logicalKey.type === 'Named' && logicalKey.name === 'Escape') {
                if (isFullscreen) {
                    // If in fullscreen mode, exit fullscreen first
                    isFullscreen = false;
                    window.setFullscreen(null);
                    console.log('🪟  Exited fullscreen mode');
                } else {
                    // Otherwise exit application
                    console.log('🚪 Exiting application');
                    eventLoop.exit();
                }
            }
        }

        if (event.type === 'Resized') {
            const {width, height} = event.size;
            console.log(`📐 Window resized: ${width}x${height}`);

            // Reinitialize stars
            initStars(200, width, height);
            window.requestRedraw();
        }

        if (event.type === 'Resized') {
            isMaximized = window.isMaximized();
            console.log(`📏 Window ${isMaximized ? 'maximized' : 'restored'}`);
        }

        if (event.type === 'RedrawRequested') {
            window.prePresentNotify();

            surface.presentWithWriter((width, height, view) => {
                frameCount++;

                // Dark background
                view.fill(0xFF0a0a0f);

                // Update and draw stars
                for (const star of stars) {
                    // Update position
                    star.y += star.speed;

                    // Loop
                    if (star.y > height) {
                        star.y = 0;
                        star.x = Math.random() * width;
                    }

                    // Draw star
                    const x = Math.floor(star.x);
                    const y = Math.floor(star.y);
                    const brightness = Math.floor(255 * star.brightness);
                    const color = 0xFF000000 | (brightness << 16) | (brightness << 8) | brightness;

                    if (x >= 0 && x < width && y >= 0 && y < height) {
                        view[y * width + x] = color;

                        // Add glow effect
                        if (star.brightness > 0.7) {
                            const positions = [
                                [x - 1, y], [x + 1, y], [x, y - 1], [x, y + 1]
                            ];
                            for (const [px, py] of positions) {
                                if (px >= 0 && px < width && py >= 0 && py < height) {
                                    const glowBrightness = Math.floor(brightness * 0.5);
                                    const glowColor = 0xFF000000 | (glowBrightness << 16) | (glowBrightness << 8) | glowBrightness;
                                    view[py * width + px] = glowColor;
                                }
                            }
                        }
                    }
                }

                // Draw title
                const titleY = Math.floor(height * 0.3);
                const title = isFullscreen ? 'FULLSCREEN MODE' : 'WINDOWED MODE';
                drawCenteredText(view, width, height, title, titleY, 0xFFFFFFFF);

                // Draw state info
                const infoY = Math.floor(height * 0.4);
                drawCenteredText(view, width, height, `${width} x ${height}`, infoY + 20, 0xFFAAAAAA);

                // Draw hints
                const hintY = Math.floor(height * 0.7);
                drawCenteredText(view, width, height, 'Press F to toggle fullscreen', hintY, 0xFF888888);
                drawCenteredText(view, width, height, 'Press D to toggle decorations', hintY + 15, 0xFF888888);
                drawCenteredText(view, width, height, 'Press M to maximize', hintY + 30, 0xFF888888);
                drawCenteredText(view, width, height, 'Press ESC to exit', hintY + 45, 0xFF888888);

                // Draw status indicators
                const statusY = height - 30;
                let statusText = '';
                if (isFullscreen) statusText += '[FULLSCREEN] ';
                if (!hasDecorations) statusText += '[NO-DECORATIONS] ';
                if (isMaximized) statusText += '[MAXIMIZED] ';

                if (statusText) {
                    drawCenteredText(view, width, height, statusText.trim(), statusY, 0xFF44FF44);
                }

                // FPS counter
                drawText(view, width, `FPS: ~60  Frame: ${frameCount}`, 10, 10);
            });

            // Request next frame
            window.requestRedraw();
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
            console.log('\n✨ Application exited');
            break;
        }
        await new Promise(resolve => setTimeout(resolve, 1000 / 60));
    }
}

run().catch(console.error);
