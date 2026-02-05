import test from 'ava'

import {Duration, Extra} from '../index.js'

test('tokioSleep', async (t) => {
    const before = Date.now();
    await Extra.tokioSleep(Duration.fromMillis(50));
    t.assert(Date.now() - before >= 50)
    t.pass()
})
