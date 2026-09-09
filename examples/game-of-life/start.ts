import {Application, EventLoop, Extra, Window, WindowAttributes} from '@ylcc/napi-winit';
import {compute, type Device, initFromDevice, storage} from 'vgpu/node';
import {createNodeDevice} from "@vgpu/adapter-node";

const CELL_SIZE = 8;
const STEP_INTERVAL_MS = 100;
const BACKGROUND_COLOR = 0xFF101418;
const GRID_COLOR = 0xFF1B252C;
const LIVE_COLOR = 0xFF72E06A;

const LIFE_SHADER = `
struct GridInfo {
    columns: u32,
    rows: u32,
}

@group(0) @binding(0) var<storage, read> input_cells: array<u32>;
@group(0) @binding(1) var<storage, read_write> output_cells: array<u32>;
@group(0) @binding(2) var<storage, read> grid: GridInfo;

fn cell_at(x: i32, y: i32) -> u32 {
    let wrapped_x = (x + i32(grid.columns)) % i32(grid.columns);
    let wrapped_y = (y + i32(grid.rows)) % i32(grid.rows);
    return input_cells[u32(wrapped_y) * grid.columns + u32(wrapped_x)];
}

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    if (id.x >= grid.columns || id.y >= grid.rows) {
        return;
    }

    var neighbors = 0u;
    for (var dy = -1; dy <= 1; dy++) {
        for (var dx = -1; dx <= 1; dx++) {
            if (dx != 0 || dy != 0) {
                neighbors += cell_at(i32(id.x) + dx, i32(id.y) + dy);
            }
        }
    }

    let index = id.y * grid.columns + id.x;
    let alive = input_cells[index] == 1u;
    output_cells[index] = select(0u, 1u, neighbors == 3u || (alive && neighbors == 2u));
}
`;

console.log("Conway's Game of Life");
console.log('SPACE: pause/resume');
console.log('N: advance one generation while paused');
console.log('R: create a new random population');
console.log('ESC: exit\n');

const eventLoop = new EventLoop();
const attrs = new WindowAttributes()
    .withSurfaceSize({type: 'Logical', width: 800, height: 600})
    .withTitle("Conway's Game of Life - SPACE:Pause  N:Step  R:Reset");

let window: Window;
let surface: Extra.BufferSurface;
let cells = new Uint32Array();
let columns = 0;
let rows = 0;
let generation = 0;
let running = true;
let stepRequested = false;
let lastStepTime = 0;
let gpuContext: Awaited<ReturnType<typeof initFromDevice>>;
let gpuDevice: Device;
let lifeCompute: ReturnType<typeof compute>;
let inputBuffer: ReturnType<typeof storage>;
let outputBuffer: ReturnType<typeof storage>;
let gridBuffer: ReturnType<typeof storage>;
let advanceInFlight = false;
let stateRevision = 0;

function randomizeGrid(): void {
    for (let i = 0; i < cells.length; i++) {
        cells[i] = Math.random() < 0.28 ? 1 : 0;
    }
    inputBuffer.write(cells);
    generation = 0;
    stateRevision++;
}

function resizeGrid(width: number, height: number): void {
    const newColumns = Math.max(1, Math.floor(width / CELL_SIZE));
    const newRows = Math.max(1, Math.floor(height / CELL_SIZE));

    if (newColumns === columns && newRows === rows) {
        return;
    }

    columns = newColumns;
    rows = newRows;
    cells = new Uint32Array(columns * rows);
    inputBuffer = storage(gpuContext, cells.byteLength, 'read');
    outputBuffer = storage(gpuContext, cells.byteLength, 'read-write');
    gridBuffer = storage(gpuContext, 8, 'read');
    gridBuffer.write(new Uint32Array([columns, rows]));
    lifeCompute = compute(gpuContext, LIFE_SHADER);
    randomizeGrid();
}

async function advanceGeneration(): Promise<void> {
    const revision = stateRevision;
    const source = inputBuffer;
    const destination = outputBuffer;
    const computePipeline = lifeCompute;
    const workgroupsX = Math.ceil(columns / 8);
    const workgroupsY = Math.ceil(rows / 8);

    computePipeline
        .set({
            input_cells: source,
            output_cells: destination,
            grid: gridBuffer
        })
        .dispatch(workgroupsX, workgroupsY);

    const nextCells = new Uint32Array(await destination.read());
    if (revision !== stateRevision) {
        return;
    }

    cells = nextCells;
    source.write(cells);
    generation++;
}

function scheduleGeneration(): void {
    if (advanceInFlight || columns === 0) {
        return;
    }

    advanceInFlight = true;
    void advanceGeneration()
        .catch(console.error)
        .finally(() => {
            advanceInFlight = false;
            window.requestRedraw();
        });
}

function drawGrid(view: Uint32Array, width: number, height: number): void {
    view.fill(BACKGROUND_COLOR);

    for (let y = 0; y < rows; y++) {
        for (let x = 0; x < columns; x++) {
            const color = cells[y * columns + x] === 1 ? LIVE_COLOR : GRID_COLOR;
            const left = x * CELL_SIZE + 1;
            const top = y * CELL_SIZE + 1;
            const right = Math.min(left + CELL_SIZE - 1, width);
            const bottom = Math.min(top + CELL_SIZE - 1, height);

            for (let pixelY = top; pixelY < bottom; pixelY++) {
                view.fill(color, pixelY * width + left, pixelY * width + right);
            }
        }
    }
}

const app = Application.withOptions({
    onCanCreateSurfaces: (activeEventLoop) => {
        window = activeEventLoop.createWindow(attrs);
        surface = new Extra.BufferSurface(window);
        window.requestRedraw();
    },

    onWindowEvent: (activeEventLoop, _windowId, event) => {
        if (event.type === 'CloseRequested') {
            activeEventLoop.exit();
            return;
        }

        if (event.type === 'KeyboardInput' && event.event.state === 'Released') {
            const {logicalKey, physicalKey} = event.event;

            if (logicalKey.type === 'Named' && logicalKey.name === 'Escape') {
                activeEventLoop.exit();
                return;
            }

            if (physicalKey.type === 'Code' && physicalKey.code === 'Space') {
                running = !running;
                console.log(running ? 'Running' : `Paused at generation ${generation}`);
            }

            if (logicalKey.type === 'Character') {
                const key = logicalKey.ch.toLowerCase();
                if (key === 'r') {
                    randomizeGrid();
                    console.log('Created a new random population');
                } else if (key === 'n' && !running) {
                    stepRequested = true;
                }
            }

            window.requestRedraw();
        }

        if (event.type === 'RedrawRequested') {
            window.prePresentNotify();
            surface.presentWithWriter((view, width, height) => {
                resizeGrid(width, height);

                const now = Date.now();
                if (stepRequested || (running && now - lastStepTime >= STEP_INTERVAL_MS)) {
                    scheduleGeneration();
                    stepRequested = false;
                    lastStepTime = now;
                }

                drawGrid(view, width, height);
            });

            if (running) {
                window.requestRedraw();
            }
        }
    },

    onAboutToWait: (activeEventLoop) => {
        activeEventLoop.setControlFlow({type: 'Wait'});
    }
});

async function run(): Promise<void> {
    // Dawn's OpenGL backend translates WGSL into GLSL whose generated code fails to
    // compile (integer mix()). backend: "webgpu" compiles on a native backend instead.
    gpuDevice = await createNodeDevice({backend: 'webgpu'});
    gpuContext = await initFromDevice(gpuDevice.gpu);
    gpuContext.onError(error => {
        console.error(`[vgpu] ${error.code}: ${error.message}`);
    });

    while (true) {
        const status = eventLoop.pumpAppEvents(app);
        if (status.type === 'Exit') {
            console.log(`Exited at generation ${generation}`);
            await gpuContext.settled();
            gpuContext.dispose();
            gpuDevice.dispose();
            break;
        }
        await new Promise(resolve => setTimeout(resolve, 1000 / 60));
    }
}

run().catch(console.error);
