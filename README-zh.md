# napi-winit

[![npm version](https://img.shields.io/npm/v/@ylcc/napi-winit.svg)](https://www.npmjs.com/package/@ylcc/napi-winit)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[English](README.md)

> 基于 Rust winit 的 Node.js / Deno 原生窗口库，性能接近原生

**napi-winit** 是面向 Node.js 和 Deno 的高性能原生窗口管理库，基于 Rust 的 [winit](https://github.com/rust-windowing/winit) 库与 NAPI-RS 绑定构建，以极低的开销提供创建原生窗口、处理窗口事件、管理应用生命周期的完整方案。

- 🚀 **原生性能**：基于 Rust 构建，接近原生的执行速度
- 🪟 **窗口管理**：创建并完全掌控原生窗口
- 🎮 **事件处理**：完整的键盘、鼠标与窗口事件支持
- ⚡ **灵活的控制流**：支持 Wait、WaitUntil、Poll 三种模式
- 🎨 **软件渲染**：内置基于 [softbuffer](https://github.com/rust-windowing/softbuffer) 的软渲染表面
- 🌍 **跨平台**：支持 Windows、macOS、Linux、FreeBSD（10+ 架构）
- 📘 **TypeScript 支持**：提供完整 TypeScript 类型定义
- 🦕 **Deno 兼容**：同时适配 Node.js 与 Deno

## 安装

### Node.js

```bash
npm install @ylcc/napi-winit
```

### Deno

```javascript
import { Application, EventLoop, Window, WindowAttributes } from 'npm:@ylcc/napi-winit';
```

## 快速开始

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

// 创建应用并设置事件回调
const app = Application.withOptions({
    onCanCreateSurfaces: (activeEventLoop) => {
        // 应用就绪时创建窗口
        const window = activeEventLoop.createWindow(attrs);
        console.log('Window created');
    },
    onWindowEvent: (activeEventLoop, windowId, event) => {
        // 处理窗口事件
        if (event.type === 'CloseRequested') {
            console.log('User requested to close window');
            activeEventLoop.exit();
        } else if (event.type === 'KeyboardInput') {
            const { state, text, logicalKey } = event.event;
            console.log(`Key: ${text}, state: ${state}`);

            // 按 Esc 退出
            if (logicalKey.type === 'Named' && logicalKey.name === 'Escape') {
                activeEventLoop.exit();
            }
        }
    },
    onAboutToWait: (activeEventLoop) => {
        // 设置控制流模式
        activeEventLoop.setControlFlow({ type: 'Wait' });
    }
});

// 运行事件循环
async function run() {
    while (true) {
        const status = eventLoop.pumpAppEvents(app);
        if (status.type === 'Exit') {
            console.log(`Exiting with code: ${status.code}`);
            break;
        }
        // 控制事件循环频率（60 FPS）
        await new Promise(resolve => setTimeout(resolve, 1000 / 60));
    }
}

run().catch(console.error);
```

## 文档

| 文档 | 内容 |
|------|------|
| [API 参考](docs/zh/api.md) | EventLoop、ActiveEventLoop、Application、WindowAttributes、Window、ControlFlow、Extra |
| [事件处理](docs/zh/events.md) | 窗口事件：键盘、指针、缩放、焦点等 |
| [控制流模式](docs/zh/control-flow.md) | Wait / WaitUntil / Poll 模式及适用场景 |
| [最佳实践](docs/zh/best-practices.md) | 事件循环管理、资源清理、性能建议 |

## 示例

完整可运行的程序位于 [examples](examples/) 目录，每个子目录的 `start.ts` 就是完整程序。

| 示例 | 说明 |
|------|------|
| [hello-world](examples/hello-world) | 最简窗口示例 |
| [control-flow](examples/control-flow) | Wait / WaitUntil / Poll 模式与 FPS 显示 |
| [animation](examples/animation) | 动画渲染：弹跳小球、拖尾效果 |
| [game-of-life](examples/game-of-life) | 基于 vgpu 计算的康威生命游戏 |
| [keyboard-mouse](examples/keyboard-mouse) | 输入事件处理、绘图应用 |
| [multi-window](examples/multi-window) | 多窗口与焦点管理 |
| [fullscreen](examples/fullscreen) | 全屏模式与窗口装饰 |

运行任意示例：

```bash
cd examples/<example-name>
npm install
npm start
```

## 平台支持

napi-winit 为以下平台提供预构建二进制：

| 平台 | 架构 | 说明 |
|------|------|------|
| **Windows** | x86_64, i686, aarch64 | 完整支持 |
| **macOS** | x86_64 (Intel), aarch64 (Apple Silicon) | 完整支持 |
| **Linux (GNU)** | x86_64, aarch64, armv7 | 完整支持 |
| **Linux (musl)** | x86_64, aarch64 | 兼容 Alpine Linux |
| **FreeBSD** | x86_64 | 完整支持 |

### 系统要求

- **Node.js**: >= 10.0.0
- **Deno**: 最新稳定版
- **操作系统**: Windows 7+、macOS 10.12+、Linux（任意现代发行版）

### 显示服务器支持（Linux）

- ✅ X11（通过 x11-dlopen 动态链接）
- ✅ Wayland（通过 wayland-dlopen 动态链接）

库会在运行时自动检测可用的显示服务器。

## 参与贡献

欢迎提交 issue 和 pull request。

### 开发环境

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

## 许可证

MIT License - 详见 [LICENSE](LICENSE)

## 相关链接

- **GitHub**: https://github.com/jerry4718/napi-winit
- **npm**: https://www.npmjs.com/package/@ylcc/napi-winit
- **winit**: https://github.com/rust-windowing/winit
- **NAPI-RS**: https://napi.rs/

## 致谢

本项目基于以下项目构建：

- [winit](https://github.com/rust-windowing/winit) - 跨平台窗口创建与管理
- [NAPI-RS](https://napi.rs/) - Node.js 原生插件框架
- [softbuffer](https://github.com/rust-windowing/softbuffer) - 软件缓冲渲染
