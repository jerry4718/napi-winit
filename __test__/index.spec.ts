import test from 'ava'

import {Duration, Extra} from '../index.js'

test('asyncSleep', async (t) => {
  await Extra.tokioSleep(Duration.fromMillis(50));
  t.pass("asyncSleep was over")
})
