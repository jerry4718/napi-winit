use std::rc::Rc;
use winit::event_loop::{
    ActiveEventLoop as OriginActiveEventLoop,
    EventLoop as OriginEventLoop,
    AsyncRequestSerial as OriginAsyncRequestSerial,
    ControlFlow as OriginControlFlow,
    DeviceEvents as OriginDeviceEvents,
    OwnedDisplayHandle as OriginOwnedDisplayHandle,
};
use proc::{mapping_enum};
use crate::event::UserPayload;
use crate::{mark_ex_into, wrap_struct};
use crate::cursor::{CustomCursor, CustomCursorSource};
use crate::extra::{
    TimeDuration,
    convert::ExInto
};
use crate::window::{Theme, Window, WindowAttributes};
use crate::monitor::MonitorHandle;

use napi::bindgen_prelude::*;

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
    // with_user_event
    // run
    // run_app
    // create_proxy
    // owned_display_handle
    // listen_device_events
    // set_control_flow
    // create_window
    // create_custom_cursor
}

// wrap_struct!();
#[napi]
pub struct ActiveEventLoop {
    pub(crate) inner: Rc<OriginActiveEventLoop>
}

impl ActiveEventLoop {
    pub fn create_window(&self, window_attributes: WindowAttributes) -> Result<Window> {
        self.inner.create_window(window_attributes.into())
            .map(Window::from)
            .map_err(|e| Error::from_reason(format!("{}", e)))
    }
    pub fn create_custom_cursor(&self, custom_cursor: CustomCursorSource) -> CustomCursor {
        self.inner.create_custom_cursor(custom_cursor.into()).into()
    }
    pub fn available_monitors(&self) -> Vec<MonitorHandle> {
        self.inner.available_monitors().map(|m| m.into()).collect()
    }
    pub fn primary_monitor(&self) -> Option<MonitorHandle> {
        self.inner.primary_monitor().map(|m| m.into())
    }
    pub fn listen_device_events(&self, allowed: DeviceEvents) {
        self.inner.listen_device_events(allowed.into())
    }
    pub fn system_theme(&self) -> Option<Theme> {
        self.inner.system_theme().map(|theme| theme.into())
    }
    pub fn set_control_flow(&self, control_flow: ControlFlow) {
        self.inner.set_control_flow(control_flow.into())
    }
    pub fn control_flow(&self) -> ControlFlow {
        self.inner.control_flow().into()
    }
    pub fn exit(&self) {
        self.inner.exit()
    }
    pub fn exiting(&self) -> bool {
        self.inner.exiting()
    }
    pub fn owned_display_handle(&self) -> OwnedDisplayHandle {
        self.inner.owned_display_handle().into()
    }
}

mapping_enum!(
    enum DeviceEvents {
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