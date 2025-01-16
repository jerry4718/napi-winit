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
        Touch as OriginTouch,
        InnerSizeWriter as OriginInnerSizeWriter,
        RawKeyEvent as OriginRawKeyEvent,
    },
    window::{
        WindowId as OriginWindowId,
        ActivationToken as OriginActivationToken,
        Theme as OriginTheme
    },
    event_loop::AsyncRequestSerial as OriginAsyncRequestSerial
};

use crate::{
    event_loop::AsyncRequestSerial,
    extra::{
        convert::ExInto,
        TimeDuration,
    },
    dpi::{
        Position,
        Size,
    },
    window::{
        WindowId,
        Theme,
        ActivationToken,
    },
    mark_ex_into,
};

use proc::{mapping_enum, simple_enum, simple_struct};
use napi::bindgen_prelude::*;

#[napi]
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
            #[conf_trans_type = u32] axis: AxisId,
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

simple_struct!(DeviceId);
simple_struct!(RawKeyEvent);
simple_struct!(KeyEvent);
simple_struct!(Modifiers);

mapping_enum!(
    enum Ime {
        Enabled,
        Preedit(#[conf_direct_type] String, #[conf_trans_type = Option::<Position>] Option<(usize, usize)>),
        Commit(#[conf_direct_type] String),
        Disabled,
    }
);

mapping_enum!(
    enum MouseButton {
        Left,
        Right,
        Middle,
        Back,
        Forward,
        Other(u16),
    }
);

#[napi(string_enum)]
pub enum MouseScrollDeltaType {
    Line,
    Pixel,
}

#[napi]
#[derive(Clone)]
pub struct MouseScrollDelta {
    delta_type: MouseScrollDeltaType,
    delta: Position,
}

impl From<OriginMouseScrollDelta> for MouseScrollDelta {
    fn from(value: OriginMouseScrollDelta) -> Self {
        match value {
            OriginMouseScrollDelta::LineDelta(x, y) => {
                Self {
                    delta_type: MouseScrollDeltaType::Line,
                    delta: Position::from((f64::from(x), f64::from(y)))
                }
            }
            OriginMouseScrollDelta::PixelDelta(position) => {
                Self {
                    delta_type: MouseScrollDeltaType::Pixel,
                    delta: Position::from(position)
                }
            }
        }
    }
}

simple_struct!(InnerSizeWriter);

mapping_enum!(
    enum TouchPhase {
        Started,
        Moved,
        Ended,
        Cancelled,
    }
);

simple_struct!(Touch);

mapping_enum!(
    enum DeviceEvent {
        Added,
        Removed,
        MouseMotion {
            #[conf_trans_type = Position] delta: (f64, f64),
        },
        MouseWheel {
            #[conf_trans_type = MouseScrollDelta] delta: MouseScrollDelta,
        },
        Motion {
            #[conf_trans_type = u32] axis: AxisId,
            value: f64,
        },
        Button {
            #[conf_trans_type = u32] button: ButtonId,
            state: ElementState,
        },
        Key(RawKeyEvent),
    }
);

simple_enum!(
    enum ElementState {
        Pressed,
        Released,
    }
);

mark_ex_into!(
    OriginDeviceEvent,
    OriginDeviceId,
    OriginEvent<UserPayload>,
    OriginStartCause,
    OriginWindowEvent,
    OriginKeyEvent,
    OriginModifiers,
    OriginIme,
    OriginMouseScrollDelta,
    OriginTouchPhase,
    OriginElementState,
    OriginMouseButton,
    OriginTouch,
    OriginInnerSizeWriter,
    OriginRawKeyEvent,
    OriginTheme,
    // local
    UserPayload,
    Event,
    StartCause,
    WindowEvent,
    DeviceId,
    RawKeyEvent,
    KeyEvent,
    Modifiers,
    Ime,
    MouseButton,
    MouseScrollDelta,
    InnerSizeWriter,
    TouchPhase,
    Touch,
    DeviceEvent,
    ElementState
);