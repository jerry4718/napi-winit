import test from 'ava'

import {Extra} from '../index.js'

test('asyncSleep', async (t) => {
  await Extra.asyncSleep(50);
  t.pass("asyncSleep was over")
})
