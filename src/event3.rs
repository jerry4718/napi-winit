use winit::event::DeviceId;
use proc::enum_to_mod;
use crate::window::WindowId;

pub struct UserPayload {}

enum_to_mod!(
    enum Event {
        NewEvents(StartCause),
        WindowEvent {
            window_id: WindowId,
            event: WindowEvent,
        },
        DeviceEvent {
            device_id: DeviceId,
            event: DeviceEvent,
        },
        UserEvent(#[conf_dirct_type] UserPayload),
        Suspended,
        Resumed,
        AboutToWait,
        LoopExiting,
        MemoryWarning,
    }
);

enum_to_mod!(
    enum StartCause {
        ResumeTimeReached {
            start: Instant,
            requested_resume: Instant,
        },
        WaitCancelled {
            start: Instant,
            requested_resume: Option<Instant>,
        },
        Poll,
        Init,
    }
);

enum_to_mod!(
    enum WindowEvent {
        ActivationTokenDone {
            serial: AsyncRequestSerial,
            token: ActivationToken,
        },
        Resized(#[conf_trans_type = Size] PhysicalSize<u32>),
        Moved(#[conf_trans_type = Position<u32>] PhysicalPosition<i32>),
        CloseRequested,
        Destroyed,
        DroppedFile(PathBuf),
        HoveredFile(PathBuf),
        HoveredFileCancelled,
        Focused(bool),
        KeyboardInput {
            device_id: DeviceId,
            event: KeyEvent,
            is_synthetic: bool,
        },
        ModifiersChanged(Modifiers),
        Ime(Ime),
        CursorMoved {
            device_id: DeviceId,
            #[conf_trans_type = Position] position: PhysicalPosition<f64>,
        },
        CursorEntered {
            device_id: DeviceId,
        },
        CursorLeft {
            device_id: DeviceId,
        },
        MouseWheel {
            device_id: DeviceId,
            delta: MouseScrollDelta,
            phase: TouchPhase,
        },
        MouseInput {
            device_id: DeviceId,
            state: ElementState,
            button: MouseButton,
        },
        PinchGesture {
            device_id: DeviceId,
            delta: f64,
            phase: TouchPhase,
        },
        PanGesture {
            device_id: DeviceId,
            #[conf_trans_type = Position] delta: PhysicalPosition<f32>,
            phase: TouchPhase,
        },
        DoubleTapGesture {
            device_id: DeviceId,
        },
        RotationGesture {
            device_id: DeviceId,
            delta: f32,
            phase: TouchPhase,
        },
        TouchpadPressure {
            device_id: DeviceId,
            pressure: f32,
            stage: i64,
        },
        AxisMotion {
            device_id: DeviceId,
            axis: AxisId,
            value: f64,
        },
        Touch(Touch),
        ScaleFactorChanged {
            scale_factor: f64,
            inner_size_writer: InnerSizeWriter,
        },
        ThemeChanged(Theme),
        Occluded(bool),
        RedrawRequested,
    }
);

enum_to_mod!(
    enum DeviceEvent {
        Added,
        Removed,
        MouseMotion {
            delta: (f64, f64),
        },
        MouseWheel {
            delta: MouseScrollDelta,
        },
        Motion {
            axis: AxisId,
            value: f64,
        },
        Button {
            button: ButtonId,
            state: ElementState,
        },
        Key(RawKeyEvent),
    }
);