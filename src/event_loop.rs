use napi::bindgen_prelude::*;

use std::ptr::NonNull;

use winit::event_loop::{
    pump_events::EventLoopExtPumpEvents, run_on_demand::EventLoopExtRunOnDemand,
};

use proc::{proxy_enum, proxy_wrap};

use crate::{
    application::Application,
    cursor::{CustomCursor, CustomCursorSource},
    extra::time::{Duration, Instant, try_std_duration, try_std_instant},
    monitor::MonitorHandle,
    napi_reason,
    window::{Theme, Window, WindowAttributes},
};

#[proxy_wrap(origin_type = winit::event_loop::EventLoop, field_name = inner)]
pub struct EventLoop;

#[napi]
impl EventLoop {
    #[napi(constructor)]
    pub fn new() -> Self {
        let event_loop = winit::event_loop::EventLoop::new().expect("Failed to build EventLoop");
        Self { inner: event_loop }
    }
}

#[proxy_enum(origin_type = winit::event_loop::pump_events::PumpStatus, skip_backward)]
pub enum PumpStatus {
    Continue,
    Exit(#[proxy_enum(field_name = code)] i32),
}

#[napi]
impl EventLoop {
    // with_user_event
    #[napi]
    pub unsafe fn run_app(&mut self, env: Env, app: &mut Application) -> Result<()> {
        // SAFETY: the event loop runs synchronously on this thread, so the JS `Application`
        // (and its env) outlives every `ApplicationHandler` call inside `run_app`.
        let app: &'static mut Application<'static> = unsafe { std::mem::transmute(app) };
        let event_loop = unsafe { Box::from_raw(self as *const _ as *mut EventLoop) };
        event_loop
            .inner
            .run_app(app)
            .map_err(|e| napi_reason!("{e}"))
    }

    #[napi]
    pub fn run_app_on_demand(&mut self, env: Env, app: &mut Application) -> Result<()> {
        self.inner
            .run_app_on_demand(app)
            .map_err(|e| napi_reason!("{e}"))
    }

    #[napi]
    pub fn pump_app_events(
        &mut self,
        env: Env,
        timeout: Option<Duration>,
        app: &mut Application,
    ) -> Result<PumpStatus> {
        timeout
            .map(|duration| try_std_duration(&duration))
            .transpose()
            .map(|timeout| PumpStatus::from(self.inner.pump_app_events(timeout, app)))
    }

    // create_proxy
    // owned_display_handle
    // listen_device_events
    // create_window
    // create_custom_cursor
}

#[napi]
pub struct ActiveEventLoop {
    pub(crate) inner_non_null: NonNull<dyn winit::event_loop::ActiveEventLoop>,
}

impl ActiveEventLoop {
    pub fn new(origin: &dyn winit::event_loop::ActiveEventLoop) -> Self {
        Self {
            inner_non_null: NonNull::from(origin),
        }
    }
}

impl From<&dyn winit::event_loop::ActiveEventLoop> for ActiveEventLoop {
    fn from(value: &dyn winit::event_loop::ActiveEventLoop) -> Self {
        Self::new(value)
    }
}

macro_rules! inner_ref {
    ($self: ident) => {
        unsafe { $self.inner_non_null.as_ref() }
    };
}

#[napi]
impl ActiveEventLoop {
    #[napi]
    pub fn create_window(&self, window_attributes: &WindowAttributes) -> Result<Window> {
        winit::window::WindowAttributes::try_from(window_attributes.clone())
            .and_then(|attrs| {
                inner_ref!(self)
                    .create_window(attrs)
                    .map_err(|e| napi_reason!("{e}"))
            })
            .map(Window::from)
    }
    // #[napi]
    // pub fn create_custom_cursor(&self, custom_cursor: &CustomCursorSource) -> CustomCursor {
    //     self.inner.create_custom_cursor(custom_cursor.clone().into()).into()
    // }
    #[napi]
    pub fn available_monitors(&self) -> Vec<MonitorHandle> {
        inner_ref!(self)
            .available_monitors()
            .map(|m| m.into())
            .collect()
    }
    #[napi]
    pub fn primary_monitor(&self) -> Option<MonitorHandle> {
        inner_ref!(self).primary_monitor().map(|m| m.into())
    }
    #[napi]
    pub fn listen_device_events(&self, allowed: DeviceEvents) {
        inner_ref!(self).listen_device_events(allowed.into())
    }
    #[napi]
    pub fn system_theme(&self) -> Option<Theme> {
        inner_ref!(self).system_theme().map(|theme| theme.into())
    }
    #[napi]
    pub fn set_control_flow(&self, control_flow: ControlFlow) -> Result<()> {
        match &control_flow {
            ControlFlow::WaitUntil { timeout } => try_std_instant(timeout).map(|_| ()),
            _ => Ok(()),
        }
        .map(|()| inner_ref!(self).set_control_flow(control_flow.into()))
    }
    #[napi]
    pub fn control_flow(&self) -> ControlFlow {
        inner_ref!(self).control_flow().into()
    }
    #[napi]
    pub fn exit(&self) {
        inner_ref!(self).exit()
    }
    #[napi]
    pub fn exiting(&self) -> bool {
        inner_ref!(self).exiting()
    }
    #[napi]
    pub fn owned_display_handle(&self) -> OwnedDisplayHandle {
        inner_ref!(self).owned_display_handle().into()
    }
}

#[proxy_enum(origin_type = winit::event_loop::DeviceEvents, string_enum, skip_forward)]
pub enum DeviceEvents {
    Always,
    WhenFocused,
    Never,
}

// /** [winit::event_loop::ControlFlow] */
#[proxy_enum(origin_type = winit::event_loop::ControlFlow)]
pub enum ControlFlow {
    Poll,
    Wait,
    WaitUntil(#[proxy_enum(field_name = timeout)] Instant),
}

/** [winit::event_loop::OwnedDisplayHandle] */
#[proxy_wrap(origin_type = winit::event_loop::OwnedDisplayHandle)]
pub struct OwnedDisplayHandle;

/** [winit::event_loop::AsyncRequestSerial] */
#[proxy_wrap(origin_type = winit::event_loop::AsyncRequestSerial)]
pub struct AsyncRequestSerial;
