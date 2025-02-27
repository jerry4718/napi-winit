import {
    ActiveEventLoop,
    ControlFlow,
    ApplicationT1 as Application,
    EventLoop,
    NamedKey,
    StartCause,
    Window,
    WindowAttributes,
    WindowEvent,
    WindowId,
    asyncSleep
} from "../index";
import process from "node:process";

const event_loop = new EventLoop();

const attrs = new WindowAttributes()
    .withActive(true)
    .withFullscreen(null)
    .withResizable(true)
    // .withInnerSize({type: UnitType.Logical, width: 100, height: 100})
    .withPosition({type: "Logical", x: 500, y: 500})
    .withTransparent(false);

let window: Window;

let mode: ControlFlow["type"] = "Wait";
let wait_cancelled: boolean = false;
let close_requested: boolean = false;
let request_redraw: boolean = false;

const app = Application.fromRefs({
    onNewEvents: (_eventLoop: ActiveEventLoop, cause: StartCause) => {
        // console.log({ type: cause.type });
        if (cause.type === "WaitCancelled") {
            wait_cancelled = (mode === "WaitUntil");
        }
    },
    onResumed: (eventLoop: ActiveEventLoop) => {
        attrs.withTitle("Press 1, 2, 3 to change control flow mode. Press R to toggle redraw requests.");
        window = eventLoop.createWindow(attrs)
    },
    onWindowEvent: async (_eventLoop: ActiveEventLoop, _window_id: WindowId, event: WindowEvent) => {
        if (event.type === "CloseRequested") {
            close_requested = true;
            return;
        }
        if (event.type === "KeyboardInput") {
            const keyEvent = event.event;
            const { state, logicalKey, text } = keyEvent;

            if (logicalKey.type === "Character") {
                if (logicalKey.ch === "1") {
                    mode = "Wait";
                }
                if (logicalKey.ch === "2") {
                    mode = "WaitUntil";
                }
                if (logicalKey.ch === "3") {
                    mode = "Poll";
                }
                if (logicalKey.ch === "r") {
                    request_redraw = !request_redraw;
                }
            }
            if (logicalKey.type === "Named") {
                if (logicalKey.name === NamedKey.Escape) {
                    close_requested = true;
                }
            }
            console.log({ state, text, mode })
            return;
        }
        if (event.type === "RedrawRequested") {
            window.prePresentNotify();
            return;
        }
        // console.log({ window_id, event: event.type });
    },
    onAboutToWait: async (eventLoop: ActiveEventLoop) => {
        if (request_redraw && !wait_cancelled && !close_requested) {
            window?.requestRedraw();
        }

        if (mode === "Wait") {
            eventLoop.setControlFlowWait();
        }
        if (mode === "WaitUntil") {
            if (wait_cancelled) {
                eventLoop.setControlFlowWaitUntil(100);
            }
        }
        if (mode === "Poll") {
            await asyncSleep(100);
            eventLoop.setControlFlowPoll();
        }

        if (close_requested) {
            eventLoop.exit()
            process.exit()
        }
    }
});

const interval = setInterval(() => {
    const { type, code } = event_loop.pumpAppEvents(30, app);

    if (type === "Exit") {
        clearInterval(interval);
        process.exit(code);
    }
}, 30);

console.log(attrs, { transparent: attrs.transparent });
