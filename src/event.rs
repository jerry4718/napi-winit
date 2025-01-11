use winit::event::Event as WinitEvent;

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

// #[napi]
pub struct Event {
  pub event_type: EventType,
  pub payload: WinitEvent<UserPayload>,
}

#[napi]
#[repr(u8)]
pub enum StartCause {
  ResumeTimeReached,
  WaitCancelled,
  Poll,
  Init,
}
