use napi::bindgen_prelude::*;

use std::{
    process::ExitCode,
    ptr::NonNull,
    time::{Duration, Instant},
};
use winit::{
    application::ApplicationHandler,
    event_loop::{
        ActiveEventLoop as OriginActiveEventLoop,
        AsyncRequestSerial as OriginAsyncRequestSerial,
        ControlFlow as OriginControlFlow,
        DeviceEvents as OriginDeviceEvents,
        EventLoop as OriginEventLoop,
        OwnedDisplayHandle as OriginOwnedDisplayHandle,
    },
    platform::{
        pump_events::EventLoopExtPumpEvents,
        run_on_demand::EventLoopExtRunOnDemand,
    },
};

use crate::{
    application::public::{Application, Runner},
    cursor::{CustomCursor, CustomCursorSource},
    event::UserPayload,
    extra::{
        convert::ExInto,
        time::TimeDuration,
    },
    mark_ex_into,
    monitor::MonitorHandle,
    string_enum,
    window::{Theme, Window, WindowAttributes},
    wrap_struct,
};

use proc::proxy_enum;

wrap_struct!(struct EventLoop { inner: OriginEventLoop<UserPayload> });

#[napi]
impl EventLoop {
    #[napi(constructor)]
    pub fn new() -> Self {
        let event_loop = OriginEventLoop::<UserPayload>::with_user_event().build().expect("Failed to build EventLoop");
        Self { inner: event_loop }
    }
}

#[proxy_enum(origin_enum = winit::platform::pump_events::PumpStatus)]
pub enum PumpStatus {
    Continue,
    Exit(#[proxy_enum(field_name = "code")] i32),
}

#[napi]
impl EventLoop {
    // with_user_event
    #[napi]
    pub fn run_app(&mut self, env: Env, app: &mut Application) -> Result<()> {
        let this = unsafe { Box::from_raw(self as *const _ as *mut EventLoop) };

        let result = match app.runner {
            Runner::Async(ref mut handler) => this.inner.run_app(&mut handler.with_env(env)),
            Runner::Sync(ref mut handler) => this.inner.run_app(&mut handler.with_env(env)),
            Runner::AsyncRef(ref mut handler) => this.inner.run_app(&mut handler.borrow_back(&env)),
            Runner::SyncRef(ref mut handler) => this.inner.run_app(&mut handler.borrow_back(&env)),
            Runner::AsyncEnvRef(ref mut handler) => this.inner.run_app(handler),
            Runner::SyncEnvRef(ref mut handler) => this.inner.run_app(handler),
        };

        result.map_err(|e| Error::from_reason(format!("{}", e)))
    }

    #[napi]
    pub fn run_app_on_demand(&mut self, env: Env, app: &mut Application) -> Result<()> {
        let result = match app.runner {
            Runner::Async(ref mut handler) => self.inner.run_app_on_demand(&mut handler.with_env(env)),
            Runner::Sync(ref mut handler) => self.inner.run_app_on_demand(&mut handler.with_env(env)),
            Runner::AsyncRef(ref mut handler) => self.inner.run_app_on_demand(&mut handler.borrow_back(&env)),
            Runner::SyncRef(ref mut handler) => self.inner.run_app_on_demand(&mut handler.borrow_back(&env)),
            Runner::AsyncEnvRef(ref mut handler) => self.inner.run_app_on_demand(handler),
            Runner::SyncEnvRef(ref mut handler) => self.inner.run_app_on_demand(handler),
        };

        result.map_err(|e| Error::from_reason(format!("{}", e)))
    }

    #[napi]
    pub fn pump_app_events(&mut self, env: Env, millis: f64, app: &mut Application) -> PumpStatus {
        let timeout = Some(Duration::from_millis(millis as u64));

        let result = match app.runner {
            Runner::Async(ref mut handler) => self.inner.pump_app_events(timeout, &mut handler.with_env(env)),
            Runner::Sync(ref mut handler) => self.inner.pump_app_events(timeout, &mut handler.with_env(env)),
            Runner::AsyncRef(ref mut handler) => self.inner.pump_app_events(timeout, &mut handler.borrow_back(&env)),
            Runner::SyncRef(ref mut handler) => self.inner.pump_app_events(timeout, &mut handler.borrow_back(&env)),
            Runner::AsyncEnvRef(ref mut handler) => self.inner.pump_app_events(timeout, handler),
            Runner::SyncEnvRef(ref mut handler) => self.inner.pump_app_events(timeout, handler),
        };

        PumpStatus::from(result)
    }
    /*#[napi]
    pub fn pump_app_events(&mut self, millis: f64, app: Application) -> PumpStatus {
        let application = unsafe { NonNull::new(&app as *const _ as *mut Application).unwrap().as_mut() };
        let timeout = Some(Duration::from_millis(millis as u64));
        self.inner.pump_app_events(timeout, application).into()
    }*/
    // create_proxy
    // owned_display_handle
    // listen_device_events
    // create_window
    // create_custom_cursor
}

#[napi]
pub struct ActiveEventLoop {
    pub(crate) inner_non_null: NonNull<OriginActiveEventLoop>,
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
        inner_ref!(self).create_window(window_attributes.clone().into())
            .map(Window::from)
            .map_err(|e| Error::from_reason(format!("{}", e)))
    }
    // #[napi]
    // pub fn create_custom_cursor(&self, custom_cursor: &CustomCursorSource) -> CustomCursor {
    //     self.inner.create_custom_cursor(custom_cursor.clone().into()).into()
    // }
    #[napi]
    pub fn available_monitors(&self) -> Vec<MonitorHandle> {
        inner_ref!(self).available_monitors().map(|m| m.into()).collect()
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
    // #[napi]
    // pub fn set_control_flow(&self, control_flow: &ControlFlow) {
    //     self.inner.set_control_flow(control_flow.clone().into())
    // }

    #[napi]
    pub fn set_control_flow_poll(&self) {
        inner_ref!(self).set_control_flow(OriginControlFlow::Poll)
    }
    #[napi]
    pub fn set_control_flow_wait(&self) {
        inner_ref!(self).set_control_flow(OriginControlFlow::Wait)
    }
    #[napi]
    pub fn set_control_flow_wait_until(&self, millis: f64) {
        inner_ref!(self).set_control_flow(OriginControlFlow::WaitUntil(Instant::now() + Duration::from_millis(millis as u64)))
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

string_enum!(
    enum DeviceEvents => OriginDeviceEvents {
        Always,
        WhenFocused,
        Never,
    }
);

#[proxy_enum(origin_enum = winit::event_loop::ControlFlow, skip_backward)]
pub enum ControlFlow {
    Poll,
    Wait,
    WaitUntil(TimeDuration),
}

wrap_struct!(#[derive(Clone)]struct OwnedDisplayHandle(OriginOwnedDisplayHandle));
wrap_struct!(#[derive(Clone)]struct AsyncRequestSerial(OriginAsyncRequestSerial));

mark_ex_into!(OriginAsyncRequestSerial, OriginControlFlow, AsyncRequestSerial, ControlFlow);