# 最佳实践

## 事件循环管理

**控制事件循环频率**，平衡 CPU 占用与响应性：

```typescript
// 好：控制泵事件频率
async function run() {
    while (true) {
        const status = eventLoop.pumpAppEvents(app);
        if (status.type === 'Exit') break;
        await new Promise(resolve => setTimeout(resolve, 1000 / 60)); // 60 FPS
    }
}

// 也好：使用 Wait 模式，让操作系统唤醒应用
onAboutToWait: (activeEventLoop) => {
    activeEventLoop.setControlFlow({ type: 'Wait' }); // 效率最高
}
```

## 资源管理

始终正确清理资源：

```typescript
let window: Window | null = null;
let surface: Extra.BufferSurface | null = null;

const app = Application.withOptions({
    onCanCreateSurfaces: (activeEventLoop) => {
        window = activeEventLoop.createWindow(attrs);
        surface = new Extra.BufferSurface(window);
    },
    onWindowEvent: (activeEventLoop, windowId, event) => {
        if (event.type === 'CloseRequested') {
            // 退出前清理
            surface = null;
            window = null;
            activeEventLoop.exit();
        }
    }
});
```

## 性能建议

1. **尽量使用 Wait 模式**：事件驱动应用的 CPU 占用最低
2. **合并重绘**：只在必要时调用 `requestRedraw()`
3. **在呈现前调用 `prePresentNotify()`**
4. **动画考虑使用 WaitUntil**：提供稳定的帧间隔

## 跨平台注意事项

- **窗口装饰**：各平台行为可能不同
- **DPI 缩放**：使用逻辑尺寸，并处理缩放因子变化
- **键盘布局**：用不同键盘配置测试
- **全屏模式**：检查平台特定的全屏行为
