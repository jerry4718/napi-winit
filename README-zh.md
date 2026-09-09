# napi-winit

[![npm version](https://img.shields.io/npm/v/@ylcc/napi-winit.svg)](https://www.npmjs.com/package/@ylcc/napi-winit)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> 为 Node.js 和 Deno 提供原生性能的窗口管理

## 1. 项目简介

**napi-winit** 是一个为 Node.js 和 Deno 打造的高性能原生窗口管理库，基于 Rust 的 [winit](https://github.com/rust-windowing/winit) 库和 NAPI-RS 绑定构建。它提供了创建原生窗口、处理窗口事件和管理应用程序生命周期的完整解决方案，性能开销极小。

### 1.1 主要特性

- 🚀 **原生性能**：使用 Rust 构建，提供接近原生的执行速度
- 🪟 **窗口管理**：完全控制原生窗口的创建和管理
- 🎮 **事件处理**：全面支持键盘、鼠标和窗口事件
- ⚡ **灵活的控制流**：支持 Wait、WaitUntil 和 Poll 三种模式
- 🎨 **软件渲染**：内置基于 [softbuffer](https://github.com/rust-windowing/softbuffer) 的 BufferSurface 渲染
- 🌍 **跨平台**：支持 Windows、macOS、Linux、FreeBSD（10+ 架构）
- 📘 **TypeScript 支持**：完整的 TypeScript 类型定义
- 🦕 **Deno 兼容**：同时支持 Node.js 和 Deno

## 2. 安装

### 2.1 Node.js

使用 npm 安装：

```bash
npm install @ylcc/napi-winit
```

使用 yarn 安装：

```bash
yarn add @ylcc/napi-winit
```

使用 pnpm 安装：

```bash
pnpm add @ylcc/napi-winit
```

### 2.2 Deno

```javascript
import { Application, EventLoop, Window, WindowAttributes } from 'npm:@ylcc/napi-winit';
```

## 3. 快速开始

以下是一个最小化示例，展示如何创建窗口并处理基本事件：

```typescript
import { Application, EventLoop, WindowAttributes } from '@ylcc/napi-winit';

// 创建事件循环
const eventLoop = new EventLoop();

// 配置窗口属性
const attrs = new WindowAttributes()
    .withActive(true)
    .withResizable(true)
    .withSurfaceSize({ type: 'Logical', width: 800, height: 600 })
    .withTitle('Hello napi-winit');

// 创建应用程序并设置事件处理器
const app = Application.withOptions({
    onCanCreateSurfaces: (eventLoop) => {
        // 应用准备就绪时创建窗口
        const window = eventLoop.createWindow(attrs);
        console.log('窗口已创建');
    },
    onWindowEvent: (eventLoop, windowId, event) => {
        // 处理窗口事件
        if (event.type === 'CloseRequested') {
            console.log('用户请求关闭窗口');
            eventLoop.exit();
        } else if (event.type === 'KeyboardInput') {
            const { state, text, logicalKey } = event.event;
            console.log(`按键: ${text}, 状态: ${state}`);
            
            // 按 Escape 键退出
            if (logicalKey.type === 'Named' && logicalKey.name === 'Escape') {
                eventLoop.exit();
            }
        }
    },
    onAboutToWait: (eventLoop) => {
        // 设置控制流模式
        eventLoop.setControlFlow({ type: 'Wait' });
    }
});

// 运行事件循环
async function run() {
    while (true) {
        const status = eventLoop.pumpAppEvents(null, app);
        if (status.type === 'Exit') {
            console.log(`退出，代码: ${status.code}`);
            break;
        }
        // 控制事件循环频率（60 FPS）
        await new Promise(resolve => setTimeout(resolve, 1000 / 60));
    }
}

run().catch(console.error);
```

## 4. API 参考

### 4.1 EventLoop

事件循环是窗口系统的核心，负责处理所有窗口事件。

```typescript
const eventLoop = new EventLoop();

// 处理应用事件
const status = eventLoop.pumpAppEvents(null, app);

// 设置控制流
eventLoop.setControlFlow(controlFlow);

// 创建窗口
const window = eventLoop.createWindow(attributes);
```

### 4.2 WindowAttributes

用于配置窗口的属性。

```typescript
const attrs = new WindowAttributes()
    .withActive(true) // 窗口是否激活
    .withResizable(true) // 窗口是否可调整大小
    .withSurfaceSize({ type: 'Logical', width: 800, height: 600 }) // 窗口表面尺寸
    .withPosition({ type: 'Logical', x: 100, y: 100 }) // 窗口位置
    .withTitle('Window Title') // 窗口标题
    .withTransparent(false) // 窗口是否透明
    .withFullscreen(null); // 是否全屏
```

### 4.3 Window

窗口实例方法，用于控制和查询窗口状态：

```typescript
// 重绘和呈现
window.requestRedraw(); // 请求重绘事件
window.prePresentNotify(); // 呈现前通知（某些平台需要）

// 尺寸和位置
const surfaceSize = window.surfaceSize(); // 获取当前表面尺寸
const outerSize = window.outerSize(); // 获取外部尺寸（包括装饰）
const surfacePosition = window.surfacePosition(); // 获取表面位置
const outerPos = window.outerPosition(); // 获取外部位置

// 请求尺寸变化（返回实际尺寸或 null 如果不支持）
const actualSize = window.requestSurfaceSize({ type: 'Logical', width: 1024, height: 768 });

// 设置位置
window.setOuterPosition({ type: 'Logical', x: 100, y: 100 });

// 尺寸约束
window.setMinSurfaceSize({ type: 'Logical', width: 400, height: 300 });
window.setMaxSurfaceSize({ type: 'Logical', width: 1920, height: 1080 });

// 窗口属性
window.setTitle('新标题');
window.setVisible(true);
window.setResizable(false);
window.setDecorations(true);

// 显示属性
const scaleFactor = window.scaleFactor(); // 获取 DPI 缩放因子
const id = window.id(); // 获取唯一窗口 ID

// 光标控制
import { Cursor, CursorIcon } from '@ylcc/napi-winit';
window.setCursor(Cursor.fromIcon('Pointer'));
window.setCursorVisible(false);

// 全屏
window.setFullscreen({
    type: 'Borderless',
    monitor: null // null = 当前显示器
});
window.setFullscreen(null); // 退出全屏

// 焦点和注意
window.focusWindow();
window.requestUserAttention('Informational'); // 或 'Critical'

// 高级功能
window.resetDeadKeys(); // 重置死键状态
window.requestImeUpdate({ type: 'Enable', capabilities: { hintAndPurpose: true, cursorArea: false, surroundingText: false }, data: { hintAndPurpose: { hint: { completion: true }, purpose: 'Normal' } } }); // 启用输入法
```

### 4.4 Application

应用程序类，用于处理应用级别的事件。

```typescript
const app = Application.withOptions({
    onNewEvents: (eventLoop, cause) => {
        // 新事件到达时调用
    },
    onCanCreateSurfaces: (eventLoop) => {
        // 应用恢复时调用，通常在这里创建窗口
    },
    onWindowEvent: (eventLoop, windowId, event) => {
        // 窗口事件处理
    },
    onAboutToWait: async (eventLoop) => {
        // 事件循环即将等待时调用，用于设置控制流
    }
});
```

### 4.5 ControlFlow

控制事件循环的行为：

```typescript
// 等待模式 - 无限期等待下一个事件（最高效）
eventLoop.setControlFlow({ type: 'Wait' });

// WaitUntil 模式 - 等待指定时间或下一个事件
import { Instant, Duration } from '@ylcc/napi-winit';

// 等到指定时刻
eventLoop.setControlFlow({ 
    type: 'WaitUntil', 
    timeout: Instant.afterMillis(16) // ~60 FPS
});

// 等待指定时长
eventLoop.setControlFlow({ 
    type: 'WaitUntil', 
    timeout: Instant.afterNanos(1_000_000 / 120) // ~120 FPS
});

// Poll 模式 - 立即处理所有事件，不等待
eventLoop.setControlFlow({ type: 'Poll' });

// 退出事件循环
eventLoop.exit();

// 检查是否正在退出
if (eventLoop.exiting()) {
    console.log('事件循环正在退出');
}

// 获取当前控制流
const currentFlow = eventLoop.controlFlow();
console.log(currentFlow.type); // 'Wait' | 'WaitUntil' | 'Poll'
```

### 4.6 Extra 功能

额外的实用工具，增强功能：

```typescript
import { Extra, Duration, Instant } from '@ylcc/napi-winit';

// BufferSurface 渲染
const surface = new Extra.BufferSurface(window);

// 方法 1：使用回调函数绘制
surface.presentWithWriter((view, width, height) => {
    // view 是 Uint32Array - 每个元素是 0xAARRGGBB
    for (let i = 0; i < view.length; i++) {
        view[i] = 0xFF00FF00; // 绿色
    }
});

// 方法 2：使用预填充的缓冲区
const buffer = new Uint32Array(width * height);
buffer.fill(0xFFFF0000); // 红色
surface.presentWithTyped(buffer);

// 异步睡眠（基于 Tokio）
await Extra.tokioSleep(Duration.fromMillis(100));

// 基于时间的执行
const instant = Instant.now();
const future = Instant.afterMillis(1000);

// 时长操作
const duration = Duration.fromSecs(1);
const doubled = Duration.mul(duration, 2);

// 线程池执行
const pool = Extra.ThreadPool.default();
pool.execute(() => {
    console.log('在线程池中运行');
});

// 获取原始窗口句柄（用于自定义渲染 API）
const options = Extra.getRwh05Options(window);
console.log(options.system); // 'win32' | 'cocoa' | 'x11' | 'wayland'
```

## 5. 事件处理

### 5.1 常见窗口事件

在 `onWindowEvent` 回调中处理各种窗口事件：

```typescript
onWindowEvent: (eventLoop, windowId, event) => {
    switch (event.type) {
        case 'CloseRequested':
            // 用户请求关闭窗口（例如点击关闭按钮）
            console.log('收到关闭请求');
            eventLoop.exit();
            break;

        case 'RedrawRequested':
            // 窗口需要重绘
            // 在此执行渲染
            break;

        case 'SurfaceResized':
            // 窗口表面尺寸已改变
            const { width, height } = event.size;
            console.log(`调整大小至 ${width}x${height}`);
            break;

        case 'Moved':
            // 窗口位置已改变
            const { x, y } = event.position;
            console.log(`移动到 (${x}, ${y})`);
            break;

        case 'Focused':
            // 窗口获得或失去焦点
            console.log(`焦点: ${event.focused}`);
            break;

        case 'KeyboardInput':
            // 键盘输入事件
            const keyEvent = event.event;
            const { state, logicalKey, physicalKey, text, repeat } = keyEvent;
            
            // 处理字符键
            if (logicalKey.type === 'Character') {
                console.log(`字符: ${logicalKey.ch}, 状态: ${state}`);
            }
            
            // 处理命名键（Escape、Enter 等）
            if (logicalKey.type === 'Named') {
                console.log(`命名键: ${logicalKey.name}, 状态: ${state}`);
            }
            break;

        case 'ModifiersChanged':
            // 修饰键状态改变（Shift、Ctrl、Alt、Meta）
            const mods = event.modifiers.state;
            console.log({
                shift: mods.hasShift(),
                ctrl: mods.hasControl(),
                alt: mods.hasAlt(),
                meta: mods.hasMeta()
            });
            break;

        case 'PointerButton':
            // 鼠标按钮事件
            const { button, state: btnState } = event;
            console.log(`鼠标按钮 ${button.type}: ${btnState}`);
            break;

        case 'MouseWheel':
            // 鼠标滚轮事件
            const { deltaX, deltaY } = event.delta;
            console.log(`滚轮: (${deltaX}, ${deltaY})`);
            break;

        case 'PointerMoved':
            // 指针位置改变
            const position = event.position;
            console.log(`指针: (${position.x}, ${position.y})`);
            break;

        case 'PointerEntered':
            // 指针进入窗口
            console.log('指针进入');
            break;

        case 'PointerLeft':
            // 指针离开窗口
            console.log('指针离开');
            break;

        case 'ScaleFactorChanged':
            // DPI 缩放因子改变
            const { scaleFactor, surfaceSizeWriter } = event;
            console.log(`新缩放因子: ${scaleFactor}`);
            break;

        case 'ThemeChanged':
            // 系统主题改变
            console.log(`主题: ${event.theme}`); // 'Light' 或 'Dark'
            break;

        case 'Destroyed':
            // 窗口已销毁
            console.log('窗口已销毁');
            break;
    }
}
```

## 6. 控制流模式

napi-winit 支持三种控制流模式来优化性能和响应性：

### Wait 模式（推荐用于事件驱动应用）

无限期等待下一个事件。最节能的选项。

```typescript
eventLoop.setControlFlow({ type: 'Wait' });
```

**适用场景：**
- 纯事件驱动的应用
- 不需要持续动画或更新
- 电池续航很重要（移动/笔记本场景）

### WaitUntil 模式（最适合动画）

等待到指定时间或下一个事件到达。

```typescript
import { Instant } from '@ylcc/napi-winit';

// 等待 16ms（~60 FPS）
eventLoop.setControlFlow({ 
    type: 'WaitUntil', 
    timeout: Instant.afterMillis(16) 
});

// 等待 8.33ms（~120 FPS）
eventLoop.setControlFlow({ 
    type: 'WaitUntil', 
    timeout: Instant.afterNanos(1_000_000 / 120) 
});
```

**适用场景：**
- 运行动画或游戏
- 需要一致的帧时序
- 想要精确控制更新频率

### Poll 模式（高频更新）

立即处理所有事件，不等待。

```typescript
import { Extra, Duration } from '@ylcc/napi-winit';

onAboutToWait: async (eventLoop) => {
    // 添加小延迟以防止 100% CPU 使用率
    await Extra.tokioSleep(Duration.fromMillis(1));
    eventLoop.setControlFlow({ type: 'Poll' });
}
```

**适用场景：**
- 需要最大响应性
- 处理密集的实时数据
- CPU 使用率不是问题

### 模式对比

| 模式 | CPU 使用 | 延迟 | 最适合 |
|------|---------|------|--------|
| **Wait** | 最低 | 低 | 事件驱动应用 |
| **WaitUntil** | 中等 | 可预测 | 动画、游戏 |
| **Poll** | 高 | 最低 | 实时处理 |

## 7. 高级示例

### 7.1 交互式控制流示例

此示例演示如何通过键盘输入切换控制流模式：

```typescript
import { Application, EventLoop, WindowAttributes, type ControlFlow,Instant,Duration,Extra } from '@ylcc/napi-winit';

const eventLoop = new EventLoop();

const attrs = new WindowAttributes()
    .withSurfaceSize({ type: 'Logical', width: 800, height: 600 })
    .withTitle('控制流演示 - 按 1、2、3 切换模式，ESC 退出');

let window;
let mode: ControlFlow['type'] = 'Wait';
let waitCancelled = false;

const app = Application.withOptions({
    onCanCreateSurfaces: (eventLoop) => {
        window = eventLoop.createWindow(attrs);
        console.log('窗口已创建。按 1（Wait）、2（WaitUntil）、3（Poll）');
    },
    
    onNewEvents: (eventLoop, cause) => {
        // 跟踪等待是否被取消
        waitCancelled = (mode === 'WaitUntil' && cause.type === 'WaitCancelled');
    },
    
    onWindowEvent: (eventLoop, windowId, event) => {
        if (event.type === 'CloseRequested') {
            eventLoop.exit();
            return;
        }
        
        if (event.type === 'KeyboardInput') {
            const { logicalKey, state } = event.event;
            
            if (state === 'Released' && logicalKey.type === 'Character') {
                switch (logicalKey.ch) {
                    case '1':
                        mode = 'Wait';
                        console.log('→ 切换到 WAIT 模式（最高效）');
                        break;
                    case '2':
                        mode = 'WaitUntil';
                        console.log('→ 切换到 WAITUNTIL 模式（~60 FPS）');
                        break;
                    case '3':
                        mode = 'Poll';
                        console.log('→ 切换到 POLL 模式（连续）');
                        break;
                }
            }
            
            if (state === 'Released' && logicalKey.type === 'Named') {
                if (logicalKey.name === 'Escape') {
                    eventLoop.exit();
                }
            }
        }
    },
    
    onAboutToWait: async (eventLoop) => {
        switch (mode) {
            case 'Wait':
                eventLoop.setControlFlow({ type: 'Wait' });
                break;
                
            case 'WaitUntil':
                if (waitCancelled) {
                    eventLoop.setControlFlow({ 
                        type: 'WaitUntil', 
                        timeout: Instant.afterMillis(16) // ~60 FPS
                    });
                }
                break;
                
            case 'Poll':
                await Extra.tokioSleep(Duration.fromMillis(1));
                eventLoop.setControlFlow({ type: 'Poll' });
                break;
        }
    }
});

async function run() {
    while (true) {
        const status = eventLoop.pumpAppEvents(null, app);
        if (status.type === 'Exit') {
            console.log(`退出，代码: ${status.code}`);
            break;
        }
        await new Promise(resolve => setTimeout(resolve, 1000 / 60));
    }
}

run().catch(console.error);
```

### 7.2 BufferSurface 动画渲染示例

此示例展示使用软件渲染实现流畅动画：

```typescript
import { Application, EventLoop, WindowAttributes, Window, Instant, Extra } from '@ylcc/napi-winit';

const eventLoop = new EventLoop();

const attrs = new WindowAttributes()
    .withSurfaceSize({ type: 'Logical', width: 800, height: 600 })
    .withTitle('动画示例 - 按 R 切换重绘');

let window: Window;
let surface: Extra.BufferSurface;
let frameCount = 0;
let requestRedraw = false;

// 动画状态
let rectangleX = 0;
let velocityX = 2;

const app = Application.withOptions({
    onCanCreateSurfaces: (eventLoop) => {
        window = eventLoop.createWindow(attrs);
        surface = new Extra.BufferSurface(window);
        // 请求初始重绘
        window.requestRedraw();
        console.log('按 R 切换动画，ESC 退出');
    },
    
    onWindowEvent: (eventLoop, windowId, event) => {
        if (event.type === 'CloseRequested') {
            eventLoop.exit();
            return;
        }
        
        if (event.type === 'KeyboardInput') {
            const { logicalKey, state } = event.event;
            
            if (state === 'Released' && logicalKey.type === 'Character') {
                if (logicalKey.ch === 'r' || logicalKey.ch === 'R') {
                    requestRedraw = !requestRedraw;
                    console.log(`动画: ${requestRedraw ? '开启' : '关闭'}`);
                }
            }
            
            if (state === 'Released' && logicalKey.type === 'Named') {
                if (logicalKey.name === 'Escape') {
                    eventLoop.exit();
                }
            }
        }
        
        if (event.type === 'RedrawRequested') {
            // 通知平台我们即将呈现
            window.prePresentNotify();
            
            // 渲染帧
            surface.presentWithWriter((view, width, height) => {
                frameCount++;
                
                // 清空为暗色背景（ARGB 格式：0xAARRGGBB）
                view.fill(0xFF101010);
                
                // 更新矩形位置
                rectangleX += velocityX;
                if (rectangleX <= 0 || rectangleX >= width - 100) {
                    velocityX *= -1; // 反弹
                }
                
                // 绘制动画矩形
                const rectWidth = 100;
                const rectHeight = 50;
                const y = Math.floor(
                    Math.sin(frameCount * 0.05) * 100 + height / 2 - rectHeight / 2
                );
                
                // 根据帧数计算颜色
                const hue = (frameCount * 2) % 360;
                const color = hslToRgb(hue, 100, 50);
                
                // 绘制矩形
                for (let row = 0; row < rectHeight; row++) {
                    for (let col = 0; col < rectWidth; col++) {
                        const px = Math.floor(rectangleX) + col;
                        const py = y + row;
                        
                        if (px >= 0 && px < width && py >= 0 && py < height) {
                            const index = py * width + px;
                            view[index] = color;
                        }
                    }
                }
                
                // 绘制 FPS 计数器（简单点阵）
                const fps = frameCount % 60;
                for (let i = 0; i < fps; i++) {
                    const idx = i * 10;
                    if (idx < width) {
                        view[idx] = 0xFFFFFFFF; // 白色点
                    }
                }
            });
            
            // 如果动画已启用，请求下一帧
            if (requestRedraw) {
                window.requestRedraw();
            }
        }
    },
    
    onAboutToWait: (eventLoop) => {
        // 使用 Wait 模式，仅在需要时重绘
        eventLoop.setControlFlow({ type: 'Wait' });
    }
});

// 辅助函数：将 HSL 转换为 RGB（返回 0xAARRGGBB 格式）
function hslToRgb(h: number, s: number, l: number): number {
    s /= 100;
    l /= 100;
    const k = (n: number) => (n + h / 30) % 12;
    const a = s * Math.min(l, 1 - l);
    const f = (n: number) => l - a * Math.max(-1, Math.min(k(n) - 3, Math.min(9 - k(n), 1)));
    
    const r = Math.round(255 * f(0));
    const g = Math.round(255 * f(8));
    const b = Math.round(255 * f(4));
    
    return (0xFF << 24) | (r << 16) | (g << 8) | b;
}

async function run() {
    while (true) {
        const status = eventLoop.pumpAppEvents(null, app);
        if (status.type === 'Exit') {
            console.log(`退出，代码: ${status.code}`);
            break;
        }
        await new Promise(resolve => setTimeout(resolve, 1000 / 60)); // 60 FPS
    }
}

run().catch(console.error);
```

## 8. 平台支持

napi-winit 为以下平台提供预构建的二进制文件：

| 平台 | 架构 | 说明 |
|------|-----|------|
| **Windows** | x86_64, i686, aarch64 | 完全支持 |
| **macOS** | x86_64 (Intel), aarch64 (Apple Silicon) | 完全支持 |
| **Linux (GNU)** | x86_64, aarch64, armv7 | 完全支持 |
| **Linux (musl)** | x86_64, aarch64 | 兼容 Alpine Linux |
| **FreeBSD** | x86_64 | 完全支持 |

### 系统要求

- **Node.js**: >= 10.0.0
- **Deno**: 最新稳定版本
- **操作系统**: Windows 7+、macOS 10.12+、Linux（任何现代发行版）

### 显示服务器支持（Linux）

- ✅ X11（通过 x11-dlopen 动态链接）
- ✅ Wayland（通过 wayland-dlopen 动态链接）

库会在运行时自动检测可用的显示服务器。

## 9. 最佳实践

### 9.1 事件循环管理

**控制事件循环频率**以平衡 CPU 使用率和响应性：

```typescript
// 推荐：控制轮询频率
async function run() {
    while (true) {
        const status = eventLoop.pumpAppEvents(null, app);
        if (status.type === 'Exit') break;
        await new Promise(resolve => setTimeout(resolve, 1000 / 60)); // 60 FPS
    }
}

// 也推荐：使用 Wait 模式让操作系统唤醒应用
onAboutToWait: (eventLoop) => {
    eventLoop.setControlFlow({ type: 'Wait' }); // 最高效
}
```

### 9.2 资源管理

始终正确清理资源：

```typescript
let window: Window | null = null;
let surface: Extra.BufferSurface | null = null;

const app = Application.withOptions({
    onCanCreateSurfaces: (eventLoop) => {
        window = eventLoop.createWindow(attrs);
        surface = new Extra.BufferSurface(window);
    },
    onWindowEvent: (eventLoop, windowId, event) => {
        if (event.type === 'CloseRequested') {
            // 退出前清理
            surface = null;
            window = null;
            eventLoop.exit();
        }
    }
});
```

### 9.3 性能提示

1. **尽可能使用 Wait 模式**：对于事件驱动应用最节能
2. **批量重绘**：仅在必要时调用 `requestRedraw()`
3. **分析渲染性能**：在呈现帧前使用 `prePresentNotify()`
4. **动画考虑使用 WaitUntil**：提供一致的帧时序

### 9.4 跨平台注意事项

- **窗口装饰**：不同平台行为可能有差异
- **DPI 缩放**：使用逻辑尺寸并处理缩放因子变化
- **键盘布局**：使用不同键盘配置进行测试
- **全屏模式**：检查平台特定的全屏行为

## 10. 示例项目

`examples/` 目录包含各种示例项目，展示不同的功能和使用场景：

### 可用示例

| 示例 | 描述 | 主要特性 |
|------|------|---------|
| **hello-world** | 最简单的窗口示例 | 基础窗口创建、事件处理 |
| **control-flow** | 控制流模式 | Wait/WaitUntil/Poll 模式、FPS 显示 |
| **animation** | 动画渲染 | 弹跳球、拖尾效果、颜色循环 |
| **keyboard-mouse** | 输入事件处理 | 绘图应用、鼠标跟踪、键盘输入 |
| **multi-window** | 多窗口管理 | 动态创建窗口、焦点管理 |
| **fullscreen** | 全屏模式 | 全屏切换、窗口装饰、最大化状态 |

### 快速开始

```bash
# 进入任意示例目录
cd examples/<示例名称>

# 安装依赖
npm install

# 运行示例
npm start
```

### 示例详情

#### 1. hello-world
最简单的窗口示例，非常适合初学者。

```bash
cd examples/hello-world
npm install
npm start
```

**操作**: 按 ESC 退出

#### 2. control-flow
演示在不同控制流模式之间切换，并实时显示 FPS。

```bash
cd examples/control-flow
npm install
npm start
```

**操作**: 
- 1/2/3: 切换控制流模式
- R: 切换重绘
- ESC: 退出

#### 3. animation
流畅的动画渲染，包含弹跳球物理效果和拖尾效果。

```bash
cd examples/animation
npm install
npm start
```

**操作**:
- R: 切换动画
- SPACE: 更改背景色
- ESC: 退出

#### 4. keyboard-mouse
完整的输入处理示例，实现了一个简单的绘图应用。

```bash
cd examples/keyboard-mouse
npm install
npm start
```

**操作**:
- 鼠标: 移动查看坐标
- 左键: 绘制点
- 右键: 擦除附近的点
- 滚轮: 调整画笔大小
- C: 清空画布
- H: 切换光标可见性
- N: 更改颜色
- ESC: 退出

#### 5. multi-window
演示动态创建和管理多个窗口。

```bash
cd examples/multi-window
npm install
npm start
```

**操作**:
- N: 创建新窗口
- C: 关闭当前窗口
- 1-9: 切换到指定窗口
- ESC: 退出

#### 6. fullscreen
展示全屏模式切换和窗口属性控制。

```bash
cd examples/fullscreen
npm install
npm start
```

**操作**:
- F: 切换全屏
- D: 切换窗口装饰
- M: 最大化/还原
- V: 切换可见性
- ESC: 退出全屏或应用

### 学习路径

我们推荐按以下顺序学习：

1. **hello-world** - 理解基本概念
2. **control-flow** - 学习控制流模式
3. **animation** - 掌握渲染技术
4. **keyboard-mouse** - 处理用户输入
5. **multi-window** - 管理多个窗口
6. **fullscreen** - 高级窗口控制

更多详情请查看 [examples/README.md](examples/README.md)。

## 11. 许可证

MIT License - 详见 [LICENSE](LICENSE)

## 12. 贡献

欢迎贡献！请随时提交 issue 和 pull request。

### 开发环境设置

```bash
# 克隆仓库
git clone https://github.com/jerry4718/napi-winit.git
cd napi-winit

# 安装依赖
pnpm install

# 构建项目
pnpm build

# 运行测试
pnpm test
```

### 为不同平台构建

```bash
# 为特定平台构建
pnpm build --target x86_64-unknown-linux-gnu

# 构建调试版本
pnpm build:debug
```

## 13. 相关链接

- **GitHub**: https://github.com/jerry4718/napi-winit
- **npm**: https://www.npmjs.com/package/@ylcc/napi-winit
- **winit**: https://github.com/rust-windowing/winit
- **NAPI-RS**: https://napi.rs/

## 14. 致谢

本项目基于以下优秀项目构建：

- [winit](https://github.com/rust-windowing/winit) - 跨平台窗口创建和管理
- [NAPI-RS](https://napi.rs/) - Node.js 原生插件框架
- [softbuffer](https://github.com/rust-windowing/softbuffer) - 软件缓冲区渲染
