use std::process::ExitCode;
use std::ptr::NonNull;
use std::rc::Rc;
use std::time;
use std::time::{Duration, Instant};
use winit::event_loop::{
    ActiveEventLoop as OriginActiveEventLoop,
    EventLoop as OriginEventLoop,
    AsyncRequestSerial as OriginAsyncRequestSerial,
    ControlFlow as OriginControlFlow,
    DeviceEvents as OriginDeviceEvents,
    OwnedDisplayHandle as OriginOwnedDisplayHandle,
};
use winit::platform::run_on_demand::EventLoopExtRunOnDemand;


use proc::{mapping_enum};
use crate::event::UserPayload;
use crate::{mark_ex_into, string_enum, wrap_struct};
use crate::cursor::{CustomCursor, CustomCursorSource};
use crate::extra::{
    TimeDuration,
    convert::ExInto
};
use crate::window::{Theme, Window, WindowAttributes};
use crate::monitor::MonitorHandle;

use napi::bindgen_prelude::*;
use winit::platform::pump_events::{
    EventLoopExtPumpEvents,
    PumpStatus as OriginPumpStatus,
};
use crate::application::Application;

#[napi]
pub struct EventLoop {
    pub(crate) inner: OriginEventLoop<UserPayload>,
}

#[napi]
impl EventLoop {
    #[napi(constructor)]
    pub fn new() -> Self {
        let event_loop = OriginEventLoop::<UserPayload>::with_user_event().build().expect("Failed to build EventLoop");
        Self { inner: event_loop }
    }
}

#[napi(object)]
pub struct PumpStatus {
    #[napi(ts_type = "PumpStatus.Type")]
    pub r#type: pump_status::Type,
    pub code: Option<i32>
}

impl From<OriginPumpStatus> for PumpStatus {
    fn from(status: OriginPumpStatus) -> Self {
        match status {
            OriginPumpStatus::Continue => Self { r#type:pump_status::Type::Continue, code: None },
            OriginPumpStatus::Exit(code) => Self { r#type:pump_status::Type::Exit, code: Some(code) },
        }
    }
}

#[napi(js_name = "PumpStatus")]
pub mod pump_status {
    #[napi(string_enum)]
    pub enum Type {
        Continue,
        Exit,
    }
}

#[napi]
impl EventLoop {
    // with_user_event
    #[napi]
    pub fn run_app(&self, app: Application) -> Result<()> {
        let mut ptr = NonNull::new(&app as *const _ as *mut Application).unwrap();
        let this = unsafe { Box::from_raw(self as * const _ as *mut EventLoop) };
        this.inner.run_app(unsafe { ptr.as_mut() }).map_err(|e| Error::from_reason(format!("{}", e)))
    }
    #[napi]
    pub fn run_app_on_demand(&mut self, app: Application) -> Result<()> {
        let mut ptr = NonNull::new(&app as *const _ as *mut Application).unwrap();
        self.inner.run_app_on_demand(unsafe { ptr.as_mut() }).map_err(|e| Error::from_reason(format!("{}", e)))
    }
    #[napi]
    pub fn pump_app_events(&mut self, millis: f64, app: Application) -> PumpStatus {
        let mut ptr = NonNull::new(&app as *const _ as *mut Application).unwrap();
        let timeout = Some(Duration::from_millis(millis as u64));
        self.inner.pump_app_events(timeout, unsafe { ptr.as_mut() }).into()
    }
    // create_proxy
    // owned_display_handle
    // listen_device_events
    // create_window
    // create_custom_cursor
}

// wrap_struct!();
#[napi]
pub struct ActiveEventLoop {
    pub(crate) inner: Rc<OriginActiveEventLoop>
}

#[napi]
impl ActiveEventLoop {
    #[napi]
    pub fn create_window(&self, window_attributes: &WindowAttributes) -> Result<Window> {
        self.inner.create_window(window_attributes.clone().into())
            .map(Window::from)
            .map_err(|e| Error::from_reason(format!("{}", e)))
    }
    // #[napi]
    // pub fn create_custom_cursor(&self, custom_cursor: &CustomCursorSource) -> CustomCursor {
    //     self.inner.create_custom_cursor(custom_cursor.clone().into()).into()
    // }
    #[napi]
    pub fn available_monitors(&self) -> Vec<MonitorHandle> {
        self.inner.available_monitors().map(|m| m.into()).collect()
    }
    #[napi]
    pub fn primary_monitor(&self) -> Option<MonitorHandle> {
        self.inner.primary_monitor().map(|m| m.into())
    }
    #[napi]
    pub fn listen_device_events(&self, allowed: DeviceEvents) {
        self.inner.listen_device_events(allowed.into())
    }
    #[napi]
    pub fn system_theme(&self) -> Option<Theme> {
        self.inner.system_theme().map(|theme| theme.into())
    }
    // #[napi]
    // pub fn set_control_flow(&self, control_flow: &ControlFlow) {
    //     self.inner.set_control_flow(control_flow.clone().into())
    // }

    #[napi]
    pub fn set_control_flow_poll(&self) {
        self.inner.set_control_flow(OriginControlFlow::Poll)
    }
    #[napi]
    pub fn set_control_flow_wait(&self) {
        self.inner.set_control_flow(OriginControlFlow::Wait)
    }
    #[napi]
    pub fn set_control_flow_wait_until(&self, millis: f64) {
        self.inner.set_control_flow(OriginControlFlow::WaitUntil(Instant::now() + Duration::from_millis(millis as u64)))
    }

    #[napi]
    pub fn control_flow(&self) -> ControlFlow {
        self.inner.control_flow().into()
    }
    #[napi]
    pub fn exit(&self) {
        self.inner.exit()
    }
    #[napi]
    pub fn exiting(&self) -> bool {
        self.inner.exiting()
    }
    #[napi]
    pub fn owned_display_handle(&self) -> OwnedDisplayHandle {
        self.inner.owned_display_handle().into()
    }
}

string_enum!(
    enum DeviceEvents => OriginDeviceEvents {
        Always,
        WhenFocused,
        Never,
    }
);


mapping_enum!(
    enum ControlFlow {
        Poll,
        Wait,
        WaitUntil(
            #[conf_trans_type = TimeDuration]
            #[conf_assign_name = "time"]
            Instant
        ),
    }
);

wrap_struct!(#[derive(Clone)]struct OwnedDisplayHandle(OriginOwnedDisplayHandle));
wrap_struct!(#[derive(Clone)]struct AsyncRequestSerial(OriginAsyncRequestSerial));

mark_ex_into!(OriginAsyncRequestSerial, OriginControlFlow, AsyncRequestSerial, ControlFlow);