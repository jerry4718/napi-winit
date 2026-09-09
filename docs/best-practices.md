# Best Practices

## Event Loop Management

**Control event loop frequency** to balance CPU usage and responsiveness:

```typescript
// Good: Control pump frequency
async function run() {
    while (true) {
        const status = eventLoop.pumpAppEvents(app);
        if (status.type === 'Exit') break;
        await new Promise(resolve => setTimeout(resolve, 1000 / 60)); // 60 FPS
    }
}

// Also good: Use Wait mode to let the OS wake your app
onAboutToWait: (eventLoop) => {
    eventLoop.setControlFlow({ type: 'Wait' }); // Most efficient
}
```

## Resource Management

Always properly clean up resources:

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
            // Cleanup before exit
            surface = null;
            window = null;
            eventLoop.exit();
        }
    }
});
```

## Performance Tips

1. **Use Wait mode when possible**: Most CPU-efficient for event-driven apps
2. **Batch redraws**: Only call `requestRedraw()` when necessary
3. **Profile your rendering**: Use `prePresentNotify()` before presenting frames
4. **Consider WaitUntil for animations**: Provides consistent frame timing

## Cross-platform Considerations

- **Window decorations**: Behavior may vary across platforms
- **DPI scaling**: Use logical sizes and handle scale factor changes
- **Keyboard layouts**: Test with different keyboard configurations
- **Fullscreen modes**: Check platform-specific fullscreen behavior
