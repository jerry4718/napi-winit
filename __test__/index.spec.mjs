import test from 'ava'

import { tokioSleep } from '../index.js'

test('sum from native', async (t) => {
  await tokioSleep(50);
  t.is(1 + 2, 3)
})
