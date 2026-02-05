# napi-winit

[![npm version](https://img.shields.io/npm/v/@ylcc/napi-winit.svg)](https://www.npmjs.com/package/@ylcc/napi-winit)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> Windowing for Node.js and Deno with native performance

## 1. Project Introduction

**napi-winit** is a high-performance native window management library for Node.js and Deno, built on Rust's [winit](https://github.com/rust-windowing/winit) library and NAPI-RS bindings. It provides a complete solution for creating native windows, handling window events, and managing application lifecycles with minimal overhead.

### 1.1 Key Features

- 🚀 **Native Performance**: Built with Rust, providing near-native execution speed
- 🪟 **Window Management**: Create and manage native windows with full control
- 🎮 **Event Handling**: Comprehensive keyboard, mouse, and window event support
- ⚡ **Flexible Control Flow**: Support for Wait, WaitUntil, and Poll modes
- 🎨 **Software Rendering**: Built-in soft surface rendering with [softbuffer](https://github.com/rust-windowing/softbuffer)
- 🌍 **Cross-platform**: Windows, macOS, Linux, FreeBSD support (10+ architectures)
- 📘 **TypeScript Support**: Full TypeScript type definitions included
- 🦕 **Deno Compatible**: Works seamlessly with both Node.js and Deno

## 2. Installation

### 2.1 Node.js

Install with npm:

```bash
npm install @ylcc/napi-winit
```

Install with yarn:

```bash
yarn add @ylcc/napi-winit
```

Install with pnpm:

```bash
pnpm add @ylcc/napi-winit
```

### 2.2 Deno

```javascript
import { Application, EventLoop, Window, WindowAttributes } from 'npm:@ylcc/napi-winit';
```

## 3. Quick Start

Here's a minimal example showing how to create a window and handle basic events:

```typescript
import { Application, EventLoop, WindowAttributes } from '@ylcc/napi-winit';

// Create event loop
const eventLoop = new EventLoop();

// Configure window attributes
const attrs = new WindowAttributes()
    .withActive(true)
    .withResizable(true)
    .withInnerSize({ type: 'Logical', width: 800, height: 600 })
    .withTitle('Hello napi-winit');

// Create application with event handlers
const app = Application.withSyncRef({
    onResumed: (eventLoop) => {
        // Create window when application is ready
        const window = eventLoop.createWindow(attrs);
        console.log('Window created');
    },
    onWindowEvent: (eventLoop, windowId, event) => {
        // Handle window events
        if (event.type === 'CloseRequested') {
            console.log('User requested to close window');
            eventLoop.exit();
        } else if (event.type === 'KeyboardInput') {
            const { state, text, logicalKey } = event.event;
            console.log(`Key: ${text}, state: ${state}`);
            
            // Exit on Escape key
            if (logicalKey.type === 'Named' && logicalKey.name === 'Escape') {
                eventLoop.exit();
            }
        }
    },
    onAboutToWait: (eventLoop) => {
        // Set control flow mode
        eventLoop.setControlFlow({ type: 'Wait' });
    }
});

// Run event loop
async function run() {
    while (true) {
        const status = eventLoop.pumpAppEvents(0, app);
        if (status.type === 'Exit') {
            console.log(`Exiting with code: ${status.code}`);
            break;
        }
        // Control event loop frequency (60 FPS)
        await new Promise(resolve => setTimeout(resolve, 1000 / 60));
    }
}

run().catch(console.error);
```

## 4. API Reference

### 4.1 EventLoop

The event loop is the core of the window system, responsible for handling all window events.

```typescript
const eventLoop = new EventLoop();

// Process application events
const status = eventLoop.pumpAppEvents(0, app);

// Set control flow
eventLoop.setControlFlow(controlFlow);

// Create window
const window = eventLoop.createWindow(attributes);
```

### 4.2 WindowAttributes

Used to configure window properties.

```typescript
const attrs = new WindowAttributes()
    .withActive(true) // Whether the window is active
    .withResizable(true) // Whether the window is resizable
    .withInnerSize({ type: 'Logical', width: 800, height: 600 }) // Window inner size
    .withPosition({ type: 'Logical', x: 100, y: 100 }) // Window position
    .withTitle('Window Title') // Window title
    .withTransparent(false) // Whether the window is transparent
    .withFullscreen(null); // Whether to be fullscreen
```

### 4.3 Window

Window instance methods for controlling and querying window state:

```typescript
// Redraw and presentation
window.requestRedraw(); // Request a redraw event
window.prePresentNotify(); // Notify before presenting (required for some platforms)

// Size and position
const innerSize = window.innerSize(); // Get current inner size
const outerSize = window.outerSize(); // Get outer size (including decorations)
const innerPos = window.innerPosition(); // Get inner position
const outerPos = window.outerPosition(); // Get outer position

// Request size change (returns actual size or null if not supported)
const actualSize = window.requestInnerSize({ type: 'Logical', width: 1024, height: 768 });

// Set position
window.setOuterPosition({ type: 'Logical', x: 100, y: 100 });

// Size constraints
window.setMinInnerSize({ type: 'Logical', width: 400, height: 300 });
window.setMaxInnerSize({ type: 'Logical', width: 1920, height: 1080 });

// Window properties
window.setTitle('New Title');
window.setVisible(true);
window.setResizable(false);
window.setDecorations(true);

// Display properties
const scaleFactor = window.scaleFactor(); // Get DPI scale factor
const id = window.id(); // Get unique window ID

// Cursor control
import { Cursor, CursorIcon } from '@ylcc/napi-winit';
window.setCursor(Cursor.fromIcon('Hand'));
window.setCursorVisible(false);

// Fullscreen
window.setFullscreen({
    type: 'Borderless',
    monitor: null // null = current monitor
});
window.setFullscreen(null); // Exit fullscreen

// Focus and attention
window.focus();
window.requestUserAttention('Informational'); // or 'Critical'

// Advanced
window.resetDeadKeys(); // Reset dead key state
window.setImeAllowed(true); // Allow IME input
```

### 4.4 Application

Application class for handling application-level events.

```typescript
const app = Application.withSyncRef({
    onNewEvents: (eventLoop, cause) => {
        // Called when new events arrive
    },
    onResumed: (eventLoop) => {
        // Called when the application resumes, usually where windows are created
    },
    onWindowEvent: (eventLoop, windowId, event) => {
        // Handle window events
    },
    onAboutToWait: async (eventLoop) => {
        // Called when the event loop is about to wait, used to set control flow
    }
});
```

### 4.5 ControlFlow

Controls the behavior of the event loop:

```typescript
// Wait Mode - Wait indefinitely for the next event (most efficient)
eventLoop.setControlFlow({ type: 'Wait' });

// WaitUntil Mode - Wait for specified time or next event
import { Instant, Duration } from '@ylcc/napi-winit';

// Wait until specific instant
eventLoop.setControlFlow({ 
    type: 'WaitUntil', 
    timeout: Instant.afterMillis(16) // ~60 FPS
});

// Wait for duration
eventLoop.setControlFlow({ 
    type: 'WaitUntil', 
    timeout: Instant.afterNanos(1_000_000 / 120) // ~120 FPS
});

// Poll Mode - Process all events immediately without waiting
eventLoop.setControlFlow({ type: 'Poll' });

// Exit the event loop
eventLoop.exit();

// Check if exiting
if (eventLoop.exiting()) {
    console.log('Event loop is exiting');
}

// Get current control flow
const currentFlow = eventLoop.controlFlow();
console.log(currentFlow.type); // 'Wait' | 'WaitUntil' | 'Poll'
```

### 4.6 Extra Features

Additional utilities for enhanced functionality:

```typescript
import { Extra, Duration, Instant } from '@ylcc/napi-winit';

// Buffer Surface Rendering
const surface = new Extra.BufferSurface(window);

// Method 1: Draw with writer callback
surface.presentWithWriter((view, width, height) => {
    // view is Uint32Array - each element is 0xAARRGGBB
    for (let i = 0; i < view.length; i++) {
        view[i] = 0xFF00FF00; // Green color
    }
});

// Method 2: Present with pre-filled buffer
const buffer = new Uint32Array(width * height);
buffer.fill(0xFFFF0000); // Red color
surface.presentWithTyped(buffer);

// Async Sleep (Tokio-based)
await Extra.tokioSleep(Duration.fromMillis(100));

// Time-based execution
const instant = Instant.now();
const future = Instant.afterMillis(1000);

// Duration operations
const duration = Duration.fromSecs(1);
const doubled = Duration.mul(duration, 2);

// Thread Pool Execution
const pool = Extra.ThreadPool.default();
pool.execute(() => {
    console.log('Running in thread pool');
});

// Get raw window handles (for custom rendering APIs)
const options = Extra.getRwh05Options(window);
console.log(options.system); // 'win32' | 'cocoa' | 'x11' | 'wayland'
```

## 5. Event Handling

### 5.1 Common Window Events

Handle various window events in the `onWindowEvent` callback:

```typescript
onWindowEvent: (eventLoop, windowId, event) => {
    switch (event.type) {
        case 'CloseRequested':
            // User requested to close window (e.g., clicked X button)
            console.log('Close requested');
            eventLoop.exit();
            break;

        case 'RedrawRequested':
            // Window needs to be redrawn
            // Perform rendering here
            break;

        case 'Resized':
            // Window size changed
            const { width, height } = event.size;
            console.log(`Resized to ${width}x${height}`);
            break;

        case 'Moved':
            // Window position changed
            const { x, y } = event.position;
            console.log(`Moved to (${x}, ${y})`);
            break;

        case 'Focused':
            // Window gained or lost focus
            console.log(`Focus: ${event.focused}`);
            break;

        case 'KeyboardInput':
            // Keyboard input event
            const keyEvent = event.event;
            const { state, logicalKey, physicalKey, text, repeat } = keyEvent;
            
            // Handle character keys
            if (logicalKey.type === 'Character') {
                console.log(`Character: ${logicalKey.ch}, state: ${state}`);
            }
            
            // Handle named keys (Escape, Enter, etc.)
            if (logicalKey.type === 'Named') {
                console.log(`Named key: ${logicalKey.name}, state: ${state}`);
            }
            break;

        case 'ModifiersChanged':
            // Modifier keys state changed (Shift, Ctrl, Alt, Super)
            const mods = event.modifiers.state();
            console.log({
                shift: mods.hasShift(),
                ctrl: mods.hasControl(),
                alt: mods.hasAlt(),
                super: mods.hasSuper()
            });
            break;

        case 'MouseInput':
            // Mouse button event
            const { button, state: btnState } = event.event;
            console.log(`Mouse button ${button}: ${btnState}`);
            break;

        case 'MouseWheel':
            // Mouse wheel event
            const { deltaX, deltaY } = event.delta;
            console.log(`Wheel: (${deltaX}, ${deltaY})`);
            break;

        case 'CursorMoved':
            // Cursor position changed
            const position = event.position;
            console.log(`Cursor: (${position.x}, ${position.y})`);
            break;

        case 'CursorEntered':
            // Cursor entered window
            console.log('Cursor entered');
            break;

        case 'CursorLeft':
            // Cursor left window
            console.log('Cursor left');
            break;

        case 'ScaleFactorChanged':
            // DPI scale factor changed
            const { scaleFactor, innerSizeWriter } = event;
            console.log(`New scale factor: ${scaleFactor}`);
            break;

        case 'ThemeChanged':
            // System theme changed
            console.log(`Theme: ${event.theme}`); // 'Light' or 'Dark'
            break;

        case 'Destroyed':
            // Window was destroyed
            console.log('Window destroyed');
            break;
    }
}
```

## 6. Control Flow Modes

napi-winit supports three control flow modes to optimize performance and responsiveness:

### Wait Mode (Recommended for event-driven apps)

Waits indefinitely for the next event. Most CPU-efficient option.

```typescript
eventLoop.setControlFlow({ type: 'Wait' });
```

**Use when:**
- Your app is purely event-driven
- No continuous animations or updates needed
- Battery life is important (mobile/laptop scenarios)

### WaitUntil Mode (Best for animations)

Waits until a specified time or the next event arrives.

```typescript
import { Instant } from '@ylcc/napi-winit';

// Wait for 16ms (~60 FPS)
eventLoop.setControlFlow({ 
    type: 'WaitUntil', 
    timeout: Instant.afterMillis(16) 
});

// Wait for 8.33ms (~120 FPS)
eventLoop.setControlFlow({ 
    type: 'WaitUntil', 
    timeout: Instant.afterNanos(1_000_000 / 120) 
});
```

**Use when:**
- Running animations or games
- Need consistent frame timing
- Want precise control over update frequency

### Poll Mode (High-frequency updates)

Processes all events immediately without waiting.

```typescript
import { Extra, Duration } from '@ylcc/napi-winit';

onAboutToWait: async (eventLoop) => {
    // Add small delay to prevent 100% CPU usage
    await Extra.tokioSleep(Duration.fromMillis(1));
    eventLoop.setControlFlow({ type: 'Poll' });
}
```

**Use when:**
- Maximum responsiveness is critical
- Processing intensive real-time data
- CPU usage is not a concern

### Mode Comparison

| Mode | CPU Usage | Latency | Best For |
|------|-----------|---------|----------|
| **Wait** | Minimal | Low | Event-driven apps |
| **WaitUntil** | Moderate | Predictable | Animations, games |
| **Poll** | High | Minimal | Real-time processing |

## 7. Advanced Examples

### 7.1 Control Flow Example

The following example shows how to switch between different control flow modes:

```typescript
import { Application, EventLoop, Window, WindowAttributes, ControlFlow, Timeout } from '@ylcc/napi-winit';
import { Extra } from '@ylcc/napi-winit';

const eventLoop = new EventLoop();

const attrs = new WindowAttributes()
    .withInnerSize({ type: 'Logical', width: 800, height: 600 })
    .withTitle('Control Flow Example');

let window: Window;
let mode: ControlFlow['type'] = 'Wait';

const app = Application.withSyncRef({
    onResumed: (eventLoop) => {
        window = eventLoop.createWindow(attrs);
    },
    onWindowEvent: (eventLoop, windowId, event) => {
        if (event.type === 'KeyboardInput') {
            const { logicalKey, state } = event.event;
            if (logicalKey.type === 'Character' && state === 'Released') {
                // Press 1, 2, 3 to switch control flow modes
                if (logicalKey.ch === '1') {
                    mode = 'Wait';
                    console.log('Switched to Wait mode');
                } else if (logicalKey.ch === '2') {
                    mode = 'WaitUntil';
                    console.log('Switched to WaitUntil mode');
                } else if (logicalKey.ch === '3') {
                    mode = 'Poll';
                    console.log('Switched to Poll mode');
                }
            }
        }
    },
    onAboutToWait: async (eventLoop) => {
        // Set control flow according to current mode
        if (mode === 'Wait') {
            eventLoop.setControlFlow({ type: 'Wait' });
        } else if (mode === 'WaitUntil') {
            eventLoop.setControlFlow({ type: 'WaitUntil', timeout: Timeout.fromMillis(16) });
        } else if (mode === 'Poll') {
            await Extra.tokioSleep(Timeout.fromMillis(10));
            eventLoop.setControlFlow({ type: 'Poll' });
        }
    }
});

async function run() {
    while (true) {
        const status = eventLoop.pumpAppEvents(0, app);
        if (status.type === 'Exit') {
            console.log(`Exit, code: ${status.code}`);
            break;
        }
        await new Promise(resolve => setTimeout(resolve, 1000 / 60));
    }
}

run().catch(console.error);
```

### 7.2 Animated Rendering with BufferSurface

This example shows smooth animation using software rendering:

```typescript
import { Application, EventLoop, WindowAttributes, Window, Instant, Extra } from '@ylcc/napi-winit';

const eventLoop = new EventLoop();

const attrs = new WindowAttributes()
    .withInnerSize({ type: 'Logical', width: 800, height: 600 })
    .withTitle('Animation Example - Press R to toggle redraw');

let window: Window;
let surface: Extra.BufferSurface;
let frameCount = 0;
let requestRedraw = false;

// Animation state
let rectangleX = 0;
let velocityX = 2;

const app = Application.withSyncRef({
    onResumed: (eventLoop) => {
        window = eventLoop.createWindow(attrs);
        surface = new Extra.BufferSurface(window);
        // Request initial redraw
        window.requestRedraw();
        console.log('Press R to toggle animation, ESC to exit');
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
                    console.log(`Animation: ${requestRedraw ? 'ON' : 'OFF'}`);
                }
            }
            
            if (state === 'Released' && logicalKey.type === 'Named') {
                if (logicalKey.name === 'Escape') {
                    eventLoop.exit();
                }
            }
        }
        
        if (event.type === 'RedrawRequested') {
            // Notify platform we're about to present
            window.prePresentNotify();
            
            // Render frame
            surface.presentWithWriter((view, width, height) => {
                frameCount++;
                
                // Clear to dark background (ARGB format: 0xAARRGGBB)
                view.fill(0xFF101010);
                
                // Update rectangle position
                rectangleX += velocityX;
                if (rectangleX <= 0 || rectangleX >= width - 100) {
                    velocityX *= -1; // Bounce
                }
                
                // Draw animated rectangle
                const rectWidth = 100;
                const rectHeight = 50;
                const y = Math.floor(
                    Math.sin(frameCount * 0.05) * 100 + height / 2 - rectHeight / 2
                );
                
                // Calculate color based on frame count
                const hue = (frameCount * 2) % 360;
                const color = hslToRgb(hue, 100, 50);
                
                // Draw rectangle
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
                
                // Draw FPS counter (simple dot pattern)
                const fps = frameCount % 60;
                for (let i = 0; i < fps; i++) {
                    const idx = i * 10;
                    if (idx < width) {
                        view[idx] = 0xFFFFFFFF; // White dot
                    }
                }
            });
            
            // Request next frame if animation is enabled
            if (requestRedraw) {
                window.requestRedraw();
            }
        }
    },
    
    onAboutToWait: (eventLoop) => {
        // Use Wait mode to only redraw when needed
        eventLoop.setControlFlow({ type: 'Wait' });
    }
});

// Helper: Convert HSL to RGB (returns 0xAARRGGBB format)
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
        const status = eventLoop.pumpAppEvents(0, app);
        if (status.type === 'Exit') {
            console.log(`Exited with code: ${status.code}`);
            break;
        }
        await new Promise(resolve => setTimeout(resolve, 1000 / 60)); // 60 FPS
    }
}

run().catch(console.error);
```

## 8. Platform Support

napi-winit provides pre-built binaries for the following platforms:

| Platform | Architectures | Notes |
|----------|--------------|-------|
| **Windows** | x86_64, i686, aarch64 | Full support |
| **macOS** | x86_64 (Intel), aarch64 (Apple Silicon) | Full support |
| **Linux (GNU)** | x86_64, aarch64, armv7 | Full support |
| **Linux (musl)** | x86_64, aarch64 | Alpine Linux compatible |
| **FreeBSD** | x86_64 | Full support |

### System Requirements

- **Node.js**: >= 10.0.0
- **Deno**: Latest stable version
- **Operating System**: Windows 7+, macOS 10.12+, Linux (any modern distribution)

### Display Server Support (Linux)

- ✅ X11 (with dynamic linking via x11-dlopen)
- ✅ Wayland (with dynamic linking via wayland-dlopen)

The library automatically detects the available display server at runtime.

## 9. Best Practices

### 9.1 Event Loop Management

**Control event loop frequency** to balance CPU usage and responsiveness:

```typescript
// Good: Control pump frequency
async function run() {
    while (true) {
        const status = eventLoop.pumpAppEvents(0, app);
        if (status.type === 'Exit') break;
        await new Promise(resolve => setTimeout(resolve, 1000 / 60)); // 60 FPS
    }
}

// Also good: Use Wait mode to let the OS wake your app
onAboutToWait: (eventLoop) => {
    eventLoop.setControlFlow({ type: 'Wait' }); // Most efficient
}
```

### 9.2 Resource Management

Always properly clean up resources:

```typescript
let window: Window | null = null;
let surface: Extra.BufferSurface | null = null;

const app = Application.withSyncRef({
    onResumed: (eventLoop) => {
        window = eventLoop.createWindow(attrs);
        surface = new Extra.BufferSurface(window);
    },
    onWindowEvent: (eventLoop, windowId, event) => {
        if (event.type === 'CloseRequested') {
            // Cleanup before exit
            surface = null;
            window = null;
            eventLoop.exit();
        }
    }
});
```

### 9.3 Performance Tips

1. **Use Wait mode when possible**: Most CPU-efficient for event-driven apps
2. **Batch redraws**: Only call `requestRedraw()` when necessary
3. **Profile your rendering**: Use `prePresentNotify()` before presenting frames
4. **Consider WaitUntil for animations**: Provides consistent frame timing

### 9.4 Cross-platform Considerations

- **Window decorations**: Behavior may vary across platforms
- **DPI scaling**: Use logical sizes and handle scale factor changes
- **Keyboard layouts**: Test with different keyboard configurations
- **Fullscreen modes**: Check platform-specific fullscreen behavior

## 10. Example Projects

The `examples/` directory contains various example projects demonstrating different features and use cases:

### Available Examples

| Example | Description | Key Features |
|---------|-------------|--------------|
| **hello-world** | Simplest window example | Basic window creation, event handling |
| **control-flow** | Control flow modes | Wait/WaitUntil/Poll modes, FPS display |
| **animation** | Animated rendering | Bouncing ball, trail effects, color cycling |
| **keyboard-mouse** | Input event handling | Drawing app, mouse tracking, keyboard input |
| **multi-window** | Multiple windows | Dynamic window creation, focus management |
| **fullscreen** | Fullscreen modes | Fullscreen toggle, decorations, maximized state |

### Quick Start

```bash
# Navigate to any example
cd examples/<example-name>

# Install dependencies
npm install

# Run the example
npm start
```

### Example Details

#### 1. hello-world
The simplest possible window example, perfect for beginners.

```bash
cd examples/hello-world
npm install
npm start
```

**Controls**: ESC to exit

#### 2. control-flow
Demonstrates switching between different control flow modes with real-time FPS display.

```bash
cd examples/control-flow
npm install
npm start
```

**Controls**: 
- 1/2/3: Switch control flow modes
- R: Toggle redraw
- ESC: Exit

#### 3. animation
Smooth animation rendering with bouncing ball physics and trail effects.

```bash
cd examples/animation
npm install
npm start
```

**Controls**:
- R: Toggle animation
- SPACE: Change background color
- ESC: Exit

#### 4. keyboard-mouse
Complete input handling example implementing a simple drawing application.

```bash
cd examples/keyboard-mouse
npm install
npm start
```

**Controls**:
- Mouse: Move to see coordinates
- Left Click: Draw points
- Right Click: Erase nearby points
- Scroll: Adjust brush size
- C: Clear canvas
- H: Toggle cursor visibility
- N: Change color
- ESC: Exit

#### 5. multi-window
Demonstrates creating and managing multiple windows dynamically.

```bash
cd examples/multi-window
npm install
npm start
```

**Controls**:
- N: Create new window
- C: Close current window
- 1-9: Switch to window by number
- ESC: Exit

#### 6. fullscreen
Shows fullscreen mode switching and window property control.

```bash
cd examples/fullscreen
npm install
npm start
```

**Controls**:
- F: Toggle fullscreen
- D: Toggle window decorations
- M: Maximize/restore
- V: Toggle visibility
- ESC: Exit fullscreen or app

### Learning Path

We recommend following this order:

1. **hello-world** - Understand basic concepts
2. **control-flow** - Learn control flow modes
3. **animation** - Master rendering techniques
4. **keyboard-mouse** - Handle user input
5. **multi-window** - Manage multiple windows
6. **fullscreen** - Advanced window control

For more details, see [examples/README.md](examples/README.md).

## 11. License

MIT License - see [LICENSE](LICENSE) for details

## 12. Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/jerry4718/napi-winit.git
cd napi-winit

# Install dependencies
pnpm install

# Build the project
pnpm build

# Run tests
pnpm test
```

### Building for Different Platforms

```bash
# Build for specific platform
pnpm build --target x86_64-unknown-linux-gnu

# Build debug version
pnpm build:debug
```

## 13. Links

- **GitHub**: https://github.com/jerry4718/napi-winit
- **npm**: https://www.npmjs.com/package/@ylcc/napi-winit
- **winit**: https://github.com/rust-windowing/winit
- **NAPI-RS**: https://napi.rs/

## 14. Acknowledgments

This project is built upon:

- [winit](https://github.com/rust-windowing/winit) - Cross-platform window creation and management
- [NAPI-RS](https://napi.rs/) - Node.js Native Addon framework
- [softbuffer](https://github.com/rust-windowing/softbuffer) - Software buffer rendering
