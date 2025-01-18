import {
    ActiveEventLoop,
    Application,
    ControlFlowEnum,
    EventLoop,
    KeyEnum,
    NamedKey,
    PumpStatus,
    StartCause,
    StartCauseEnum, threadSleep,
    UnitType,
    Window,
    WindowAttributes,
    WindowEvent,
    WindowEventEnum,
    WindowId
} from "../index.js";
import process from "node:process";

const event_loop = new EventLoop();

console.log({ControlFlowEnum, EventLoop, StartCause, WindowAttributes });

const attrs = new WindowAttributes()
    .withActive(true)
    .withFullscreen(null)
    .withResizable(true)
    // .withInnerSize({type: UnitType.Logical, width: 100, height: 100})
    .withPosition({type: UnitType.Logical, x: 500, y: 500})
    .withTransparent(false);

let window: Window;

let mode: ControlFlowEnum = ControlFlowEnum.Wait;
let wait_cancelled: boolean = false;
let close_requested: boolean = false;
let request_redraw: boolean = false;

const app: Application = {
    async onNewEvents(eventLoop: ActiveEventLoop, cause: StartCause): Promise<void> {
        // console.log({ type: cause.type });
        if (cause.type === StartCauseEnum.WaitCancelled) {
            wait_cancelled = (mode === ControlFlowEnum.WaitUntil);
        }
    },
    onResumed(eventLoop: ActiveEventLoop): void {
        attrs.withTitle("Press 1, 2, 3 to change control flow mode. Press R to toggle redraw requests.");
        window = eventLoop.createWindow(attrs)
    },
    async onWindowEvent(eventLoop: ActiveEventLoop, window_id: WindowId, event: WindowEvent): Promise<void> {
        if (event.type === WindowEventEnum.CloseRequested) {
            close_requested = true;
            return;
        }
        if (event.type === WindowEventEnum.KeyboardInput) {
            const keyEvent = event.KeyboardInput.event;
            const { state, logicalKey, text } = keyEvent;

            if (logicalKey.type === KeyEnum.Character) {
                if (logicalKey.Character.elem0 === "1") {
                    mode = ControlFlowEnum.Wait;
                }
                if (logicalKey.Character.elem0 === "2") {
                    mode = ControlFlowEnum.WaitUntil;
                }
                if (logicalKey.Character.elem0 === "3") {
                    mode = ControlFlowEnum.Poll;
                }
                if (logicalKey.Character.elem0 === "r") {
                    request_redraw = !request_redraw;
                }
            }
            if (logicalKey.type === KeyEnum.Named) {
                if (logicalKey.Named.name === NamedKey.Escape) {
                    close_requested = true;
                }
            }
            console.log({ state, text, mode })
            return;
        }
        if (event.type === WindowEventEnum.RedrawRequested) {
            window.prePresentNotify();
            return;
        }
        // console.log({ window_id, event: event.type });
    },
    async onAboutToWait(eventLoop: ActiveEventLoop): Promise<void> {
        if (request_redraw && !wait_cancelled && !close_requested) {
            window?.requestRedraw();
        }

        if (mode == ControlFlowEnum.Wait) {
            eventLoop.setControlFlowWait();
        }
        if (mode == ControlFlowEnum.WaitUntil) {
            if (wait_cancelled) {
                eventLoop.setControlFlowWaitUntil(100);
            }
        }
        if (mode == ControlFlowEnum.Poll) {
            threadSleep(100);
            eventLoop.setControlFlowPoll();
        }

        if (close_requested) {
            eventLoop.exit()
            process.exit()
        }
    }
};

const interval = setInterval(() => {
    const { type, code } = event_loop.pumpAppEvents(30, app);

    if (type === PumpStatus.Type.Exit) {
        clearInterval(interval);
        process.exit(code);
    }
}, 30);

console.log(attrs, { transparent: attrs.transparent });
