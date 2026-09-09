use crate::{
    event::{DeviceEvent, DeviceId, StartCause, WindowEvent},
    event_loop::ActiveEventLoop,
    handle_res, handle_rop,
    window::WindowId,
};
use napi::bindgen_prelude::*;

// Callback aliases: keep the `Option` wrapper on the struct fields — napi-rs derives the
// optional (`?`) TS signature from seeing the explicit `Option<...>` field type.
type NewEventsCallback<'scope> =
    FunctionRef<FnArgs<(ActiveEventLoop, StartCause)>, Unknown<'scope>>;
type EventLoopCallback<'scope> = FunctionRef<FnArgs<(ActiveEventLoop,)>, Unknown<'scope>>;
type WindowEventCallback<'scope> =
    FunctionRef<FnArgs<(ActiveEventLoop, WindowId, WindowEvent)>, Unknown<'scope>>;
type DeviceEventCallback<'scope> =
    FunctionRef<FnArgs<(ActiveEventLoop, Option<DeviceId>, DeviceEvent)>, Unknown<'scope>>;

#[napi(object, object_to_js = false)]
pub struct ApplicationCallbacks<'scope> {
    #[napi(ts_type = "(eventLoop: ActiveEventLoop, cause: StartCause) => unknown")]
    pub on_new_events: Option<NewEventsCallback<'scope>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => unknown")]
    pub on_resumed: Option<EventLoopCallback<'scope>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => unknown")]
    pub on_can_create_surfaces: EventLoopCallback<'scope>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => unknown")]
    pub on_proxy_wake_up: Option<EventLoopCallback<'scope>>,
    #[napi(
        ts_type = "(eventLoop: ActiveEventLoop, windowId: WindowId, event: WindowEvent) => unknown"
    )]
    pub on_window_event: WindowEventCallback<'scope>,
    #[napi(
        ts_type = "(eventLoop: ActiveEventLoop, deviceId: DeviceId | null, event: DeviceEvent) => unknown"
    )]
    pub on_device_event: Option<DeviceEventCallback<'scope>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => unknown")]
    pub on_about_to_wait: Option<EventLoopCallback<'scope>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => unknown")]
    pub on_suspended: Option<EventLoopCallback<'scope>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => unknown")]
    pub on_destroy_surfaces: Option<EventLoopCallback<'scope>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => unknown")]
    pub on_memory_warning: Option<EventLoopCallback<'scope>>,
}

#[napi]
pub struct Application<'env> {
    pub(crate) env: Env,
    pub(crate) options: ApplicationCallbacks<'env>,
}

#[napi]
impl<'env> Application<'env> {
    #[napi(factory)]
    pub fn with_options(env: Env, options: ApplicationCallbacks<'env>) -> Self {
        Self { env, options }
    }
}

macro_rules! impl_with_macros {
    (impl <$($life: lifetime), *> for $impl_ty: ty { $macro_get: ident, $macro_call: ident }) => {
        impl <$($life), *> winit::application::ApplicationHandler for $impl_ty {
            fn new_events(&mut self, event_loop: &dyn winit::event_loop::ActiveEventLoop, cause: winit::event::StartCause) {
                $macro_get!($macro_call, self, on_new_events?, event_loop, cause);
            }

            fn resumed(&mut self, event_loop: &dyn winit::event_loop::ActiveEventLoop) {
                $macro_get!($macro_call, self, on_resumed?, event_loop);
            }

            fn can_create_surfaces(&mut self, event_loop: &dyn winit::event_loop::ActiveEventLoop) {
                $macro_get!($macro_call, self, on_can_create_surfaces, event_loop);
            }

            fn proxy_wake_up(&mut self, event_loop: &dyn winit::event_loop::ActiveEventLoop) {
                $macro_get!($macro_call, self, on_proxy_wake_up?, event_loop);
            }

            fn window_event(&mut self, event_loop: &dyn winit::event_loop::ActiveEventLoop, window_id: winit::window::WindowId, event: winit::event::WindowEvent) {
                $macro_get!($macro_call, self, on_window_event, event_loop, window_id, event);
            }

            fn device_event(&mut self, event_loop: &dyn winit::event_loop::ActiveEventLoop, device_id: Option<winit::event::DeviceId>, event: winit::event::DeviceEvent) {
                let device_id = device_id.map(crate::event::DeviceId::from);
                $macro_get!($macro_call, self, on_device_event?, event_loop, device_id, event);
            }

            fn about_to_wait(&mut self, event_loop: &dyn winit::event_loop::ActiveEventLoop) {
                $macro_get!($macro_call, self, on_about_to_wait?, event_loop);
            }

            fn suspended(&mut self, event_loop: &dyn winit::event_loop::ActiveEventLoop) {
                $macro_get!($macro_call, self, on_suspended?, event_loop);
            }

            fn destroy_surfaces(&mut self, event_loop: &dyn winit::event_loop::ActiveEventLoop) {
                $macro_get!($macro_call, self, on_destroy_surfaces?, event_loop);
            }

            fn memory_warning(&mut self, event_loop: &dyn winit::event_loop::ActiveEventLoop) {
                $macro_get!($macro_call, self, on_memory_warning?, event_loop);
            }
        }
    };
}

macro_rules! get_with_env {
    ($macro_call: ident, $self: ident, $func: ident $(, $arg_name: ident: $arg_type: ty )+) => {
        let Self { env, options: ApplicationCallbacks { $func: $func, .. } } = &$self;
        $macro_call!($func, env $(, $arg_name)+);
    };
    ($macro_call: ident, $self: ident, $func: ident, $($arg_name: ident), +) => {
        let Self { env, options: ApplicationCallbacks { $func: $func, .. } } = &$self;
        $macro_call!($func, env $(, $arg_name)+);
    };
    ($macro_call: ident, $self: ident, $func: ident?, $($arg_name: ident), +) => {
        let Self { env, options: ApplicationCallbacks { $func: Some($func), .. } } = &$self else { return; };
        $macro_call!($func, env $(, $arg_name)+);
    };
}

macro_rules! call_ref_sync {
    ($fx: ident, $env: ident $(, $args: expr)+) => {
        let $fx = $fx.borrow_back($env).unwrap();
        let result = $fx.call(FnArgs::from(($(From::from($args), )+)));
        handle_res!(result);
    }
}

impl_with_macros!(impl <'env> for Application<'env> { get_with_env, call_ref_sync });
