use std::ptr::NonNull;
use std::rc::Rc;
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

pub struct Application {
    pub on_new_events: Option<fn(event_loop: &ActiveEventLoop, cause: StartCause)>,
    pub on_resumed: fn(event_loop: &ActiveEventLoop),
    pub on_user_event: Option<fn(event_loop: &ActiveEventLoop, event: UserPayload)>,
    pub on_window_event: fn(event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent),
    pub on_device_event: Option<fn(event_loop: &ActiveEventLoop, device_id: DeviceId, event: DeviceEvent)>,
    pub on_about_to_wait: Option<fn(event_loop: &ActiveEventLoop)>,
    pub on_suspended: Option<fn(event_loop: &ActiveEventLoop)>,
    pub on_exiting: Option<fn(event_loop: &ActiveEventLoop)>,
    pub on_memory_warning: Option<fn(event_loop: &ActiveEventLoop)>,
}

impl OriginApplicationHandler<UserPayload> for Application {
    fn new_events(&mut self, event_loop: &OriginActiveEventLoop, cause: OriginStartCause) {
        let Self { on_new_events: Some(new_events), .. } = self else { return; };
        let event_loop = ActiveEventLoop { inner: unsafe { Rc::from_raw(event_loop as *const _) } };
        new_events(&event_loop, StartCause::from(cause))
    }

    fn resumed(&mut self, event_loop: &OriginActiveEventLoop) {
        let Self { on_resumed, .. } = self;
        let event_loop = ActiveEventLoop { inner: unsafe { Rc::from_raw(event_loop as *const _) } };
        on_resumed(&event_loop);
    }

    fn user_event(&mut self, event_loop: &OriginActiveEventLoop, event: UserPayload) {
        let Self { on_user_event: Some(user_event), .. } = self else { return; };
        let event_loop = ActiveEventLoop { inner: unsafe { Rc::from_raw(event_loop as *const _) } };
        user_event(&event_loop, event)
    }

    fn window_event(&mut self, event_loop: &OriginActiveEventLoop, window_id: OriginWindowId, event: OriginWindowEvent) {
        let Self { on_window_event, .. } = self;
        let event_loop = ActiveEventLoop { inner: unsafe { Rc::from_raw(event_loop as *const _) } };
        on_window_event(&event_loop, WindowId::from(window_id), WindowEvent::from(event));
    }

    fn device_event(&mut self, event_loop: &OriginActiveEventLoop, device_id: OriginDeviceId, event: OriginDeviceEvent) {
        let Self { on_device_event: Some(device_event), .. } = self else { return; };
        let event_loop = ActiveEventLoop { inner: unsafe { Rc::from_raw(event_loop as *const _) } };
        device_event(&event_loop, DeviceId::from(device_id), DeviceEvent::from(event))
    }

    fn about_to_wait(&mut self, event_loop: &OriginActiveEventLoop) {
        let Self { on_about_to_wait: Some(about_to_wait), .. } = self else { return; };
        let event_loop = ActiveEventLoop { inner: unsafe { Rc::from_raw(event_loop as *const _) } };
        about_to_wait(&event_loop)
    }

    fn suspended(&mut self, event_loop: &OriginActiveEventLoop) {
        let Self { on_suspended: Some(suspended), .. } = self else { return; };
        let event_loop = ActiveEventLoop { inner: unsafe { Rc::from_raw(event_loop as *const _) } };
        suspended(&event_loop)
    }

    fn exiting(&mut self, event_loop: &OriginActiveEventLoop) {
        let Self { on_exiting: Some(exiting), .. } = self else { return; };
        let event_loop = ActiveEventLoop { inner: unsafe { Rc::from_raw(event_loop as *const _) } };
        exiting(&event_loop)
    }

    fn memory_warning(&mut self, event_loop: &OriginActiveEventLoop) {
        let Self { on_memory_warning: Some(memory_warning), .. } = self else { return; };
        let event_loop = ActiveEventLoop { inner: unsafe { Rc::from_raw(event_loop as *const _) } };
        memory_warning(&event_loop)
    }
}