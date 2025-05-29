import test from 'ava'

import {asyncSleep} from '../index.js'

test('asyncSleep', async (t) => {
  await asyncSleep(50);
  t.pass("asyncSleep was over")
})
