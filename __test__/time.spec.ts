import test from 'ava'
import { Duration, Instant, Extra } from '../index.js'

test('Duration: from units', (t) => {
  const d1 = Duration.fromSecs(1)
  t.is(d1.secs, 1)
  t.is(d1.nanos, 0)

  const d2 = Duration.fromMillis(1500)
  t.is(d2.secs, 1)
  t.is(d2.nanos, 500_000_000)

  const d3 = Duration.fromMicros(1_000_000)
  t.is(d3.secs, 1)
  t.is(d3.nanos, 0)

  const d4 = Duration.fromNanos(1_234_567_890)
  t.is(d4.secs, 1)
  t.is(d4.nanos, 234_567_890)
})

test('Duration: arithmetic', (t) => {
  const d1 = Duration.fromSecs(1)
  const d2 = Duration.fromSecs(2)

  const sum = Duration.add(d1, d2)
  t.is(sum.secs, 3)

  const diff = Duration.sub(d2, d1)
  t.is(diff.secs, 1)

  const double = Duration.mul(d1, 2.5)
  t.is(double.secs, 2)
  t.is(double.nanos, 500_000_000)

  const half = Duration.div(d2, 2)
  t.is(half.secs, 1)
  t.is(half.nanos, 0)
})

test('Instant: now and after', (t) => {
  const now = Instant.now()
  t.true(now.secs >= 0)

  const later = Instant.afterSecs(1)
  // later should be approximately now + 1s
  t.true(later.secs >= now.secs + 1)
})

test('Instant: arithmetic', (t) => {
  const now = Instant.now()
  const d = Duration.fromSecs(5)

  const future = Instant.add(now, d)
  t.is(future.secs, now.secs + 5)
  t.is(future.nanos, now.nanos)

  const past = Instant.sub(future, d)
  t.is(past.secs, now.secs)
  t.is(past.nanos, now.nanos)
})

test('Extra: tokioSleep with Duration', async (t) => {
  const start = Instant.now()
  const sleepTime = Duration.fromMillis(100)
  await Extra.tokioSleep(sleepTime)
  const end = Instant.now()

  // Verify that at least 100ms has passed
  // Note: Instant is monotonic, so end >= start
  const diffSecs = end.secs - start.secs
  const diffNanos = end.nanos - start.nanos
  const totalMillis = diffSecs * 1000 + diffNanos / 1_000_000

  t.true(totalMillis >= 90) // Allow small margin for system scheduler
})
