# 事件处理

## 常见窗口事件

在 `onWindowEvent` 回调中处理各种窗口事件：

```typescript
onWindowEvent: (activeEventLoop, windowId, event) => {
    switch (event.type) {
        case 'CloseRequested':
            // 用户请求关闭窗口（例如点击 X 按钮）
            console.log('Close requested');
            activeEventLoop.exit();
            break;

        case 'RedrawRequested':
            // 窗口需要重绘，在此执行渲染
            break;

        case 'SurfaceResized':
            // 窗口表面尺寸变化
            const { width, height } = event.size;
            console.log(`Resized to ${width}x${height}`);
            break;

        case 'Moved':
            // 窗口位置变化
            const { x, y } = event.position;
            console.log(`Moved to (${x}, ${y})`);
            break;

        case 'Focused':
            // 窗口获得或失去焦点
            console.log(`Focus: ${event.focused}`);
            break;

        case 'KeyboardInput':
            // 键盘输入事件
            const keyEvent = event.event;
            const { state, logicalKey, physicalKey, text, repeat } = keyEvent;

            // 字符键
            if (logicalKey.type === 'Character') {
                console.log(`Character: ${logicalKey.ch}, state: ${state}`);
            }

            // 命名键（Esc、Enter 等）
            if (logicalKey.type === 'Named') {
                console.log(`Named key: ${logicalKey.name}, state: ${state}`);
            }
            break;

        case 'ModifiersChanged':
            // 修饰键状态变化（Shift、Ctrl、Alt、Meta）
            const mods = event.modifiers.state;
            console.log({
                shift: mods.hasShift(),
                ctrl: mods.hasControl(),
                alt: mods.hasAlt(),
                meta: mods.hasMeta()
            });
            break;

        case 'PointerButton':
            // 鼠标按键事件
            const { button, state: btnState } = event;
            console.log(`Mouse button ${button.type}: ${btnState}`);
            break;

        case 'MouseWheel':
            // 滚轮事件
            const { deltaX, deltaY } = event.delta;
            console.log(`Wheel: (${deltaX}, ${deltaY})`);
            break;

        case 'PointerMoved':
            // 指针位置变化
            const position = event.position;
            console.log(`Pointer: (${position.x}, ${position.y})`);
            break;

        case 'PointerEntered':
            // 指针进入窗口
            console.log('Pointer entered');
            break;

        case 'PointerLeft':
            // 指针离开窗口
            console.log('Pointer left');
            break;

        case 'ScaleFactorChanged':
            // DPI 缩放因子变化
            const { scaleFactor, surfaceSizeWriter } = event;
            console.log(`New scale factor: ${scaleFactor}`);
            break;

        case 'ThemeChanged':
            // 系统主题变化
            console.log(`Theme: ${event.theme}`); // 'Light' 或 'Dark'
            break;

        case 'Destroyed':
            // 窗口已销毁
            console.log('Window destroyed');
            break;
    }
}
```
