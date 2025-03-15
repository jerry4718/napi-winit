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
use crate::{application::public::{
    ApplicationSyncOptionRefs,
    ApplicationSyncOptions,
}, event::{
    DeviceEvent,
    DeviceId,
    StartCause,
    UserPayload,
    WindowEvent,
}, event_loop::ActiveEventLoop, handle_res, window::WindowId};

#[napi]
pub struct ApplicationT4 {
    pub(crate) env: Env,
    pub(crate) on_new_events: Option<FunctionRef<FnArgs<(ActiveEventLoop, StartCause)>, ()>>,
    pub(crate) on_resumed: FunctionRef<FnArgs<(ActiveEventLoop,)>, ()>,
    pub(crate) on_user_event: Option<FunctionRef<FnArgs<(ActiveEventLoop, UserPayload)>, ()>>,
    pub(crate) on_window_event: FunctionRef<FnArgs<(ActiveEventLoop, WindowId, WindowEvent)>, ()>,
    pub(crate) on_device_event: Option<FunctionRef<FnArgs<(ActiveEventLoop, DeviceId, DeviceEvent)>, ()>>,
    pub(crate) on_about_to_wait: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, ()>>,
    pub(crate) on_suspended: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, ()>>,
    pub(crate) on_exiting: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, ()>>,
    pub(crate) on_memory_warning: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, ()>>,
}

#[napi]
impl ApplicationT4 {
    #[napi(constructor)]
    pub fn new(env: Env, callbacks: ApplicationSyncOptions) -> Self {
        Self {
            env,
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
    pub fn from_refs(env: Env, callbacks: ApplicationSyncOptionRefs) -> Self {
        Self {
            env,
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

macro_rules! wrap_event_loop {
    ($name: expr) => { ActiveEventLoop { inner_non_null: NonNull::new($name as *const _ as *mut OriginActiveEventLoop).unwrap() } };
}

macro_rules! borrow_back {
    ($callback: ident @ $env: ident ) => {
        $callback.borrow_back($env).unwrap()
    };
}

impl ApplicationHandler<UserPayload> for ApplicationT4 {
    fn new_events(&mut self, event_loop: &OriginActiveEventLoop, cause: OriginStartCause) {
        let Self { env, on_new_events: Some(callback), .. } = &self else { return; };
        let result = borrow_back!(callback @ env).call(FnArgs::from((wrap_event_loop!(event_loop), StartCause::from(cause))));
        handle_res!(result);
    }

    fn resumed(&mut self, event_loop: &OriginActiveEventLoop) {
        let Self { env, on_resumed: callback, .. } = &self;
        let result = borrow_back!(callback @ env).call(FnArgs::from((wrap_event_loop!(event_loop),)));
        handle_res!(result);
    }

    fn user_event(&mut self, event_loop: &OriginActiveEventLoop, event: UserPayload) {
        let Self { env, on_user_event: Some(callback), .. } = &self else { return; };
        let result = borrow_back!(callback @ env).call(FnArgs::from((wrap_event_loop!(event_loop), event)));
        handle_res!(result);
    }

    fn window_event(&mut self, event_loop: &OriginActiveEventLoop, window_id: OriginWindowId, event: OriginWindowEvent) {
        let Self { env, on_window_event: callback, .. } = &self;
        let result = borrow_back!(callback @ env).call(FnArgs::from((wrap_event_loop!(event_loop), WindowId::from(window_id), WindowEvent::from(event))));
        handle_res!(result);
    }

    fn device_event(&mut self, event_loop: &OriginActiveEventLoop, device_id: OriginDeviceId, event: OriginDeviceEvent) {
        let Self { env, on_device_event: Some(callback), .. } = &self else { return; };
        let result = borrow_back!(callback @ env).call(FnArgs::from((wrap_event_loop!(event_loop), DeviceId::from(device_id), DeviceEvent::from(event))));
        handle_res!(result);
    }

    fn about_to_wait(&mut self, event_loop: &OriginActiveEventLoop) {
        let Self { env, on_about_to_wait: Some(callback), .. } = &self else { return; };
        let result = borrow_back!(callback @ env).call(FnArgs::from((wrap_event_loop!(event_loop),)));
        handle_res!(result);
    }

    fn suspended(&mut self, event_loop: &OriginActiveEventLoop) {
        let Self { env, on_suspended: Some(callback), .. } = &self else { return; };
        let result = borrow_back!(callback @ env).call(FnArgs::from((wrap_event_loop!(event_loop),)));
        handle_res!(result);
    }

    fn exiting(&mut self, event_loop: &OriginActiveEventLoop) {
        let Self { env, on_exiting: Some(callback), .. } = &self else { return; };
        let result = borrow_back!(callback @ env).call(FnArgs::from((wrap_event_loop!(event_loop),)));
        handle_res!(result);
    }

    fn memory_warning(&mut self, event_loop: &OriginActiveEventLoop) {
        let Self { env, on_memory_warning: Some(callback), .. } = &self else { return; };
        let result = borrow_back!(callback @ env).call(FnArgs::from((wrap_event_loop!(event_loop),)));
        handle_res!(result);
    }
}