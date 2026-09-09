import test from 'ava'
import { Duration, Extra } from '../index.js'

const wait = (ms: number) => Extra.tokioSleep(Duration.fromMillis(ms))

test('Extra: tokioInterval calls immediately then per period', async (t) => {
    let count = 0
    const stopper = Extra.tokioInterval(Duration.fromMillis(50), () => {
        count++
        console.log(`test 1: ${ count }`)
    })

    await wait(180)
    // First call is immediate, then one per 50ms period
    t.true(count >= 3)

    stopper.stop()
})

test('Extra: tokioInterval stop() ends the loop', async (t) => {
    let count = 0
    const stopper = Extra.tokioInterval(Duration.fromMillis(50), () => {
        count++
        console.log(`test 2: ${ count }`)
    })

    await wait(180)
    const afterRun = count
    t.true(afterRun >= 3)

    stopper.stop()
    await wait(200)
    t.is(count, afterRun)
})
