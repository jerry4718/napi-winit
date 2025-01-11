use winit::event::{
    DeviceEvent as WDeviceEvent,
    DeviceId as WDeviceId,
    Event as WEvent,
    StartCause as WStartCause,
    WindowEvent as WWindowEvent
};

use napi::bindgen_prelude::*;
use winit::window::WindowId;
use crate::js::TimeDuration;

pub struct UserPayload {}

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
            _ => Err(Error::from_reason("accessor [start_cause] only exist on Event::NewEvents")),
        }
    }

    // pub fn window_payload(&self) -> Result<WindowPayload> {
    //     match self.inner {
    //         WEvent::WindowEvent { window_id, event } => Ok(WindowPayload { window_id, event }),
    //         _ => Err(Error::from_reason("accessor [window_payload] only exist on Event::WindowEvent")),
    //     }
    // }
    // pub fn device_payload(&self) -> Result<DevicePayload> {
    //     match self.inner {
    //         WEvent::DeviceEvent { device_id, event } => Ok(DevicePayload { device_id, event }),
    //         _ => Err(Error::from_reason("accessor [device_payload] only exist on Event::DeviceEvent")),
    //     }
    // }
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
            _ => Err(Error::from_reason("accessor [start] only exist on StartCause::ResumeTimeReached or StartCause::WaitCancelled")),
        }
    }
    #[napi(getter)]
    pub fn requested_resume(&self) -> Result<Option<TimeDuration>> {
        match self.inner {
            WStartCause::ResumeTimeReached { requested_resume, .. } => {
                Ok(Some(TimeDuration::from(requested_resume)))
            },
            WStartCause::WaitCancelled { requested_resume, .. } => {
                match requested_resume {
                    Some(requested_resume) => Ok(Some(TimeDuration::from(requested_resume))),
                    None => Ok(None),
                }
            },
            _ => Err(Error::from_reason("accessor [requested_resume] only exist on StartCause::ResumeTimeReached or StartCause::WaitCancelled")),
        }
    }
}

#[napi]
pub struct WindowPayload {
    pub(crate) window_id: WindowId,
    pub(crate) event: WWindowEvent,
}
#[napi]
pub struct DevicePayload {
    pub(crate) device_id: WDeviceId,
    pub(crate) event: WDeviceEvent,
}
