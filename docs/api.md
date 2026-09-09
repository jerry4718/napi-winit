# API Reference

## EventLoop

The entry object you create yourself. It drives the application: pump events into an `Application` and run it.

```typescript
import { Application, EventLoop } from '@ylcc/napi-winit';

const eventLoop = new EventLoop();
const app = Application.withOptions({ /* ... */ });

// Run until exit
eventLoop.runApp(app);

// Or pump events manually (e.g. to interleave async work)
const status = eventLoop.pumpAppEvents(app);
if (status.type === 'Exit') {
    console.log(`Exiting with code: ${status.code}`);
}
```

| Method | Description |
|---|---|
| `new EventLoop()` | Create the event loop |
| `runApp(app)` | Run the application until exit |
| `runAppOnDemand(app)` | Run the application on demand |
| `pumpAppEvents(app, timeout?): PumpStatus` | Pump events once; `timeout: Duration \| null` |

**Do not share an `EventLoop` between multiple `Application`s, and do not create a new one after the previous one has ended.** winit enforces one `EventLoop` per process with a one-shot flag: once created, the flag is never reset, so any later creation fails with `RecreationAttempt` — consistently across platforms.

**Events are consumed once, not broadcast.** If you create multiple `Application`s and pump them in turn, the pump hands the backlogged events — plus any that arrive during the pump — to the target `Application`.

## ActiveEventLoop

The `eventLoop` argument passed to every `Application` callback. Window creation and control flow live here.

**Difference from `EventLoop`**: two views of the same loop, split by lifetime. You create and own the `EventLoop`; it represents the loop while it is idle and dispatching nothing. While a callback runs, the loop is in its running state, and winit only allows window creation, control-flow changes, and exit in that running state — so callbacks receive an `ActiveEventLoop`, valid only for the duration of the callback. Splitting them into two types turns wrong-timing calls into compile-time errors instead of runtime panics.

In practice, `ActiveEventLoop` remains usable outside the synchronous scope of a callback — e.g. captured in a variable, called from an async task. Avoid escaping too far, though: its semantics only hold while the callback runs. Once the loop may have exited or changed state, calls such as `createWindow` have no guaranteed behavior. Prefer finishing async work in the next callback.

```typescript
onCanCreateSurfaces: (activeEventLoop) => {
    const window = activeEventLoop.createWindow(attrs);
},
onAboutToWait: (activeEventLoop) => {
    activeEventLoop.setControlFlow({ type: 'Wait' });
}
```

| Method | Description |
|---|---|
| `createWindow(attrs): Window` | Create a window |
| `setControlFlow(flow)` | Set `ControlFlow` mode |
| `controlFlow(): ControlFlow` | Get current mode |
| `exit()` | Exit the event loop |
| `exiting(): boolean` | Whether exit was requested |
| `availableMonitors(): MonitorHandle[]` | List monitors |
| `primaryMonitor(): MonitorHandle \| null` | Primary monitor |
| `systemTheme(): Theme \| null` | System theme |
| `listenDeviceEvents(allowed)` | Filter device events |
| `ownedDisplayHandle(): OwnedDisplayHandle` | Display handle |

## Application

Holds your event callbacks. Create it with `Application.withOptions(callbacks)` and pass it to `eventLoop.runApp(app)` or `eventLoop.pumpAppEvents(...)`.

```typescript
const app = Application.withOptions({
    onCanCreateSurfaces: (activeEventLoop) => {
        // Required. Called when the application resumes; create windows here
    },
    onWindowEvent: (activeEventLoop, windowId, event) => {
        // Required. Handle window events
    },
    onAboutToWait: (activeEventLoop) => {
        // Called before the loop waits; set control flow here
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

`onCanCreateSurfaces` and `onWindowEvent` are required; the rest are optional. An `EventLoop` pumps one `Application` at a time — see [EventLoop](#eventloop).

## WindowAttributes

Builder for window properties. Chain `with*` calls, then pass the result to `activeEventLoop.createWindow(attrs)`.

```typescript
const attrs = new WindowAttributes()
    .withActive(true)            // Focus the window on creation
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

A single window, obtained from `activeEventLoop.createWindow(attrs)`. Methods control and query its state.

```typescript
// Redraw
window.requestRedraw();        // Request a redraw event
window.prePresentNotify();     // Call before presenting (required on some platforms)

// Size and position
window.surfaceSize(): Size
window.outerSize(): Size
window.surfacePosition(): Position
window.outerPosition(): Position
window.requestSurfaceSize(size): Size | null   // Returns null if unsupported
window.setOuterPosition(position)
window.setMinSurfaceSize(minSize?)
window.setMaxSurfaceSize(maxSize?)

// Properties
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

// Fullscreen
window.setFullscreen(fullscreen?)   // e.g. { type: 'Borderless', monitor: null }; null exits
window.fullscreen(): Fullscreen | null

// Focus
window.focusWindow()
window.hasFocus(): boolean
window.requestUserAttention(type?)  // 'Informational' | 'Critical'; null cancels

// Cursor
window.setCursor(cursor)
window.setCursorVisible(visible)
window.setCursorPosition(position)
window.setCursorGrab(mode)
window.setCursorHittest(hittest)

// Monitors
window.currentMonitor(): MonitorHandle | null
window.availableMonitors(): MonitorHandle[]
window.primaryMonitor(): MonitorHandle | null

// Misc
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

Tells the event loop how to wait. Set it via `activeEventLoop.setControlFlow(...)` (see [Control Flow Modes](control-flow.md) for guidance).

```typescript
type ControlFlow =
    | { type: 'Poll' }                        // Pump continuously; highest CPU
    | { type: 'Wait' }                        // Sleep until the next event; lowest CPU
    | { type: 'WaitUntil', timeout: Instant } // Sleep until a deadline or the next event
```

## Extra

Utility namespace: software rendering, thread pools, timers, and raw window handles.

### BufferSurface

Software rendering into a window's surface. Each pixel of the view is `0xAARRGGBB`.

```typescript
const surface = new Extra.BufferSurface(window);

// Draw one frame with a writer callback
surface.presentWithWriter((view, width, height) => {
    view.fill(0xFF101010);
});

// Present a pre-filled buffer
const buffer = new Uint32Array(width * height);
surface.presentWithTyped(buffer);

// Variant safe to call from other threads
surface.presentWithThreadsafeWriter((view, width, height) => {});
```

### ThreadPool

```typescript
const pool = Extra.ThreadPool.default(); // or Extra.ThreadPool.main(), new Extra.ThreadPool(n)
pool.execute(() => {
    console.log('Running in thread pool');
});
```

### Timers

```typescript
await Extra.tokioSleep(Duration.fromMillis(100));
Extra.tokioInterval(Duration.fromMillis(100), () => {});
Extra.tokioCallSpawn(() => {});
Extra.threadInterval(Duration.fromMillis(100), () => {});
```

### Raw window handles

```typescript
const options = Extra.getRwhOptions(window);
console.log(options.system); // 'win32' | 'cocoa' | 'x11' | 'wayland'
```

Returns a discriminated union of raw window/system handles (see `SurfaceOptions` in [index.d.ts](../index.d.ts)) for handing to external renderers such as wgpu or vgpu.

## Instant and Duration

Time types used for `WaitUntil` timeouts and timers.

```typescript
const now = Instant.now();
const deadline = Instant.afterMillis(16); // ~60 FPS

const duration = Duration.fromMillis(100);
const halved = Duration.div(duration, 2);
```

`Instant`: `now()`, `afterSecs/Millis/Micros/Nanos()`, `add()`, `sub()`, `durationSince()`.
`Duration`: `fromSecs/Millis/Micros/Nanos()`, `add()`, `sub()`, `mul()`, `div()`.

