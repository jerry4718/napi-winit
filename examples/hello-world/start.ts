import {Application, EventLoop, Extra, Window, WindowAttributes} from '@ylcc/napi-winit';

console.log('🚀 Hello napi-winit!');
console.log('📝 The simplest window example');
console.log('💡 Press ESC to exit\n');

// Create event loop
const eventLoop = new EventLoop();

// Configure window attributes
const attrs = new WindowAttributes()
    .withActive(true)
    .withResizable(true)
    .withInnerSize({type: 'Logical', width: 600, height: 400})
    .withTitle('Hello napi-winit - Press ESC to exit');

let window: Window;
let surface: Extra.BufferSurface;

// Create application
const app = Application.withSyncRef({
    onResumed: (eventLoop) => {
        // Create window when app starts
        window = eventLoop.createWindow(attrs);
        surface = new Extra.BufferSurface(window);
        window.requestRedraw();
        console.log('✅ Window created');
    },

    onWindowEvent: (eventLoop, _windowId, event) => {
        // Handle window events
        if (event.type === 'CloseRequested') {
            console.log('👋 Close requested by user');
            eventLoop.exit();
        }

        if (event.type === 'KeyboardInput') {
            const {logicalKey, state} = event.event;

            if (state === 'Released' && logicalKey.type === 'Character' && logicalKey.ch.toLowerCase() === 'r') {
                window.requestRedraw();
            }

            // Exit on ESC key
            if (state === 'Released' && logicalKey.type === 'Named' && logicalKey.name === 'Escape') {
                console.log('🚪 ESC pressed, exiting application');
                eventLoop.exit();
            }
        }

        if (event.type === 'Focused') {
            console.log(`🎯 Window focus: ${event.focused ? 'gained' : 'lost'}`);
        }

        if (event.type === 'RedrawRequested') {
            surface.presentWithWriter(view => view.fill(0x000000));
        }
    },

    onAboutToWait: (eventLoop) => {
        // Use Wait mode (most efficient)
        eventLoop.setControlFlow({type: 'Wait'});
    }
});

// Run event loop
async function run() {
    while (true) {
        const status = eventLoop.pumpAppEvents(0, app);
        if (status.type === 'Exit') {
            console.log(`\n✨ Application exited with code: ${status.code}`);
            break;
        }
        // 60 FPS
        await new Promise(resolve => setTimeout(resolve, 1000 / 60));
    }
}

run().catch(console.error);
