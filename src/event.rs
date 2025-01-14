use winit::event::{
    DeviceEvent as WDeviceEvent,
    DeviceId as WDeviceId,
    Event as WEvent,
    StartCause as WStartCause,
    WindowEvent as WWindowEvent,
};
use winit::window::WindowId as WWindowId;

use crate::extra::TimeDuration;
use napi::bindgen_prelude::*;

pub struct UserPayload {}

macro_rules! accessor_not_exist {
    ($x: literal, $( $p: path ),*) => {
        stringify!(accessor [$x] only exist on $( $p ) or *)
    };
}

#[napi]
#[repr(u8)]
pub enum EventType {
    NewEvents,
    WindowEvent,
    DeviceEvent,
    UserEvent,
    Suspended,
    Resumed,
    AboutToWait,
    LoopExiting,
    MemoryWarning,
}

#[napi]
pub struct Event {
    pub(crate) inner: WEvent<UserPayload>,
}

impl From<WEvent<UserPayload>> for Event {
    fn from(value: WEvent<UserPayload>) -> Self {
        Self { inner: value }
    }
}

#[napi]
impl Event {
    #[napi(getter, js_name = "type")]
    pub fn event_type(&self) -> Result<EventType> {
        match self.inner {
            WEvent::NewEvents(_) => Ok(EventType::NewEvents),
            WEvent::WindowEvent { .. } => Ok(EventType::WindowEvent),
            WEvent::DeviceEvent { .. } => Ok(EventType::DeviceEvent),
            WEvent::UserEvent(_) => Ok(EventType::UserEvent),
            WEvent::Suspended => Ok(EventType::Suspended),
            WEvent::Resumed => Ok(EventType::Resumed),
            WEvent::AboutToWait => Ok(EventType::AboutToWait),
            WEvent::LoopExiting => Ok(EventType::LoopExiting),
            WEvent::MemoryWarning => Ok(EventType::MemoryWarning),
        }
    }

    #[napi(getter)]
    pub fn start_cause(&self) -> Result<StartCause> {
        match self.inner {
            WEvent::NewEvents(start_cause) => Ok(StartCause { inner: start_cause }),
            _ => Err(Error::from_reason(accessor_not_exist!("start_cause", Event::NewEvents))),
        }
    }
}

#[napi]
#[repr(u8)]
pub enum StartCauseType {
    ResumeTimeReached,
    WaitCancelled,
    Poll,
    Init,
}

#[napi]
pub struct StartCause {
    pub(crate) inner: WStartCause,
}

#[napi]
impl StartCause {
    #[napi(getter, js_name = "type")]
    pub fn start_cause_type(&self) -> Result<StartCauseType> {
        match self.inner {
            WStartCause::ResumeTimeReached { .. } => Ok(StartCauseType::ResumeTimeReached),
            WStartCause::WaitCancelled { .. } => Ok(StartCauseType::WaitCancelled),
            WStartCause::Poll => Ok(StartCauseType::Poll),
            WStartCause::Init => Ok(StartCauseType::Init),
        }
    }
    #[napi(getter)]
    pub fn start(&self) -> Result<TimeDuration> {
        match self.inner {
            WStartCause::ResumeTimeReached { start, .. } => Ok(TimeDuration::from(start)),
            WStartCause::WaitCancelled { start, .. } => Ok(TimeDuration::from(start)),
            _ => Err(Error::from_reason("accessor [start] only exist on start_cause::resume_time_reached or start_cause::wait_cancelled")),
        }
    }
    #[napi(getter)]
    pub fn requested_resume(&self) -> Result<Option<TimeDuration>> {
        match self.inner {
            WStartCause::ResumeTimeReached { requested_resume, .. } => {
                Ok(Some(TimeDuration::from(requested_resume)))
            }
            WStartCause::WaitCancelled { requested_resume, .. } => {
                match requested_resume {
                    Some(requested_resume) => Ok(Some(TimeDuration::from(requested_resume))),
                    None => Ok(None),
                }
            }
            _ => Err(Error::from_reason("accessor [requested_resume] only exist on start_cause::resume_time_reached or start_cause::wait_cancelled")),
        }
    }
}

#[napi]
pub struct WindowPayload {
    pub(crate) window_id: WWindowId,
    pub(crate) event: WWindowEvent,
}

#[napi]
pub struct DevicePayload {
    pub(crate) device_id: WDeviceId,
    pub(crate) event: WDeviceEvent,
}

#[napi]
pub struct WindowId {
    pub(crate) inner: WWindowId,
}

impl From<WWindowId> for WindowId {
    fn from(value: WWindowId) -> Self {
        Self { inner: value }
    }
}

#[napi]
pub struct WindowEvent {
    pub(crate) inner: WWindowEvent,
}

#[napi]
pub enum WindowEventType {
    // ActivationTokenDone,
    Resized,
    Moved,
    CloseRequested,
    Destroyed,
    DroppedFile,
    HoveredFile,
    HoveredFileCancelled,
    Focused,
    KeyboardInput,
    ModifiersChanged,
    Ime,
    CursorMoved,
    CursorEntered,
    CursorLeft,
    MouseWheel,
    MouseInput,
    PinchGesture,
    PanGesture,
    DoubleTapGesture,
    RotationGesture,
    TouchpadPressure,
    AxisMotion,
    Touch,
    ScaleFactorChanged,
    ThemeChanged,
    Occluded,
    RedrawRequested,
}

impl From<WWindowEvent> for WindowEvent {
    fn from(value: WWindowEvent) -> Self {
        Self { inner: value }
    }
}

impl WindowEvent {
    pub fn get_type(&self) -> WindowEventType {
        match self.inner {
            WWindowEvent::ActivationTokenDone { .. } => unimplemented!("WindowEventType::ActivationTokenDone has not mapping"), // WindowEventType::ActivationTokenDone,
            WWindowEvent::Resized(_) => WindowEventType::Resized,
            WWindowEvent::Moved(_) => WindowEventType::Moved,
            WWindowEvent::CloseRequested => WindowEventType::CloseRequested,
            WWindowEvent::Destroyed => WindowEventType::Destroyed,
            WWindowEvent::DroppedFile(_) => WindowEventType::DroppedFile,
            WWindowEvent::HoveredFile(_) => WindowEventType::HoveredFile,
            WWindowEvent::HoveredFileCancelled => WindowEventType::HoveredFileCancelled,
            WWindowEvent::Focused(_) => WindowEventType::Focused,
            WWindowEvent::KeyboardInput { .. } => WindowEventType::KeyboardInput,
            WWindowEvent::ModifiersChanged(_) => WindowEventType::ModifiersChanged,
            WWindowEvent::Ime(_) => WindowEventType::Ime,
            WWindowEvent::CursorMoved { .. } => WindowEventType::CursorMoved,
            WWindowEvent::CursorEntered { .. } => WindowEventType::CursorEntered,
            WWindowEvent::CursorLeft { .. } => WindowEventType::CursorLeft,
            WWindowEvent::MouseWheel { .. } => WindowEventType::MouseWheel,
            WWindowEvent::MouseInput { .. } => WindowEventType::MouseInput,
            WWindowEvent::PinchGesture { .. } => WindowEventType::PinchGesture,
            WWindowEvent::PanGesture { .. } => WindowEventType::PanGesture,
            WWindowEvent::DoubleTapGesture { .. } => WindowEventType::DoubleTapGesture,
            WWindowEvent::RotationGesture { .. } => WindowEventType::RotationGesture,
            WWindowEvent::TouchpadPressure { .. } => WindowEventType::TouchpadPressure,
            WWindowEvent::AxisMotion { .. } => WindowEventType::AxisMotion,
            WWindowEvent::Touch(_) => WindowEventType::Touch,
            WWindowEvent::ScaleFactorChanged { .. } => WindowEventType::ScaleFactorChanged,
            WWindowEvent::ThemeChanged(_) => WindowEventType::ThemeChanged,
            WWindowEvent::Occluded(_) => WindowEventType::Occluded,
            WWindowEvent::RedrawRequested => WindowEventType::RedrawRequested,
        }
    }
}


