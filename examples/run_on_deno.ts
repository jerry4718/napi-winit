import {
    ActiveEventLoop,
    Application,
    EventLoop,
    PumpStatus,
    UnitType,
    Window,
    WindowAttributes,
    WindowEvent,
    WindowId
} from "..";

const eventLoop = new EventLoop();
let window: Window;

const app: Application = {
    async onResumed(eventLoop: ActiveEventLoop) {
        const attrs = new WindowAttributes()
            .withActive(true)
            .withFullscreen(null)
            .withResizable(true)
            // .withInnerSize({type: UnitType.Logical, width: 100, height: 100})
            .withPosition({type: UnitType.Logical, x: 500, y: 500})
            .withTransparent(false)
            .withTitle("Hello Deno");

        window = eventLoop.createWindow(attrs);

        const options = window.getSurfaceOptions();
    },
    async onWindowEvent(eventLoop: ActiveEventLoop, windowId: WindowId, event: WindowEvent) {
    }
};

const interval = setInterval(() => {
    const { type, code } = eventLoop.pumpAppEvents(15, app);

    if (type === PumpStatus.Type.Exit) {
        clearInterval(interval);
        Deno.exit(code);
    }
}, 15);