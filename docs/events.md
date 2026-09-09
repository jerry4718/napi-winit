# Event Handling

## Common Window Events

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

        case 'SurfaceResized':
            // Window surface size changed
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
            // Modifier keys state changed (Shift, Ctrl, Alt, Meta)
            const mods = event.modifiers.state;
            console.log({
                shift: mods.hasShift(),
                ctrl: mods.hasControl(),
                alt: mods.hasAlt(),
                meta: mods.hasMeta()
            });
            break;

        case 'PointerButton':
            // Mouse button event
            const { button, state: btnState } = event;
            console.log(`Mouse button ${button.type}: ${btnState}`);
            break;

        case 'MouseWheel':
            // Mouse wheel event
            const { deltaX, deltaY } = event.delta;
            console.log(`Wheel: (${deltaX}, ${deltaY})`);
            break;

        case 'PointerMoved':
            // Pointer position changed
            const position = event.position;
            console.log(`Pointer: (${position.x}, ${position.y})`);
            break;

        case 'PointerEntered':
            // Pointer entered window
            console.log('Pointer entered');
            break;

        case 'PointerLeft':
            // Pointer left window
            console.log('Pointer left');
            break;

        case 'ScaleFactorChanged':
            // DPI scale factor changed
            const { scaleFactor, surfaceSizeWriter } = event;
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
