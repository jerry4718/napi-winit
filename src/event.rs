use napi::bindgen_prelude::*;

use proc::{proxy_enum, proxy_struct, proxy_wrap};

use crate::{
    dpi::{Position, Size},
    event_loop::AsyncRequestSerial,
    extra::time::Instant,
    keyboard::{Key, KeyLocation, ModifiersState, PhysicalKey},
    utils::helpers::{option_into, path_buf_to_string},
    window::{ActivationToken, Theme, WindowId},
};

#[napi]
#[derive(Clone)]
pub struct UserPayload {}

#[proxy_enum(origin_type = winit::event::Event::<UserPayload>, skip_backward)]
pub enum Event {
    NewEvents(#[proxy_enum(field_name = cause)] StartCause),
    WindowEvent {
        window_id: WindowId,
        event: WindowEvent,
    },
    DeviceEvent {
        device_id: DeviceId,
        event: DeviceEvent,
    },
    UserEvent(#[proxy_enum(field_name = payload)] UserPayload),
    Suspended,
    Resumed,
    AboutToWait,
    LoopExiting,
    MemoryWarning,
}

/** [winit::event::StartCause] */
#[proxy_enum(origin_type = winit::event::StartCause, skip_backward)]
pub enum StartCause {
    ResumeTimeReached {
        start: Instant,
        requested_resume: Instant,
    },
    WaitCancelled {
        start: Instant,
        #[proxy_enum(from_origin = option_into)]
        requested_resume: Option<Instant>,
    },
    Poll,
    Init,
}

#[proxy_enum(origin_type = winit::event::WindowEvent, skip_backward)]
pub enum WindowEvent {
    ActivationTokenDone {
        serial: AsyncRequestSerial,
        token: ActivationToken,
    },
    Resized(#[proxy_enum(field_name = size)] Size),
    Moved(#[proxy_enum(field_name = position)] Position),
    CloseRequested,
    Destroyed,
    DroppedFile(#[proxy_enum(field_name = path, from_origin = path_buf_to_string)] String),
    HoveredFile(#[proxy_enum(field_name = path, from_origin = path_buf_to_string)] String),
    HoveredFileCancelled,
    Focused(#[proxy_enum(field_name = focused)] bool),
    KeyboardInput {
        device_id: DeviceId,
        event: KeyEvent,
        is_synthetic: bool,
    },
    ModifiersChanged(#[proxy_enum(field_name = modifiers)] Modifiers),
    Ime(#[proxy_enum(field_name = ime)] Ime),
    CursorMoved {
        device_id: DeviceId,
        position: Position,
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
        delta: Position,
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
        axis: u32,
        value: f64,
    },
    Touch(#[proxy_enum(field_name = touch)] Touch),
    ScaleFactorChanged {
        scale_factor: f64,
        inner_size_writer: InnerSizeWriter,
    },
    ThemeChanged(#[proxy_enum(field_name = theme)] Theme),
    Occluded(#[proxy_enum(field_name = occluded)] bool),
    RedrawRequested,
}

/**[winit::event::DeviceId]*/
#[proxy_wrap(origin_type = winit::event::DeviceId)]
pub struct DeviceId;

/**[winit::event::RawKeyEvent]*/
#[proxy_wrap(origin_type = winit::event::RawKeyEvent, skip_into_origin, no_setter)]
pub struct RawKeyEvent {
    pub physical_key: PhysicalKey,
    pub state: ElementState,
}

/** [winit::event::KeyEvent] */
#[proxy_wrap(origin_type = winit::event::KeyEvent, skip_into_origin, no_setter)]
pub struct KeyEvent {
    pub physical_key: PhysicalKey,

    #[proxy_wrap(get_ref, conv_get = [Clone::clone, Into::into])]
    pub logical_key: Key,

    #[proxy_wrap(get_ref, conv_get = [Clone::clone, option_into])]
    pub text: Option<String>,

    pub location: KeyLocation,

    pub state: ElementState,

    pub repeat: bool,
}

/** [winit::event::Modifiers] */
#[proxy_wrap(origin_type = winit::event::Modifiers)]
pub struct Modifiers;

#[napi]
impl Modifiers {
    #[napi(getter)]
    pub fn state(&self) -> ModifiersState {
        self.0.state().into()
    }
}

#[proxy_enum(origin_type = winit::event::Ime, skip_backward)]
pub enum Ime {
    Enabled,
    Preedit(
        #[proxy_enum(field_name = preedit)] String,
        #[proxy_enum(field_name = position, from_origin = option_into)] Option<Position>,
    ),
    Commit(#[proxy_enum(field_name = commit)] String),
    Disabled,
}

#[proxy_enum(origin_type = winit::event::MouseButton, skip_backward)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
    Other(u16),
}

#[proxy_enum(origin_type = winit::event::MouseScrollDelta, skip_backward)]
pub enum MouseScrollDelta {
    LineDelta(#[proxy_enum(field_name = x)] f64, #[proxy_enum(field_name = y)] f64),
    PixelDelta(#[proxy_enum(field_name = delta)] Position),
}

/**[winit::event::InnerSizeWriter]*/
#[proxy_wrap(origin_type = winit::event::InnerSizeWriter)]
pub struct InnerSizeWriter;

#[proxy_enum(origin_type = winit::event::TouchPhase, string_enum, skip_backward)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    Cancelled,
}

/**[winit::event::Touch]*/
#[proxy_struct(origin_type = winit::event::Touch, object, skip_backward)]
pub struct Touch {
    pub device_id: DeviceId,
    pub phase: TouchPhase,
    pub location: Position,
    /// Describes how hard the screen was pressed. May be `None` if the platform
    /// does not support pressure sensitivity.
    ///
    /// ## Platform-specific
    ///
    /// - Only available on **iOS** 9.0+, **Windows** 8+, **Web**, and **Android**.
    /// - **Android**: This will never be [None]. If the device doesn't support pressure
    ///   sensitivity, force will either be 0.0 or 1.0. Also see the
    ///   [android documentation](https://developer.android.com/reference/android/view/MotionEvent#AXIS_PRESSURE).
    #[proxy_struct(from_origin = option_into)]
    pub force: Option<Force>,
    /// Unique identifier of a finger.
    pub id: u64,
}

#[proxy_enum(origin_type = winit::event::Force)]
pub enum Force {
    /// On iOS, the force is calibrated so that the same number corresponds to
    /// roughly the same amount of pressure on the screen regardless of the
    /// device.
    Calibrated {
        /// The force of the touch, where a value of 1.0 represents the force of
        /// an average touch (predetermined by the system, not user-specific).
        ///
        /// The force reported by Apple Pencil is measured along the axis of the
        /// pencil. If you want a force perpendicular to the device, you need to
        /// calculate this value using the `altitude_angle` value.
        force: f64,
        /// The maximum possible force for a touch.
        ///
        /// The value of this field is sufficiently high to provide a wide
        /// dynamic range for values of the `force` field.
        max_possible_force: f64,
        /// The altitude (in radians) of the stylus.
        ///
        /// A value of 0 radians indicates that the stylus is parallel to the
        /// surface. The value of this property is Pi/2 when the stylus is
        /// perpendicular to the surface.
        altitude_angle: Option<f64>,
    },
    /// If the platform reports the force as normalized, we have no way of
    /// knowing how much pressure 1.0 corresponds to – we know it's the maximum
    /// amount of force, but as to how much force, you might either have to
    /// press really really hard, or not hard at all, depending on the device.
    Normalized(#[proxy_enum(field_name = value)] f64),
}

#[proxy_enum(origin_type = winit::event::DeviceEvent, skip_backward)]
pub enum DeviceEvent {
    Added,
    Removed,
    MouseMotion {
        delta: Position,
    },
    MouseWheel {
        delta: MouseScrollDelta,
    },
    Motion {
        axis: u32,
        value: f64,
    },
    Button {
        button: u32,
        state: ElementState,
    },
    Key(#[proxy_enum(field_name = raw)] RawKeyEvent),
}

#[proxy_enum(origin_type = winit::event::ElementState, string_enum, skip_backward)]
#[derive(Clone)]
pub enum ElementState {
    Pressed,
    Released,
}