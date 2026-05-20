use napi::bindgen_prelude::*;
use crate::{
    event::{
        DeviceEvent,
        DeviceId,
        StartCause,
        UserPayload,
        WindowEvent,
    },
    event_loop::ActiveEventLoop,
    window::WindowId,
    handle_res, handle_rop,
};

#[napi(object, object_to_js = false)]
pub struct ApplicationCallbacks<'scope> {
    #[napi(ts_type = "(eventLoop: ActiveEventLoop, cause: StartCause) => unknown")]
    pub on_new_events: Option<FunctionRef<FnArgs<(ActiveEventLoop, StartCause)>, Unknown<'scope>>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => unknown")]
    pub on_resumed: FunctionRef<FnArgs<(ActiveEventLoop,)>, Unknown<'scope>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop, event: UserPayload) => unknown")]
    pub on_user_event: Option<FunctionRef<FnArgs<(ActiveEventLoop, UserPayload)>, Unknown<'scope>>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop, windowId: WindowId, event: WindowEvent) => unknown")]
    pub on_window_event: FunctionRef<FnArgs<(ActiveEventLoop, WindowId, WindowEvent)>, Unknown<'scope>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop, deviceId: DeviceId, event: DeviceEvent) => unknown")]
    pub on_device_event: Option<FunctionRef<FnArgs<(ActiveEventLoop, DeviceId, DeviceEvent)>, Unknown<'scope>>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => unknown")]
    pub on_about_to_wait: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, Unknown<'scope>>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => unknown")]
    pub on_suspended: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, Unknown<'scope>>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => unknown")]
    pub on_exiting: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, Unknown<'scope>>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => unknown")]
    pub on_memory_warning: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, Unknown<'scope>>>,
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
    (impl <$($life: lifetime), *> <$user_event: ty> for $impl_ty: ty { $macro_get: ident, $macro_call: ident }) => {
        impl <$($life), *> winit::application::ApplicationHandler<$user_event> for $impl_ty {
            fn new_events(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, cause: winit::event::StartCause) {
                $macro_get!($macro_call, self, on_new_events?, event_loop, cause);
            }

            fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
                $macro_get!($macro_call, self, on_resumed, event_loop);
            }

            fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: $user_event) {
                $macro_get!($macro_call, self, on_user_event?, event_loop, event);
            }

            fn window_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, window_id: winit::window::WindowId, event: winit::event::WindowEvent) {
                $macro_get!($macro_call, self, on_window_event, event_loop, window_id, event);
            }

            fn device_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, device_id: winit::event::DeviceId, event: winit::event::DeviceEvent) {
                $macro_get!($macro_call, self, on_device_event?, event_loop, device_id, event);
            }

            fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
                $macro_get!($macro_call, self, on_about_to_wait?, event_loop);
            }

            fn suspended(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
                $macro_get!($macro_call, self, on_suspended?, event_loop);
            }

            fn exiting(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
                $macro_get!($macro_call, self, on_exiting?, event_loop);
            }

            fn memory_warning(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
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

impl_with_macros!(impl <'env> <UserPayload> for Application<'env> { get_with_env, call_ref_sync });