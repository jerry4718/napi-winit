use std::future::Future;
use std::ptr::NonNull;
use std::rc::Rc;
use std::task::Waker;
use std::thread;
use std::time::Duration;
use winit::{
    event::{
        DeviceEvent as OriginDeviceEvent,
        DeviceId as OriginDeviceId,
        StartCause as OriginStartCause,
        WindowEvent as OriginWindowEvent,
    },
    application::ApplicationHandler as OriginApplicationHandler,
    event_loop::ActiveEventLoop as OriginActiveEventLoop,
    window::WindowId as OriginWindowId,
};
use crate::event::{Event, StartCause, WindowEvent, DeviceId, DeviceEvent, UserPayload};
use crate::window::WindowId;
use crate::event_loop::ActiveEventLoop;

use napi::bindgen_prelude::*;
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction};

#[napi(object)]
pub struct Application {
    #[napi(ts_type = "(eventLoop: ActiveEventLoop, cause: StartCause) => void")]
    pub on_new_events: Option<JsFunction>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => void")]
    pub on_resumed: JsFunction,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop, event: UserPayload) => void")]
    pub on_user_event: Option<JsFunction>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop, windowId: WindowId, event: WindowEvent) => void")]
    pub on_window_event: JsFunction,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop, deviceId: DeviceId, event: DeviceEvent) => void")]
    pub on_device_event: Option<JsFunction>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => void")]
    pub on_about_to_wait: Option<JsFunction>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => void")]
    pub on_suspended: Option<JsFunction>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => void")]
    pub on_exiting: Option<JsFunction>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => void")]
    pub on_memory_warning: Option<JsFunction>,
}

macro_rules! tokio_spawn {
    ($promise: ident) => {
        tokio::spawn(async {
            let Err(Error { status, reason, .. }) = $promise.await else { return };
            println!("status: {}, reason: {}", status, reason);
        });
    };
}

impl OriginApplicationHandler<UserPayload> for Application {
    fn new_events(&mut self, event_loop: &OriginActiveEventLoop, cause: OriginStartCause) {
        let Self { on_new_events: Some(new_events), .. } = self else { return; };
        let event_loop = ActiveEventLoop { inner: unsafe { Rc::from_raw(event_loop as *const _) } };
        let Some(promise) = new_events.call2::<ActiveEventLoop, StartCause, Option<Promise<()>>>(event_loop, StartCause::from(cause))
            .expect("call error") else { return };
        tokio_spawn!(promise);
    }

    fn resumed(&mut self, event_loop: &OriginActiveEventLoop) {
        let Self { on_resumed, .. } = self;
        let event_loop = ActiveEventLoop { inner: unsafe { Rc::from_raw(event_loop as *const _) } };
        let Some(promise) = on_resumed.call1::<ActiveEventLoop, Option<Promise<()>>>(event_loop)
            .expect("call error") else { return };
        tokio_spawn!(promise);
    }

    fn user_event(&mut self, event_loop: &OriginActiveEventLoop, event: UserPayload) {
        let Self { on_user_event: Some(user_event), .. } = self else { return; };
        let event_loop = ActiveEventLoop { inner: unsafe { Rc::from_raw(event_loop as *const _) } };
        let Some(promise) = user_event.call2::<ActiveEventLoop, UserPayload, Option<Promise<()>>>(event_loop, event)
            .expect("call error") else { return };
        tokio_spawn!(promise);
    }

    fn window_event(&mut self, event_loop: &OriginActiveEventLoop, window_id: OriginWindowId, event: OriginWindowEvent) {
        let Self { on_window_event, .. } = self;
        let event_loop = ActiveEventLoop { inner: unsafe { Rc::from_raw(event_loop as *const _) } };
        let Some(promise) = on_window_event.call3::<ActiveEventLoop, WindowId, WindowEvent, Option<Promise<()>>>(event_loop, WindowId::from(window_id), WindowEvent::from(event))
            .expect("call error") else { return };
        tokio_spawn!(promise);
    }

    fn device_event(&mut self, event_loop: &OriginActiveEventLoop, device_id: OriginDeviceId, event: OriginDeviceEvent) {
        let Self { on_device_event: Some(device_event), .. } = self else { return; };
        let event_loop = ActiveEventLoop { inner: unsafe { Rc::from_raw(event_loop as *const _) } };
        let Some(promise) = device_event.call3::<ActiveEventLoop, DeviceId, DeviceEvent, Option<Promise<()>>>(event_loop, DeviceId::from(device_id), DeviceEvent::from(event))
            .expect("call error") else { return };
        tokio_spawn!(promise);
    }

    fn about_to_wait(&mut self, event_loop: &OriginActiveEventLoop) {
        let Self { on_about_to_wait: Some(about_to_wait), .. } = self else { return; };
        let event_loop = ActiveEventLoop { inner: unsafe { Rc::from_raw(event_loop as *const _) } };
        let Some(promise) = about_to_wait.call1::<ActiveEventLoop, Option<Promise<()>>>(event_loop)
            .expect("call error") else { return };
        tokio_spawn!(promise);
    }

    fn suspended(&mut self, event_loop: &OriginActiveEventLoop) {
        let Self { on_suspended: Some(suspended), .. } = self else { return; };
        let event_loop = ActiveEventLoop { inner: unsafe { Rc::from_raw(event_loop as *const _) } };
        let Some(promise) = suspended.call1::<ActiveEventLoop, Option<Promise<()>>>(event_loop)
            .expect("call error") else { return };
        tokio_spawn!(promise);
    }

    fn exiting(&mut self, event_loop: &OriginActiveEventLoop) {
        let Self { on_exiting: Some(exiting), .. } = self else { return; };
        let event_loop = ActiveEventLoop { inner: unsafe { Rc::from_raw(event_loop as *const _) } };
        let Some(promise) = exiting.call1::<ActiveEventLoop, Option<Promise<()>>>(event_loop)
            .expect("call error") else { return };
        tokio_spawn!(promise);
    }

    fn memory_warning(&mut self, event_loop: &OriginActiveEventLoop) {
        let Self { on_memory_warning: Some(memory_warning), .. } = self else { return; };
        let event_loop = ActiveEventLoop { inner: unsafe { Rc::from_raw(event_loop as *const _) } };
        let Some(promise) = memory_warning.call1::<ActiveEventLoop, Option<Promise<()>>>(event_loop)
            .expect("call error") else { return };
        tokio_spawn!(promise);
    }
}