# API 参考

## EventLoop

由你自己创建的入口对象。它驱动应用运行：把事件泵入 `Application` 并执行。

```typescript
import { Application, EventLoop } from '@ylcc/napi-winit';

const eventLoop = new EventLoop();
const app = Application.withOptions({ /* ... */ });

// 运行直到退出
eventLoop.runApp(app);

// 或手动泵事件（例如穿插异步任务）
const status = eventLoop.pumpAppEvents(app);
if (status.type === 'Exit') {
    console.log(`Exiting with code: ${status.code}`);
}
```

| 方法 | 说明 |
|------|------|
| `new EventLoop()` | 创建事件循环 |
| `runApp(app)` | 运行应用直到退出 |
| `runAppOnDemand(app)` | 按需运行应用 |
| `pumpAppEvents(app, timeout?): PumpStatus` | 泵一次事件；`timeout: Duration \| null` |

## ActiveEventLoop

传递给每个 `Application` 回调的 `eventLoop` 参数。窗口创建与控制流都在它上面。

```typescript
onCanCreateSurfaces: (activeEventLoop) => {
    const window = activeEventLoop.createWindow(attrs);
},
onAboutToWait: (activeEventLoop) => {
    activeEventLoop.setControlFlow({ type: 'Wait' });
}
```

| 方法 | 说明 |
|------|------|
| `createWindow(attrs): Window` | 创建窗口 |
| `setControlFlow(flow)` | 设置 `ControlFlow` 模式 |
| `controlFlow(): ControlFlow` | 获取当前模式 |
| `exit()` | 退出事件循环 |
| `exiting(): boolean` | 是否已请求退出 |
| `availableMonitors(): MonitorHandle[]` | 列出显示器 |
| `primaryMonitor(): MonitorHandle \| null` | 主显示器 |
| `systemTheme(): Theme \| null` | 系统主题 |
| `listenDeviceEvents(allowed)` | 过滤设备事件 |
| `ownedDisplayHandle(): OwnedDisplayHandle` | 显示句柄 |

## Application

承载事件回调的对象。用 `Application.withOptions(callbacks)` 创建，传给 `eventLoop.runApp(app)` 或 `eventLoop.pumpAppEvents(app)`。

```typescript
const app = Application.withOptions({
    onCanCreateSurfaces: (activeEventLoop) => {
        // 必填。应用恢复时调用，通常在这里创建窗口
    },
    onWindowEvent: (activeEventLoop, windowId, event) => {
        // 必填。处理窗口事件
    },
    onAboutToWait: (activeEventLoop) => {
        // 事件循环即将等待时调用，在这里设置控制流
    },
    onNewEvents: (activeEventLoop, cause) => {},
    onResumed: (activeEventLoop) => {},
    onSuspended: (activeEventLoop) => {},
    onDeviceEvent: (activeEventLoop, deviceId, event) => {},
    onProxyWakeUp: (activeEventLoop) => {},
    onDestroySurfaces: (activeEventLoop) => {},
    onMemoryWarning: (activeEventLoop) => {}
});
```

`onCanCreateSurfaces` 与 `onWindowEvent` 为必填，其余可选。

## WindowAttributes

窗口属性构建器。链式调用 `with*`，然后把结果传给 `activeEventLoop.createWindow(attrs)`。

```typescript
const attrs = new WindowAttributes()
    .withActive(true)            // 创建时是否激活（获得焦点）
    .withResizable(true)
    .withSurfaceSize({ type: 'Logical', width: 800, height: 600 })
    .withPosition({ type: 'Logical', x: 100, y: 100 })
    .withTitle('Hello napi-winit')
    .withDecorations(true)
    .withTransparent(false)
    .withVisible(true)
    .withMaximized(false)
    .withFullscreen(null)
    .withMinSurfaceSize({ type: 'Logical', width: 400, height: 300 })
    .withMaxSurfaceSize({ type: 'Logical', width: 1920, height: 1080 })
    .withBlur(false)
    .withTheme(null)
    .withWindowLevel(null)
    .withCursor(null)
    .withEnabledButtons(null)
    .withContentProtected(false)
    .withSurfaceResizeIncrements(null);
```

## Window

单个窗口实例，从 `activeEventLoop.createWindow(attrs)` 获得。方法用于控制和查询窗口状态。

```typescript
// 重绘
window.requestRedraw();        // 请求重绘事件
window.prePresentNotify();     // 呈现前调用（部分平台必需）

// 尺寸与位置
window.surfaceSize(): Size
window.outerSize(): Size
window.surfacePosition(): Position
window.outerPosition(): Position
window.requestSurfaceSize(size): Size | null   // 不支持时返回 null
window.setOuterPosition(position)
window.setMinSurfaceSize(minSize?)
window.setMaxSurfaceSize(maxSize?)

// 窗口属性
window.title(): string
window.setTitle(title)
window.setVisible(visible)
window.setResizable(resizable)
window.setDecorations(decorations)
window.setTransparent(transparent)
window.setBlur(blur)
window.setMaximized(maximized)
window.setMinimized(minimized)
window.setWindowIcon(icon?)
window.setWindowLevel(level)
window.setEnabledButtons(buttons)
window.setContentProtected(protected)

// 全屏
window.setFullscreen(fullscreen?)   // 例如 { type: 'Borderless', monitor: null }；传 null 退出全屏
window.fullscreen(): Fullscreen | null

// 焦点
window.focusWindow()
window.hasFocus(): boolean
window.requestUserAttention(type?)  // 'Informational' | 'Critical'；传 null 取消

// 光标
window.setCursor(cursor)
window.setCursorVisible(visible)
window.setCursorPosition(position)
window.setCursorGrab(mode)
window.setCursorHittest(hittest)

// 显示器
window.currentMonitor(): MonitorHandle | null
window.availableMonitors(): MonitorHandle[]
window.primaryMonitor(): MonitorHandle | null

// 其他
window.id(): WindowId
window.scaleFactor(): number
window.setTheme(theme?)
window.theme(): Theme | null
window.resetDeadKeys()
window.requestImeUpdate(request)
window.dragWindow()
window.dragResizeWindow(direction)
window.showWindowMenu(position)
```

## ControlFlow

告诉事件循环如何等待。通过 `activeEventLoop.setControlFlow(...)` 设置（选型指南见[控制流模式](control-flow.md)）。

```typescript
type ControlFlow =
    | { type: 'Poll' }                        // 持续泵事件；CPU 占用最高
    | { type: 'Wait' }                        // 休眠到下一个事件；CPU 占用最低
    | { type: 'WaitUntil', timeout: Instant } // 休眠到截止时间或下一个事件
```

## Extra

工具命名空间：软件渲染、线程池、定时器、原生窗口句柄。

### BufferSurface

向窗口表面做软件渲染。视图中每个像素为 `0xAARRGGBB`。

```typescript
const surface = new Extra.BufferSurface(window);

// 用 writer 回调绘制一帧
surface.presentWithWriter((view, width, height) => {
    view.fill(0xFF101010);
});

// 呈现预填充的缓冲
const buffer = new Uint32Array(width * height);
surface.presentWithTyped(buffer);

// 可从其他线程调用的变体
surface.presentWithThreadsafeWriter((view, width, height) => {});
```

### ThreadPool

```typescript
const pool = Extra.ThreadPool.default(); // 或 Extra.ThreadPool.main()、new Extra.ThreadPool(n)
pool.execute(() => {
    console.log('Running in thread pool');
});
```

### 定时器

```typescript
await Extra.tokioSleep(Duration.fromMillis(100));
Extra.tokioInterval(Duration.fromMillis(100), () => {});
Extra.tokioCallSpawn(() => {});
Extra.threadInterval(Duration.fromMillis(100), () => {});
```

### 原生窗口句柄

```typescript
const options = Extra.getRwhOptions(window);
console.log(options.system); // 'win32' | 'cocoa' | 'x11' | 'wayland'
```

返回原生窗口/系统句柄的判别联合（形状见 [index.d.ts](../../index.d.ts) 中的 `SurfaceOptions`），用于对接 wgpu、vgpu 等外部渲染器。

## Instant 与 Duration

用于 `WaitUntil` 超时和定时器的时间类型。

```typescript
const now = Instant.now();
const deadline = Instant.afterMillis(16); // 约 60 FPS

const duration = Duration.fromMillis(100);
const halved = Duration.div(duration, 2);
```

`Instant`：`now()`、`afterSecs/Millis/Micros/Nanos()`、`add()`、`sub()`、`durationSince()`。
`Duration`：`fromSecs/Millis/Micros/Nanos()`、`add()`、`sub()`、`mul()`、`div()`。

