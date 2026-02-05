import {
    Application,
    EventLoop,
    WindowAttributes,
    Window,
    Extra
} from '@ylcc/napi-winit';
import {drawText} from 'examples.util/text-renderer';

console.log('🎨 Animation Rendering Example');
console.log('📝 Demonstrates soft surface rendering and animation effects');
console.log('💡 Press R to toggle animation on/off');
console.log('💡 Press SPACE to change background color');
console.log('💡 Press ESC to exit\n');

const eventLoop = new EventLoop();

const attrs = new WindowAttributes()
    .withInnerSize({type: 'Logical', width: 800, height: 600})
    .withTitle('Animation Example - R:Toggle  SPACE:Color  ESC:Exit');

let window: Window;
let surface: Extra.BufferSurface;
let frameCount = 0;
let requestRedraw = true; // 默认开启动画

// 动画状态
let ball = {
    x: 100,
    y: 100,
    vx: 3,
    vy: 2,
    radius: 25,
    color: 0xFFFF4444
};

let backgroundColor = 0xFF1a1a2e;
const backgrounds = [
    0xFF1a1a2e, // 深蓝
    0xFF16213e, // 深青
    0xFF0f3460, // 蓝色
    0xFF533483  // 紫色
];
let bgIndex = 0;

// FPS 计算
const frameTimes: number[] = [];
let lastFpsUpdate = Date.now();
let currentFps = 0;

const app = Application.withSyncRef({
    onResumed: (eventLoop) => {
        window = eventLoop.createWindow(attrs);
        surface = new Extra.BufferSurface(window);
        window.requestRedraw();
        console.log('✅ Window created, animation started');
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
                if (logicalKey.ch === 'r' || logicalKey.ch === 'R') {
                    requestRedraw = !requestRedraw;
                    console.log(`🎬 Animation: ${requestRedraw ? '✅ ON' : '⏸️  OFF'}`);
                }
                if (requestRedraw) {
                    window.requestRedraw();
                }
            }

            if (state === 'Released' && logicalKey.type === 'Named') {
                if (logicalKey.name === "Space") {
                    bgIndex = (bgIndex + 1) % backgrounds.length;
                    backgroundColor = backgrounds[bgIndex];
                    console.log(`🎨 Background color changed`);
                }
                if (requestRedraw) {
                    window.requestRedraw();
                }
            }

            if (state === 'Released' && logicalKey.type === 'Named') {
                if (logicalKey.name === 'Escape') {
                    console.log('🚪 Exiting application');
                    eventLoop.exit();
                }
            }
        }

        if (event.type === 'RedrawRequested') {
            window.prePresentNotify();

            surface.presentWithWriter((width, height, view) => {
                frameCount++;

                // 更新 FPS
                const now = Date.now();
                frameTimes.push(now);
                frameTimes.splice(0, frameTimes.findIndex(t => t > now - 1000));
                if (now - lastFpsUpdate > 500) {
                    currentFps = frameTimes.length;
                    lastFpsUpdate = now;
                }

                // 清空背景
                view.fill(backgroundColor);

                // 更新球的位置
                ball.x += ball.vx;
                ball.y += ball.vy;

                // 边界碰撞检测
                if (ball.x - ball.radius <= 0 || ball.x + ball.radius >= width) {
                    ball.vx *= -1;
                    ball.x = Math.max(ball.radius, Math.min(width - ball.radius, ball.x));
                }
                if (ball.y - ball.radius <= 0 || ball.y + ball.radius >= height) {
                    ball.vy *= -1;
                    ball.y = Math.max(ball.radius, Math.min(height - ball.radius, ball.y));
                }

                // 绘制球（简单圆形）
                const ballX = Math.floor(ball.x);
                const ballY = Math.floor(ball.y);

                for (let dy = -ball.radius; dy <= ball.radius; dy++) {
                    for (let dx = -ball.radius; dx <= ball.radius; dx++) {
                        if (dx * dx + dy * dy <= ball.radius * ball.radius) {
                            const px = ballX + dx;
                            const py = ballY + dy;

                            if (px >= 0 && px < width && py >= 0 && py < height) {
                                const index = py * width + px;
                                view[index] = ball.color;
                            }
                        }
                    }
                }

                // 绘制拖尾效果
                const trailLength = 5;
                for (let i = 1; i <= trailLength; i++) {
                    const tx = ballX - ball.vx * i * 3;
                    const ty = ballY - ball.vy * i * 3;
                    const alpha = 0xFF - (i * 40);
                    const trailRadius = Math.max(5, ball.radius - i * 3);

                    for (let dy = -trailRadius; dy <= trailRadius; dy++) {
                        for (let dx = -trailRadius; dx <= trailRadius; dx++) {
                            if (dx * dx + dy * dy <= trailRadius * trailRadius) {
                                const px = Math.floor(tx + dx);
                                const py = Math.floor(ty + dy);

                                if (px >= 0 && px < width && py >= 0 && py < height) {
                                    const index = py * width + px;
                                    view[index] = (alpha << 24) | (ball.color & 0x00FFFFFF);
                                }
                            }
                        }
                    }
                }

                // Draw FPS text (simple pixel display)
                drawText(view, width, `FPS: ${currentFps}`, 10, 10);
                drawText(view, width, `Frame: ${frameCount}`, 10, 25);
            });

            if (requestRedraw) {
                window.requestRedraw();
            }
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
            console.log(`\n✨ Application exited, total frames: ${frameCount}`);
            break;
        }
        await new Promise(resolve => setTimeout(resolve, 1000 / 60)); // 60 FPS
    }
}

run().catch(console.error);
