use winit::{
    event::{
        DeviceEvent as OriginDeviceEvent,
        DeviceId as OriginDeviceId,
        Event as OriginEvent,
        StartCause as OriginStartCause,
        WindowEvent as OriginWindowEvent,
        KeyEvent as OriginKeyEvent,
        Modifiers as OriginModifiers,
        Ime as OriginIme,
        MouseScrollDelta as OriginMouseScrollDelta,
        TouchPhase as OriginTouchPhase,
        ElementState as OriginElementState,
        MouseButton as OriginMouseButton,
        AxisId as OriginAxisId,
        Touch as OriginTouch,
        InnerSizeWriter as OriginInnerSizeWriter,
        ButtonId as OriginButtonId,
        RawKeyEvent as OriginRawKeyEvent,
    },
    window::{
        WindowId as OriginWindowId,
        ActivationToken as OriginActivationToken,
        Theme as OriginTheme
    },
    event_loop::AsyncRequestSerial as OriginAsyncRequestSerial
};

use crate::window::WindowId;
use crate::dpi::Size;
use crate::dpi::Position;
use crate::extra::TimeDuration;
use crate::extra::convert::ExFrom;
use crate::extra::convert::ExInto;

use proc::mapping_enum;
use napi::bindgen_prelude::*;

#[derive(Clone)]
pub struct UserPayload {}

mapping_enum!(
    enum Event<UserPayload> {
        NewEvents(StartCause),
        WindowEvent {
            window_id: WindowId,
            event: WindowEvent,
        },
        DeviceEvent {
            device_id: DeviceId,
            event: DeviceEvent,
        },
        UserEvent(#[conf_direct_type] UserPayload),
        Suspended,
        Resumed,
        AboutToWait,
        LoopExiting,
        MemoryWarning,
    }
);

mapping_enum!(
    enum StartCause {
        ResumeTimeReached {
            #[conf_trans_type = TimeDuration] start: Instant,
            #[conf_trans_type = TimeDuration] requested_resume: Instant,
        },
        WaitCancelled {
            #[conf_trans_type = TimeDuration] start: Instant,
            #[conf_trans_type = Option::<TimeDuration>] requested_resume: Option<Instant>,
        },
        Poll,
        Init,
    }
);

mapping_enum!(
    enum WindowEvent {
        ActivationTokenDone {
            serial: AsyncRequestSerial,
            token: ActivationToken,
        },
        Resized(#[conf_trans_type = Size] PhysicalSize<u32>),
        Moved(#[conf_trans_type = Position] PhysicalPosition<i32>),
        CloseRequested,
        Destroyed,
        DroppedFile(#[conf_trans_type = String] PathBuf),
        HoveredFile(#[conf_trans_type = String] PathBuf),
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

mapping_enum!(
    enum DeviceEvent {
        Added,
        Removed,
        MouseMotion {
            #[conf_trans_type = Position] delta: (f64, f64),
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