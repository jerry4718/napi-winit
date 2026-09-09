# Control Flow Modes

napi-winit supports three control flow modes to optimize performance and responsiveness:

## Wait Mode (Recommended for event-driven apps)

Waits indefinitely for the next event. Most CPU-efficient option.

```typescript
eventLoop.setControlFlow({ type: 'Wait' });
```

**Use when:**
- Your app is purely event-driven
- No continuous animations or updates needed
- Battery life is important (mobile/laptop scenarios)

## WaitUntil Mode (Best for animations)

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

## Poll Mode (High-frequency updates)

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

## Mode Comparison

| Mode | CPU Usage | Latency | Best For |
|------|-----------|---------|----------|
| **Wait** | Minimal | Low | Event-driven apps |
| **WaitUntil** | Moderate | Predictable | Animations, games |
| **Poll** | High | Minimal | Real-time processing |
