use winit::{event::{
    DeviceEvent as OriginDeviceEvent,
    DeviceId as OriginDeviceId,
    ElementState as OriginElementState,
    Event as OriginEvent,
    Ime as OriginIme,
    InnerSizeWriter as OriginInnerSizeWriter,
    KeyEvent as OriginKeyEvent,
    Modifiers as OriginModifiers,
    MouseButton as OriginMouseButton,
    MouseScrollDelta as OriginMouseScrollDelta,
    RawKeyEvent as OriginRawKeyEvent,
    StartCause as OriginStartCause,
    Touch as OriginTouch,
    TouchPhase as OriginTouchPhase,
    WindowEvent as OriginWindowEvent,
}, keyboard, window::Theme as OriginTheme};

use crate::{
    dpi::{
        Position,
        Size,
    },
    event_loop::AsyncRequestSerial,
    extra::{
        convert::ExInto,
        TimeDuration,
    },
    keyboard::{Key, KeyLocation, ModifiersState, PhysicalKey},
    mark_ex_into,
    string_enum,
    window::{
        ActivationToken,
        Theme,
        WindowId,
    },
    wrap_struct,
};

use napi::bindgen_prelude::*;
use proc::proxy_enum;

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
        start: TimeDuration,
        requested_resume: TimeDuration,
    },
    WaitCancelled {
        start: TimeDuration,
        #[proxy_enum(from_origin = { requested_resume.map(|x| x.into()) })]
        requested_resume: Option<TimeDuration>,
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
    DroppedFile(#[proxy_enum(field_name = "path", from_origin = { path.to_str().unwrap().into() })] String),
    HoveredFile(#[proxy_enum(field_name = "path", from_origin = { path.to_str().unwrap().into() })] String),
    HoveredFileCancelled,
    Focused(bool),
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

wrap_struct!(#[derive(Clone)] struct DeviceId(winit::event::DeviceId));

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

wrap_struct!(#[derive(Clone)] struct KeyEvent { origin: winit::event::KeyEvent });

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
        self.origin.text.clone().map(|text| text.ex_into())
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

wrap_struct!(#[derive(Clone)] struct Modifiers(OriginModifiers));

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
        #[proxy_enum(field_name = "position", from_origin = { position.map(|x| x.into()) })] Option<Position>
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

wrap_struct!(#[derive(Clone)] struct InnerSizeWriter(OriginInnerSizeWriter));

#[proxy_enum(origin_enum = winit::event::TouchPhase, skip_backward)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    Cancelled,
}

wrap_struct!(#[derive(Clone)] struct Touch(OriginTouch));

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

string_enum!(
    #[derive(Clone)]
    enum ElementState => OriginElementState {
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