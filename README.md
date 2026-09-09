# napi-winit

[![npm version](https://img.shields.io/npm/v/@ylcc/napi-winit.svg)](https://www.npmjs.com/package/@ylcc/napi-winit)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[简体中文](README-zh.md)

> Windowing for Node.js and Deno with native performance

**napi-winit** is a high-performance native window management library for Node.js and Deno, built on Rust's [winit](https://github.com/rust-windowing/winit) library and NAPI-RS bindings. It provides a complete solution for creating native windows, handling window events, and managing application lifecycles with minimal overhead.

- 🚀 **Native Performance**: Built with Rust, providing near-native execution speed
- 🪟 **Window Management**: Create and manage native windows with full control
- 🎮 **Event Handling**: Comprehensive keyboard, mouse, and window event support
- ⚡ **Flexible Control Flow**: Support for Wait, WaitUntil, and Poll modes
- 🎨 **Software Rendering**: Built-in soft surface rendering with [softbuffer](https://github.com/rust-windowing/softbuffer)
- 🌍 **Cross-platform**: Windows, macOS, Linux, FreeBSD support (10+ architectures)
- 📘 **TypeScript Support**: Full TypeScript type definitions included
- 🦕 **Deno Compatible**: Works seamlessly with both Node.js and Deno

## Installation

### Node.js

```bash
npm install @ylcc/napi-winit
```

### Deno

```javascript
import { Application, EventLoop, Window, WindowAttributes } from 'npm:@ylcc/napi-winit';
```

## Quick Start

```typescript
import { Application, EventLoop, WindowAttributes } from '@ylcc/napi-winit';

// Create event loop
const eventLoop = new EventLoop();

// Configure window attributes
const attrs = new WindowAttributes()
    .withActive(true)
    .withResizable(true)
    .withSurfaceSize({ type: 'Logical', width: 800, height: 600 })
    .withTitle('Hello napi-winit');

// Create application with event handlers
const app = Application.withOptions({
    onCanCreateSurfaces: (activeEventLoop) => {
        // Create window when application is ready
        const window = activeEventLoop.createWindow(attrs);
        console.log('Window created');
    },
    onWindowEvent: (activeEventLoop, windowId, event) => {
        // Handle window events
        if (event.type === 'CloseRequested') {
            console.log('User requested to close window');
            activeEventLoop.exit();
        } else if (event.type === 'KeyboardInput') {
            const { state, text, logicalKey } = event.event;
            console.log(`Key: ${text}, state: ${state}`);

            // Exit on Escape key
            if (logicalKey.type === 'Named' && logicalKey.name === 'Escape') {
                activeEventLoop.exit();
            }
        }
    },
    onAboutToWait: (activeEventLoop) => {
        // Set control flow mode
        activeEventLoop.setControlFlow({ type: 'Wait' });
    }
});

// Run event loop
async function run() {
    while (true) {
        const status = eventLoop.pumpAppEvents(app);
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

## Documentation

| Document | Contents |
|----------|----------|
| [API Reference](docs/api.md) | EventLoop, ActiveEventLoop, Application, WindowAttributes, Window, ControlFlow, Extra |
| [Event Handling](docs/events.md) | Window events: keyboard, pointer, resize, focus, and more |
| [Control Flow Modes](docs/control-flow.md) | Wait / WaitUntil / Poll modes and when to use each |
| [Best Practices](docs/best-practices.md) | Event loop management, resource cleanup, performance tips |

## Examples

Complete runnable programs live in [examples](examples/) — each directory has a `start.ts` with the full program.

| Example | Description |
|---------|-------------|
| [hello-world](examples/hello-world) | Simplest window example |
| [control-flow](examples/control-flow) | Wait / WaitUntil / Poll modes, FPS display |
| [animation](examples/animation) | Animated rendering: bouncing ball, trail effects |
| [game-of-life](examples/game-of-life) | Conway's Game of Life with vgpu compute |
| [keyboard-mouse](examples/keyboard-mouse) | Input event handling, drawing app |
| [multi-window](examples/multi-window) | Multiple windows, focus management |
| [fullscreen](examples/fullscreen) | Fullscreen modes and decorations |

Run any example:

```bash
cd examples/<example-name>
npm install
npm start
```

## Platform Support

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

## Contributing

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

## License

MIT License - see [LICENSE](LICENSE) for details

## Links

- **GitHub**: https://github.com/jerry4718/napi-winit
- **npm**: https://www.npmjs.com/package/@ylcc/napi-winit
- **winit**: https://github.com/rust-windowing/winit
- **NAPI-RS**: https://napi.rs/

## Acknowledgments

This project is built upon:

- [winit](https://github.com/rust-windowing/winit) - Cross-platform window creation and management
- [NAPI-RS](https://napi.rs/) - Node.js Native Addon framework
- [softbuffer](https://github.com/rust-windowing/softbuffer) - Software buffer rendering
