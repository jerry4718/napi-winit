use winit::event::Event;

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

// #[napi]
pub struct JsEvent {
  pub event_type: JsEventType,
  pub payload: Event<UserPayload>,
}

#[napi(js_name = "StartCause")]
#[repr(u8)]
pub enum JsStartCause {
  ResumeTimeReached,
  WaitCancelled,
  Poll,
  Init,
}

struct JsStartCauseResumeTimeReachedData {

}
struct JsStartCauseWaitCancelledData {

}
