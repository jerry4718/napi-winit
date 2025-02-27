import test from 'ava'

import {asyncSleep} from '../index.js'

test('sum from native', async (t) => {
  await asyncSleep(50);
  t.is(1 + 2, 3)
})
