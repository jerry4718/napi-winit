use napi::bindgen_prelude::*;

use proc::proxy_enum;

use crate::{
    dpi::{Position, Size},
    event_loop::AsyncRequestSerial,
    extra::time::Timeout,
    keyboard::{Key, KeyLocation, ModifiersState, PhysicalKey},
    utils::helpers::{option_into, path_buf_to_string},
    window::{ActivationToken, Theme, WindowId},
    wrap_struct,
};

#[napi]
#[derive(Clone)]
pub struct UserPayload {}

#[proxy_enum(origin_enum = winit::event::Event::<UserPayload>, skip_backward)]
pub enum Event {
    NewEvents(#[proxy_enum(field_name = "cause")] StartCause),
    WindowEvent {
        window_id: WindowId,
        event: WindowEvent,
    },
    DeviceEvent {
        device_id: DeviceId,
        event: DeviceEvent,
    },
    UserEvent(#[proxy_enum(field_name = "payload")] UserPayload),
    Suspended,
    Resumed,
    AboutToWait,
    LoopExiting,
    MemoryWarning,
}

#[proxy_enum(origin_enum = winit::event::StartCause, skip_backward)]
pub enum StartCause {
    ResumeTimeReached {
        start: Timeout,
        requested_resume: Timeout,
    },
    WaitCancelled {
        start: Timeout,
        #[proxy_enum(from_origin = option_into)]
        requested_resume: Option<Timeout>,
    },
    Poll,
    Init,
}

#[proxy_enum(origin_enum = winit::event::WindowEvent, skip_backward)]
pub enum WindowEvent {
    ActivationTokenDone {
        serial: AsyncRequestSerial,
        token: ActivationToken,
    },
    Resized(#[proxy_enum(field_name = "size")] Size),
    Moved(#[proxy_enum(field_name = "position")] Position),
    CloseRequested,
    Destroyed,
    DroppedFile(#[proxy_enum(field_name = "path", from_origin = path_buf_to_string)] String),
    HoveredFile(#[proxy_enum(field_name = "path", from_origin = path_buf_to_string)] String),
    HoveredFileCancelled,
    Focused(#[proxy_enum(field_name = "focused")] bool),
    KeyboardInput {
        device_id: DeviceId,
        event: KeyEvent,
        is_synthetic: bool,
    },
    ModifiersChanged(#[proxy_enum(field_name = "modifiers")] Modifiers),
    Ime(#[proxy_enum(field_name = "ime")] Ime),
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
    Touch(#[proxy_enum(field_name = "touch")] Touch),
    ScaleFactorChanged {
        scale_factor: f64,
        inner_size_writer: InnerSizeWriter,
    },
    ThemeChanged(#[proxy_enum(field_name = "theme")] Theme),
    Occluded(#[proxy_enum(field_name = "occluded")] bool),
    RedrawRequested,
}

wrap_struct!(struct DeviceId(winit::event::DeviceId));

#[napi(object, object_from_js = false)]
pub struct RawKeyEvent {
    pub physical_key: PhysicalKey,
    pub state: ElementState,
}

impl From<winit::event::RawKeyEvent> for RawKeyEvent {
    fn from(raw: winit::event::RawKeyEvent) -> Self {
        let winit::event::RawKeyEvent { physical_key, state } = raw;
        Self {
            physical_key: physical_key.into(),
            state: state.into(),
        }
    }
}

wrap_struct!(struct KeyEvent { origin: winit::event::KeyEvent });

#[napi]
impl KeyEvent {
    #[napi(getter)]
    pub fn physical_key(&self) -> PhysicalKey {
        self.origin.physical_key.into()
    }
    #[napi(getter)]
    pub fn logical_key(&self) -> Key {
        self.origin.logical_key.clone().into()
    }
    #[napi(getter)]
    pub fn text(&self) -> Option<String> {
        self.origin.text.clone().map(String::from)
    }
    #[napi(getter)]
    pub fn location(&self) -> KeyLocation {
        self.origin.location.into()
    }
    #[napi(getter)]
    pub fn state(&self) -> ElementState {
        self.origin.state.into()
    }
    #[napi(getter)]
    pub fn repeat(&self) -> bool {
        self.origin.repeat
    }
}

wrap_struct!(struct Modifiers(winit::event::Modifiers));

#[napi]
impl Modifiers {
    #[napi(getter)]
    pub fn state(&self) -> ModifiersState {
        self.0.state().into()
    }
}

#[proxy_enum(origin_enum = winit::event::Ime, skip_backward)]
pub enum Ime {
    Enabled,
    Preedit(
        #[proxy_enum(field_name = "preedit")] String,
        #[proxy_enum(field_name = "position", from_origin = option_into)] Option<Position>
    ),
    Commit(#[proxy_enum(field_name = "commit")] String),
    Disabled,
}

#[proxy_enum(origin_enum = winit::event::MouseButton, skip_backward)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
    Other(u16),
}

#[proxy_enum(origin_enum = winit::event::MouseScrollDelta, skip_backward)]
pub enum MouseScrollDelta {
    LineDelta(#[proxy_enum(field_name = "x")] f64, #[proxy_enum(field_name = "y")] f64),
    PixelDelta(#[proxy_enum(field_name = "delta")] Position),
}

wrap_struct!(struct InnerSizeWriter(winit::event::InnerSizeWriter));

#[proxy_enum(origin_enum = winit::event::TouchPhase, string_enum, skip_backward)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    Cancelled,
}

wrap_struct!(struct Touch(winit::event::Touch));

#[proxy_enum(origin_enum = winit::event::DeviceEvent, skip_backward)]
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
    Key(#[proxy_enum(field_name = "raw")] RawKeyEvent),
}

#[proxy_enum(origin_enum = winit::event::ElementState, skip_backward)]
#[derive(Clone)]
pub enum ElementState {
    Pressed,
    Released,
}