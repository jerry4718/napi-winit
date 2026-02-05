import {
    Application,
    type ControlFlow,
    Duration,
    EventLoop,
    Extra,
    Instant,
    Window,
    WindowAttributes
} from "@ylcc/napi-winit";
import process from "node:process";

const {BufferSurface, tokioSleep} = Extra;

const eventLoop = new EventLoop();

const attrs = new WindowAttributes()
    .withActive(true)
    .withFullscreen(null)
    .withResizable(true)
    .withInnerSize({type: 'Logical', width: 480, height: 320})
    .withPosition({type: "Logical", x: 500, y: 500})
    .withTransparent(false)
    .withTitle(
        "Press 1, 2, 3 to change control flow mode. Press R to toggle redraw requests.",
    );

let window: Window;
let surface: Extra.BufferSurface;

let mode: ControlFlow["type"] = "WaitUntil";
let wait_cancelled: boolean = false;
let close_requested: boolean = false;
let request_redraw: boolean = false;

let buffer = new Uint32Array(0);
let last_line = new Uint32Array(0);
let old_width: number = 0, old_height: number = 0;

function prePresentNotify() {
    window.prePresentNotify();
}

function update_buffer(width: number, height: number) {
    if (width === old_width && height === old_height) {
        if (last_line.length !== width) last_line = new Uint32Array(width);
        buffer.set(buffer.slice(width), 0);
        last_line.fill(0xFF000000 | Math.floor(Math.random() * 0xFFFFFF));
        buffer.set(last_line, (height - 1) * width);
        return;
    }

    const old_buffer = buffer;
    buffer = new Uint32Array(width * height);
    buffer.fill(0xFF000000 | Math.floor(Math.random() * 0xFFFFFF));

    for (let line = 0; line < Math.min(height, old_height); line++) {
        const old_offset_end = (line + Math.max(old_height - height, 0) + 1) * old_width;
        const old_line_buffer = old_buffer.slice(old_offset_end - Math.min(width, old_width), old_offset_end);
        buffer.set(old_line_buffer, line * width);
    }
}

function writeWithSize(width: number, height: number, view: Uint32Array) {
    update_buffer(width, height);
    // console.log(view);
    view.set(buffer);
    old_width = width;
    old_height = height;
}

function present() {
    surface.presentWithWriter(writeWithSize);
}

const frame_stamps: number[] = [];
const fps = () => frame_stamps.length;

function stamps() {
    const now = Date.now();
    const pre = now - 1000;
    while (frame_stamps[0] && frame_stamps[0] < pre) {
        frame_stamps.shift()
    }
    frame_stamps.push(now);
}

function redraw() {
    prePresentNotify();
    present();
    stamps();
}

let prev = 0;

function print_fps() {
    const now = Date.now();
    if (now - prev <= 30) return;
    prev = now;
    console.log({fps: fps()});
}

const app = Application.withSyncRef({
    onNewEvents: (_eventLoop, cause) => {
        wait_cancelled = (mode === "WaitUntil" && cause.type === "WaitCancelled");

        if (mode === "WaitUntil" && cause.type === "WaitCancelled") {
            console.log({mode, wait_cancelled, cause: cause});
        }

        if (mode === "Wait" && cause.type === "ResumeTimeReached") {
            console.log({mode, wait_cancelled, cause: cause});
        }
    },
    onResumed: (eventLoop) => {
        window = eventLoop.createWindow(attrs);
        surface = new BufferSurface(window);
    },
    onWindowEvent: (_eventLoop, _windowId, event) => {
        if (event.type === "CloseRequested") {
            close_requested = true;
            return;
        }
        if (event.type === "KeyboardInput") {
            const keyEvent = event.event;
            const {state, logicalKey, text} = keyEvent;

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
                if (logicalKey.ch === "s") {
                    window.requestInnerSize({type: 'Logical', width: 480, height: 320});
                }
            }
            if (logicalKey.type === "Named") {
                if (logicalKey.name === "Escape") {
                    close_requested = true;
                }
            }
            console.log({state, text, mode});
            return;
        }
        if (event.type === "RedrawRequested") {
            redraw();
            print_fps();
            return;
        }
    },
    onAboutToWait: async (eventLoop) => {
        if (request_redraw && !wait_cancelled && !close_requested) {
            window?.requestRedraw();
        }

        if (mode === "Wait") {
            eventLoop.setControlFlow({type: "Wait"});
        }
        if (mode === "WaitUntil" && wait_cancelled) {
            eventLoop.setControlFlow({type: "WaitUntil", timeout: Instant.afterNanos(1_000_000 / 120)});
        }
        if (mode === "Poll") {
            await tokioSleep(Duration.fromMillis(100));
            eventLoop.setControlFlow({type: "Poll"});
        }

        if (close_requested) {
            eventLoop.exit();
        }
    },
});

function pump() {
    const status = eventLoop.pumpAppEvents(0, app);
    if (status.type === "Continue") {
        return;
    }
    console.log('\n✨ Application exited');
    process.exit(status.code);
}

async function run() {
    while (true) {
        // await tokioSleep(Timeout.fromNanos(1_000_000 / 60));
        pump();
        await new Promise(resolve => setTimeout(resolve, 1000 / 60));
    }
    // threadInterval(Timeout.fromNanos(1_000_000 / 60), pump);
}

run().catch(console.error);