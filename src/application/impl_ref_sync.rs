use napi::bindgen_prelude::*;
use std::ptr::NonNull;

use winit::{
    application::ApplicationHandler,
    event::{
        DeviceEvent as OriginDeviceEvent,
        DeviceId as OriginDeviceId,
        StartCause as OriginStartCause,
        WindowEvent as OriginWindowEvent,
    },
    event_loop::ActiveEventLoop as OriginActiveEventLoop,
    window::WindowId as OriginWindowId,
};

use crate::{
    application::public::{OptionsRefHolder, OptionsRefWithEnv},
    event::{
        DeviceEvent,
        DeviceId,
        StartCause,
        UserPayload,
        WindowEvent,
    },
    event_loop::ActiveEventLoop,
    handle_res,
    window::WindowId,
};

macro_rules! wrap_event_loop {
    ($name: expr) => { ActiveEventLoop { inner_non_null: NonNull::new($name as *const _ as *mut OriginActiveEventLoop).unwrap() } };
}

macro_rules! borrow_back {
    ($callback: ident @ $env: ident ) => {
        $callback.borrow_back($env).unwrap()
    };
}

impl ApplicationHandler<UserPayload> for OptionsRefWithEnv<()> {
    fn new_events(&mut self, event_loop: &OriginActiveEventLoop, cause: OriginStartCause) {
        let Self { env, options: OptionsRefHolder { on_new_events: Some(callback), .. } } = &self else { return; };
        let result = borrow_back!(callback @ env).call(FnArgs::from((wrap_event_loop!(event_loop), StartCause::from(cause))));
        handle_res!(result);
    }

    fn resumed(&mut self, event_loop: &OriginActiveEventLoop) {
        let Self { env, options: OptionsRefHolder { on_resumed: callback, .. } } = &self;
        let result = borrow_back!(callback @ env).call(FnArgs::from((wrap_event_loop!(event_loop),)));
        handle_res!(result);
    }

    fn user_event(&mut self, event_loop: &OriginActiveEventLoop, event: UserPayload) {
        let Self { env, options: OptionsRefHolder { on_user_event: Some(callback), .. } } = &self else { return; };
        let result = borrow_back!(callback @ env).call(FnArgs::from((wrap_event_loop!(event_loop), event)));
        handle_res!(result);
    }

    fn window_event(&mut self, event_loop: &OriginActiveEventLoop, window_id: OriginWindowId, event: OriginWindowEvent) {
        let Self { env, options: OptionsRefHolder { on_window_event: callback, .. } } = &self;
        let result = borrow_back!(callback @ env).call(FnArgs::from((wrap_event_loop!(event_loop), WindowId::from(window_id), WindowEvent::from(event))));
        handle_res!(result);
    }

    fn device_event(&mut self, event_loop: &OriginActiveEventLoop, device_id: OriginDeviceId, event: OriginDeviceEvent) {
        let Self { env, options: OptionsRefHolder { on_device_event: Some(callback), .. } } = &self else { return; };
        let result = borrow_back!(callback @ env).call(FnArgs::from((wrap_event_loop!(event_loop), DeviceId::from(device_id), DeviceEvent::from(event))));
        handle_res!(result);
    }

    fn about_to_wait(&mut self, event_loop: &OriginActiveEventLoop) {
        let Self { env, options: OptionsRefHolder { on_about_to_wait: Some(callback), .. } } = &self else { return; };
        let result = borrow_back!(callback @ env).call(FnArgs::from((wrap_event_loop!(event_loop),)));
        handle_res!(result);
    }

    fn suspended(&mut self, event_loop: &OriginActiveEventLoop) {
        let Self { env, options: OptionsRefHolder { on_suspended: Some(callback), .. } } = &self else { return; };
        let result = borrow_back!(callback @ env).call(FnArgs::from((wrap_event_loop!(event_loop),)));
        handle_res!(result);
    }

    fn exiting(&mut self, event_loop: &OriginActiveEventLoop) {
        let Self { env, options: OptionsRefHolder { on_exiting: Some(callback), .. } } = &self else { return; };
        let result = borrow_back!(callback @ env).call(FnArgs::from((wrap_event_loop!(event_loop),)));
        handle_res!(result);
    }

    fn memory_warning(&mut self, event_loop: &OriginActiveEventLoop) {
        let Self { env, options: OptionsRefHolder { on_memory_warning: Some(callback), .. } } = &self else { return; };
        let result = borrow_back!(callback @ env).call(FnArgs::from((wrap_event_loop!(event_loop),)));
        handle_res!(result);
    }
}