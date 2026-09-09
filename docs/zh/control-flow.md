# 控制流模式

napi-winit 支持三种控制流模式，用于平衡性能与响应性：

## Wait 模式（事件驱动应用推荐）

无限等待下一个事件，CPU 占用最低。

```typescript
activeEventLoop.setControlFlow({ type: 'Wait' });
```

**适用场景：**
- 纯事件驱动的应用
- 不需要持续动画或更新
- 在意电池续航（移动/笔记本场景）

## WaitUntil 模式（最适合动画）

等待到指定时间点，或下一个事件到来。

```typescript
import { Instant } from '@ylcc/napi-winit';

// 等待 16ms（约 60 FPS）
activeEventLoop.setControlFlow({
    type: 'WaitUntil',
    timeout: Instant.afterMillis(16)
});

// 等待 8.33ms（约 120 FPS）
activeEventLoop.setControlFlow({
    type: 'WaitUntil',
    timeout: Instant.afterNanos(1_000_000 / 120)
});
```

**适用场景：**
- 动画或游戏
- 需要稳定的帧间隔
- 需要精确控制更新频率

## Poll 模式（高频更新）

立即处理所有事件，不等待。

```typescript
import { Extra, Duration } from '@ylcc/napi-winit';

onAboutToWait: async (activeEventLoop) => {
    // 加入小延迟，避免 100% CPU 占用
    await Extra.tokioSleep(Duration.fromMillis(1));
    activeEventLoop.setControlFlow({ type: 'Poll' });
}
```

**适用场景：**
- 追求最高响应性
- 处理密集的实时数据
- 不在意 CPU 占用

## 模式对比

| 模式 | CPU 占用 | 延迟 | 适用 |
|------|----------|------|------|
| **Wait** | 极低 | 低 | 事件驱动应用 |
| **WaitUntil** | 中等 | 可预期 | 动画、游戏 |
| **Poll** | 高 | 极低 | 实时处理 |
