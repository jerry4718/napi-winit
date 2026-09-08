import test from 'ava'

import {Application, EventLoop, WindowAttributes} from '../index.js'

test('window creation timing on x11 (resumed vs can_create_surfaces)', (t) => {
    const events: string[] = []
    const eventLoop = new EventLoop()

    const app = Application.withOptions({
        onNewEvents: (_eventLoop, cause) => {
            events.push(`newEvents:${JSON.stringify(cause)}`)
        },
        onResumed: () => {
            events.push('resumed')
        },
        onCanCreateSurfaces: (eventLoop) => {
            events.push('canCreateSurfaces')
            const window = eventLoop.createWindow(new WindowAttributes().withTitle('lifecycle-test'))
            events.push(`windowCreated:${window ? 'ok' : 'fail'}`)
        },
        onWindowEvent: (eventLoop, _windowId, event) => {
            events.push(`windowEvent:${event.type}`)
            if (event.type === 'RedrawRequested' || events.length > 60) {
                eventLoop.exit()
            }
        },
        onAboutToWait: () => {
            events.push('aboutToWait')
        },
        onSuspended: () => {
            events.push('suspended')
        },
        onDestroySurfaces: () => {
            events.push('destroySurfaces')
        },
    })

    for (let i = 0; i < 50; i++) {
        const status = eventLoop.pumpAppEvents(null, app)
        if (status.type === 'Exit') {
            break
        }
    }

    console.log('[lifecycle] callback order:\n' + events.map(e => `  ${e}`).join('\n'))

    t.true(events.some(e => e.startsWith('canCreateSurfaces')), 'canCreateSurfaces must fire')
    t.true(events.includes('windowCreated:ok'), 'window must be created inside canCreateSurfaces')
    t.false(events.includes('resumed'), 'resumed must NOT fire on x11 in winit 0.31')
})
