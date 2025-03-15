use std::ptr::NonNull;
use napi::bindgen_prelude::*;

use winit::{
    event::{
        DeviceEvent as OriginDeviceEvent,
        DeviceId as OriginDeviceId,
        StartCause as OriginStartCause,
        WindowEvent as OriginWindowEvent,
    },
    event_loop::ActiveEventLoop as OriginActiveEventLoop,
    window::WindowId as OriginWindowId,
    application::ApplicationHandler,
};
use crate::{
    application::public::{
        ApplicationSyncOptionRefs,
        ApplicationSyncOptions
    },
    event::{
        DeviceEvent,
        DeviceId,
        StartCause,
        UserPayload,
        WindowEvent
    },
    event_loop::ActiveEventLoop,
    handle_res,
    window::WindowId
};

#[napi]
pub struct ApplicationT3 {
    pub(crate) on_new_events: Option<FunctionRef<FnArgs<(ActiveEventLoop, StartCause)>, ()>>,
    pub(crate) on_resumed: FunctionRef<FnArgs<(ActiveEventLoop, )>, ()>,
    pub(crate) on_user_event: Option<FunctionRef<FnArgs<(ActiveEventLoop, UserPayload)>, ()>>,
    pub(crate) on_window_event: FunctionRef<FnArgs<(ActiveEventLoop, WindowId, WindowEvent)>, ()>,
    pub(crate) on_device_event: Option<FunctionRef<FnArgs<(ActiveEventLoop, DeviceId, DeviceEvent)>, ()>>,
    pub(crate) on_about_to_wait: Option<FunctionRef<FnArgs<(ActiveEventLoop, )>, ()>>,
    pub(crate) on_suspended: Option<FunctionRef<FnArgs<(ActiveEventLoop, )>, ()>>,
    pub(crate) on_exiting: Option<FunctionRef<FnArgs<(ActiveEventLoop, )>, ()>>,
    pub(crate) on_memory_warning: Option<FunctionRef<FnArgs<(ActiveEventLoop, )>, ()>>,
}

#[napi]
impl ApplicationT3 {
    #[napi(constructor)]
    pub fn new(callbacks: ApplicationSyncOptions) -> Self {
        Self {
            on_new_events: callbacks.on_new_events.map(|f| f.create_ref().unwrap()),
            on_resumed: callbacks.on_resumed.create_ref().unwrap(),
            on_user_event: callbacks.on_user_event.map(|f| f.create_ref().unwrap()),
            on_window_event: callbacks.on_window_event.create_ref().unwrap(),
            on_device_event: callbacks.on_device_event.map(|f| f.create_ref().unwrap()),
            on_about_to_wait: callbacks.on_about_to_wait.map(|f| f.create_ref().unwrap()),
            on_suspended: callbacks.on_suspended.map(|f| f.create_ref().unwrap()),
            on_exiting: callbacks.on_exiting.map(|f| f.create_ref().unwrap()),
            on_memory_warning: callbacks.on_memory_warning.map(|f| f.create_ref().unwrap()),
        }
    }
    #[napi(factory)]
    pub fn from_refs(callbacks: ApplicationSyncOptionRefs) -> Self {
        Self {
            on_new_events: callbacks.on_new_events,
            on_resumed: callbacks.on_resumed,
            on_user_event: callbacks.on_user_event,
            on_window_event: callbacks.on_window_event,
            on_device_event: callbacks.on_device_event,
            on_about_to_wait: callbacks.on_about_to_wait,
            on_suspended: callbacks.on_suspended,
            on_exiting: callbacks.on_exiting,
            on_memory_warning: callbacks.on_memory_warning,
        }
    }
}

pub struct ApplicationT3Runner<'scope> {
    pub on_new_events: Option<Function<'scope, FnArgs<(ActiveEventLoop, StartCause)>, ()>>,
    pub on_resumed: Function<'scope, FnArgs<(ActiveEventLoop, )>, ()>,
    pub on_user_event: Option<Function<'scope, FnArgs<(ActiveEventLoop, UserPayload)>, ()>>,
    pub on_window_event: Function<'scope, FnArgs<(ActiveEventLoop, WindowId, WindowEvent)>, ()>,
    pub on_device_event: Option<Function<'scope, FnArgs<(ActiveEventLoop, DeviceId, DeviceEvent)>, ()>>,
    pub on_about_to_wait: Option<Function<'scope, FnArgs<(ActiveEventLoop, )>, ()>>,
    pub on_suspended: Option<Function<'scope, FnArgs<(ActiveEventLoop, )>, ()>>,
    pub on_exiting: Option<Function<'scope, FnArgs<(ActiveEventLoop, )>, ()>>,
    pub on_memory_warning: Option<Function<'scope, FnArgs<(ActiveEventLoop, )>, ()>>,
}

macro_rules! wrap_event_loop {
    ($name: expr) => { ActiveEventLoop { inner_non_null: NonNull::new($name as *const _ as *mut OriginActiveEventLoop).unwrap() } };
}

impl<'a> ApplicationHandler<UserPayload> for ApplicationT3Runner<'a> {
    fn new_events(&mut self, event_loop: &OriginActiveEventLoop, cause: OriginStartCause) {
        let Self { on_new_events: Some(callback), .. } = &self else { return; };
        let result = callback.call(FnArgs::from((wrap_event_loop!(event_loop), StartCause::from(cause))));
        handle_res!(result);
    }

    fn resumed(&mut self, event_loop: &OriginActiveEventLoop) {
        let Self { on_resumed: callback, .. } = &self;
        let result = callback.call(FnArgs::from((wrap_event_loop!(event_loop),)));
        handle_res!(result);
    }

    fn user_event(&mut self, event_loop: &OriginActiveEventLoop, event: UserPayload) {
        let Self { on_user_event: Some(callback), .. } = &self else { return; };
        let result = callback.call(FnArgs::from((wrap_event_loop!(event_loop), event)));
        handle_res!(result);
    }

    fn window_event(&mut self, event_loop: &OriginActiveEventLoop, window_id: OriginWindowId, event: OriginWindowEvent) {
        let Self { on_window_event: callback, .. } = &self;
        let result = callback.call(FnArgs::from((wrap_event_loop!(event_loop), WindowId::from(window_id), WindowEvent::from(event))));
        handle_res!(result);
    }

    fn device_event(&mut self, event_loop: &OriginActiveEventLoop, device_id: OriginDeviceId, event: OriginDeviceEvent) {
        let Self { on_device_event: Some(callback), .. } = &self else { return; };
        let result = callback.call(FnArgs::from((wrap_event_loop!(event_loop), DeviceId::from(device_id), DeviceEvent::from(event))));
        handle_res!(result);
    }

    fn about_to_wait(&mut self, event_loop: &OriginActiveEventLoop) {
        let Self { on_about_to_wait: Some(callback), .. } = &self else { return; };
        let result = callback.call(FnArgs::from((wrap_event_loop!(event_loop),)));
        handle_res!(result);
    }

    fn suspended(&mut self, event_loop: &OriginActiveEventLoop) {
        let Self { on_suspended: Some(callback), .. } = &self else { return; };
        let result = callback.call(FnArgs::from((wrap_event_loop!(event_loop),)));
        handle_res!(result);
    }

    fn exiting(&mut self, event_loop: &OriginActiveEventLoop) {
        let Self { on_exiting: Some(callback), .. } = &self else { return; };
        let result = callback.call(FnArgs::from((wrap_event_loop!(event_loop),)));
        handle_res!(result);
    }

    fn memory_warning(&mut self, event_loop: &OriginActiveEventLoop) {
        let Self { on_memory_warning: Some(callback), .. } = &self else { return; };
        let result = callback.call(FnArgs::from((wrap_event_loop!(event_loop),)));
        handle_res!(result);
    }
}