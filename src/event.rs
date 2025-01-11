use winit::event::{DeviceEvent, DeviceId, Event, StartCause, WindowEvent};
use napi::bindgen_prelude::*;
use winit::window::WindowId;
use crate::js::JsTimeDuration;

pub struct UserPayload {}

#[napi(js_name = "EventType")]
#[repr(u8)]
pub enum JsEventType {
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

#[napi(js_name = "Event")]
pub struct JsEvent {
    pub(crate) inner: Event<UserPayload>,
}

impl From<Event<UserPayload>> for JsEvent {
    fn from(value: Event<UserPayload>) -> Self {
        Self { inner: value }
    }
}

#[napi]
impl JsEvent {
    #[napi(getter, js_name = "type", ts_return_type = "EventType")]
    pub fn event_type(&self) -> Result<JsEventType> {
        match self.inner {
            Event::NewEvents(_) => Ok(JsEventType::NewEvents),
            Event::WindowEvent { .. } => Ok(JsEventType::WindowEvent),
            Event::DeviceEvent { .. } => Ok(JsEventType::DeviceEvent),
            Event::UserEvent(_) => Ok(JsEventType::UserEvent),
            Event::Suspended => Ok(JsEventType::Suspended),
            Event::Resumed => Ok(JsEventType::Resumed),
            Event::AboutToWait => Ok(JsEventType::AboutToWait),
            Event::LoopExiting => Ok(JsEventType::LoopExiting),
            Event::MemoryWarning => Ok(JsEventType::MemoryWarning),
        }
    }

    // #[napi(getter, ts_return_type = "StartCause")]
    pub fn start_cause(&self) -> Result<JsStartCause> {
        match self.inner {
            Event::NewEvents(start_cause) => Ok(JsStartCause { inner: start_cause }),
            _ => Err(Error::from_reason("accessor [start_cause] only exist on Event::NewEvents")),
        }
    }

    pub fn window_payload(&self) -> Result<WindowPayload> {
        match self.inner {
            Event::WindowEvent { window_id, event } => Ok(WindowPayload { window_id, event }),
            _ => Err(Error::from_reason("accessor [window_payload] only exist on Event::WindowEvent")),
        }
    }
    pub fn device_payload(&self) -> Result<DevicePayload> {
        match self.inner {
            Event::DeviceEvent { device_id, event } => Ok(DevicePayload { device_id, event }),
            _ => Err(Error::from_reason("accessor [device_payload] only exist on Event::DeviceEvent")),
        }
    }
}

#[napi(js_name = "StartCauseType")]
#[repr(u8)]
pub enum JsStartCauseType {
    ResumeTimeReached,
    WaitCancelled,
    Poll,
    Init,
}

#[napi(js_name = "StartCause")]
pub struct JsStartCause {
    pub(crate) inner: StartCause,
}

#[napi]
impl JsStartCause {
    #[napi(getter, js_name = "type", ts_return_type = "StartCauseType")]
    pub fn start_cause_type(&self) -> Result<JsStartCauseType> {
        match self.inner {
            StartCause::ResumeTimeReached { .. } => Ok(JsStartCauseType::ResumeTimeReached),
            StartCause::WaitCancelled { .. } => Ok(JsStartCauseType::WaitCancelled),
            StartCause::Poll => Ok(JsStartCauseType::Poll),
            StartCause::Init => Ok(JsStartCauseType::Init),
        }
    }
    #[napi(getter, ts_return_type = "TimeDuration")]
    pub fn start(&self) -> Result<JsTimeDuration> {
        match self.inner {
            StartCause::ResumeTimeReached { start, .. } => Ok(JsTimeDuration::from(start)),
            StartCause::WaitCancelled { start, .. } => Ok(JsTimeDuration::from(start)),
            _ => Err(Error::from_reason("accessor [start] only exist on StartCause::ResumeTimeReached or StartCause::WaitCancelled")),
        }
    }
    #[napi(getter)]
    pub fn requested_resume(&self) -> Result<Option<JsTimeDuration>> {
        match self.inner {
            StartCause::ResumeTimeReached { requested_resume, .. } => {
                Ok(Some(JsTimeDuration::from(requested_resume)))
            },
            StartCause::WaitCancelled { requested_resume, .. } => {
                match requested_resume {
                    Some(requested_resume) => Ok(Some(JsTimeDuration::from(requested_resume))),
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
    pub(crate) event: WindowEvent,
}
#[napi]
pub struct DevicePayload {
    pub(crate) device_id: DeviceId,
    pub(crate) event: DeviceEvent,
}
