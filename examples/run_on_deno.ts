import {
    ActiveEventLoop,
    ApplicationT1 as Application,
    EventLoop,
    Window,
    WindowAttributes,
    WindowId,
    WindowEvent,
} from "../index";

const eventLoop = new EventLoop();
let window: Window;

const app = Application.fromRefs({
    onResumed(eventLoop: ActiveEventLoop) {
        const attrs = new WindowAttributes()
            .withActive(true)
            .withFullscreen(null)
            .withResizable(true)
            // .withInnerSize({type: UnitType.Logical, width: 100, height: 100})
            .withPosition({type: "Logical", x: 500, y: 500})
            .withTransparent(false)
            .withTitle("Hello Deno");

        window = eventLoop.createWindow(attrs);

        const options = window.getSurfaceOptions();
    },
    onWindowEvent(eventLoop: ActiveEventLoop, windowId: WindowId, event: WindowEvent) {
    }
})

const interval = setInterval(() => {
    const { type, code } = eventLoop.pumpAppEvents(0, app);

    if (type === "Exit") {
        clearInterval(interval);
        Deno.exit(code);
    }
}, 15);