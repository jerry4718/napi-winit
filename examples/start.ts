import {type ControlFlow} from "@ylcc/napi-winit";
import {
    Application,
    asyncSleep,
    EventLoop,
    SoftSurface,
    threadInterval,
    tokioInterval,
    tokioSleep,
    Timeout,
    Window,
    WindowAttributes,
} from "@ylcc/napi-winit";
import process from "node:process";
import { Buffer } from "node:buffer";

const event_loop = new EventLoop();

const attrs = new WindowAttributes()
    .withActive(true)
    .withFullscreen(null)
    .withResizable(true)
    // .withInnerSize({type: UnitType.Logical, width: 100, height: 100})
    .withPosition({ type: "Logical", x: 500, y: 500 })
    .withTransparent(false)
    .withTitle(
        "Press 1, 2, 3 to change control flow mode. Press R to toggle redraw requests.",
    );

let window: Window;
let surface: SoftSurface;

let mode: ControlFlow["type"] = "WaitUntil";
let wait_cancelled: boolean = false;
let close_requested: boolean = false;
let request_redraw: boolean = true;

let buffer = new Uint32Array(0);

function prePresentNotify() {
    window.prePresentNotify();
}

function get_pixels() {
    const { width, height } = window.innerSize();
    return width * height;
}

function append_buffer(pixels: number) {
    const old = buffer;
    buffer = new Uint32Array(pixels);
    buffer.fill(0xFF000000 | Math.floor(Math.random() * 0xFFFFFF));
    buffer.set(old);
}

function slice_buffer(pixels: number) {
    buffer = buffer.slice(-pixels);
}

function write_buffer(/*err: (Error | null), */view: Uint32Array)  {
    // if (err) console.error(err);
    console.log(view);
    view.set(buffer);
}

function writeWithSize(width: number, height: number, view: Uint32Array) {
    const pixels = width * height;
    if (pixels > buffer.length) {
        append_buffer(pixels);
    }
    if (pixels < buffer.length) {
        slice_buffer(pixels);
    }
    // console.log(view);
    view.set(buffer)
}

function present() {
    // surface.presentWithWriter((e, view) => {
    //     // console.log(view);
    //     view.set(buffer)
    // });
    surface.presentWithWriterVec(writeWithSize);
    // surface.presentWithTyped(buffer);
    // surface.presentWithBuffer(new Uint8Array(buffer.buffer));
}

const frame_stamps: number[] = [];

function stamps() {
    const now = Date.now();
    const pre = now - 1000;
    while (frame_stamps[0] && frame_stamps[0] < pre) {
        frame_stamps.shift()
    }
    frame_stamps.push(now)
}

function redraw() {
    prePresentNotify();
    // const pixels = get_pixels();
    // if (pixels > buffer.length) {
    //     append_buffer(pixels);
    // }
    // if (pixels < buffer.length) {
    //     slice_buffer(pixels);
    // }
    stamps();
    present();
}

const app = Application.withAsync({
    onNewEvents: (_eventLoop, cause) => {
        if (cause.type === "WaitCancelled" && mode === "WaitUntil") {
            wait_cancelled = true;
        }
    },
    onResumed: (eventLoop) => {
        window = eventLoop.createWindow(attrs);
        surface = new SoftSurface(window);
    },
    onWindowEvent: (_eventLoop, _windowId, event) => {
        if (event.type === "CloseRequested") {
            close_requested = true;
            return;
        }
        if (event.type === "KeyboardInput") {
            const keyEvent = event.event;
            const { state, logicalKey, text } = keyEvent;

            if (logicalKey.type === "Character" && state === "Released") {
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
                if (logicalKey.name === "Escape") {
                    close_requested = true;
                }
            }
            console.log({ state, text, mode });
            return;
        }
        if (event.type === "RedrawRequested") {
            redraw();
            return;
        }
        // console.log({ window_id, event: event.type });
    },
    onAboutToWait: async (eventLoop) => {
        // console.log(`request_redraw = ${ request_redraw }, wait_cancelled = ${ wait_cancelled }, close_requested = ${ close_requested }`)
        if (request_redraw && !wait_cancelled && !close_requested) {
            window?.requestRedraw();
        }

        if (mode === "Wait") {
            eventLoop.setControlFlow({ type: "Wait" });
        }
        if (mode === "WaitUntil" && wait_cancelled) {
            eventLoop.setControlFlow({ type: "WaitUntil", timeout: Timeout.fromNanos(1_000_000 / 120) });
        }
        if (mode === "Poll") {
            await tokioSleep(Timeout.fromMillis(100));
            eventLoop.setControlFlow({ type: "Poll" });
        }

        if (close_requested) {
            eventLoop.exit();
        }
    },
});

// while (true) {
//     const sleep = asyncSleep(7);
//     const status = event_loop.pumpAppEvents(0, app);
//
//     if (status.type === "Continue") {
//         await sleep;
//         continue;
//     }
//     process.exit(status.code);
// }

function pump() {
    const status = event_loop.pumpAppEvents(0, app);
    if (status.type === "Continue") {
        return;
    }
    process.exit(status.code);
}

tokioInterval(Timeout.fromNanos(1_000_000 / 60), pump);