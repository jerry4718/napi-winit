import test from 'ava'
import {Duration, Instant, Extra} from '../index.js'

test('Duration: from units (precision check)', (t) => {
    // fromSecs with floating point
    const d1 = Duration.fromSecs(1.5)
    t.is(d1.secs, 1)
    t.is(d1.nanos, 500_000_000)

    // fromMillis with fractional part
    const d2 = Duration.fromMillis(1000.5)
    t.is(d2.secs, 1)
    t.is(d2.nanos, 500_000)

    // fromMicros with fractional part
    const d3 = Duration.fromMicros(1000.5)
    t.is(d3.secs, 0)
    t.is(d3.nanos, 1_000_500)

    // fromNanos with rounding
    const d4 = Duration.fromNanos(1.4)
    t.is(d4.nanos, 1)
    const d5 = Duration.fromNanos(1.6)
    t.is(d5.nanos, 2)
})

test('Duration: arithmetic success', (t) => {
    const d1 = Duration.fromSecs(1)
    const d2 = Duration.fromSecs(2)

    const sum = Duration.add(d1, d2)
    t.is(sum.secs, 3)

    const diff = Duration.sub(d2, d1)
    t.is(diff.secs, 1)

    // Floating point multiplication
    const double = Duration.mul(d1, 2.5)
    t.is(double.secs, 2)
    t.is(double.nanos, 500_000_000)

    // Floating point division
    const half = Duration.div(d2, 2.5)
    t.is(half.secs, 0)
    t.is(half.nanos, 800_000_000)
})

test('Duration: overflow/underflow errors', (t) => {
    const d1 = Duration.fromSecs(1)
    const d2 = Duration.fromSecs(2)

    // Underflow: 1s - 2s should throw
    t.throws(() => Duration.sub(d1, d2), {
        message: /subtracting durations/
    })

    // Large overflow (using a very large value)
    const dLarge = Duration.fromSecs(18e19); // Large enough to overflow u64 if doubled
    t.throws(() => Duration.add(dLarge, dLarge), {
        message: /adding durations/
    })
})

test('Instant: now and after success', (t) => {
    const now = Instant.now()
    t.true(now.secs >= 0)

    const later = Instant.afterMillis(100.5)
    // Check that it's at least after (roughly)
    t.true(later.secs >= now.secs)
})

test('Instant: arithmetic success', (t) => {
    const now = Instant.now()
    const d = Duration.fromSecs(5)

    const future = Instant.add(now, d)
    t.is(future.secs, now.secs + 5)
    t.is(future.nanos, now.nanos)

    const past = Instant.sub(future, d)
    t.is(past.secs, now.secs)
    t.is(past.nanos, now.nanos)
})

test('Instant: overflow/underflow errors', (t) => {
    const now = Instant.now()
    const hugeDuration = Duration.fromSecs(18e19);

    t.throws(() => Instant.add(now, hugeDuration), {
        message: /adding duration to instant/
    })

    const pastDuration = Duration.fromSecs(18e19);
    t.throws(() => Instant.sub(now, pastDuration), {
        message: /subtracting duration from instant/
    })
})

test('Extra: tokioSleep with Duration', async (t) => {
    const start = Instant.now()
    const sleepTime = Duration.fromMillis(100)
    await Extra.tokioSleep(sleepTime)
    const end = Instant.now()

    const diffSecs = end.secs - start.secs
    const diffNanos = end.nanos - start.nanos
    const totalMillis = diffSecs * 1000 + diffNanos / 1_000_000

    // 100ms sleep should take at least 45ms (allowing system jitter)
    t.true(totalMillis >= 100)
})
