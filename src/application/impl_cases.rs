use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunctionCallMode;

use crate::{
    application::public::OptionsFxHolder,
    application::public::OptionsSafeHolder,
    application::public::{OptionsRefHolder, OptionsGhostHolder},
    event::UserPayload,
    handle_res,
    handle_rop,
};

macro_rules! impl_with_call_macro {
    (impl <$($life: lifetime), *> $user_event: ty => $impl_ty: ty | $get_macro: ident + $call_macro: ident) => {
        impl <$($life), *> winit::application::ApplicationHandler<$user_event> for $impl_ty {
            fn new_events(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, cause: winit::event::StartCause) {
                $get_macro!($call_macro, self, on_new_events?, event_loop, cause);
            }
        
            fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
                $get_macro!($call_macro, self, on_resumed, event_loop);
            }
        
            fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: $user_event) {
                $get_macro!($call_macro, self, on_user_event?, event_loop, event);
            }
        
            fn window_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, window_id: winit::window::WindowId, event: winit::event::WindowEvent) {
                $get_macro!($call_macro, self, on_window_event, event_loop, window_id, event);
            }
        
            fn device_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, device_id: winit::event::DeviceId, event: winit::event::DeviceEvent) {
                $get_macro!($call_macro, self, on_device_event?, event_loop, device_id, event);
            }
        
            fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
                $get_macro!($call_macro, self, on_about_to_wait?, event_loop);
            }
        
            fn suspended(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
                $get_macro!($call_macro, self, on_suspended?, event_loop);
            }
        
            fn exiting(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
                $get_macro!($call_macro, self, on_exiting?, event_loop);
            }
        
            fn memory_warning(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
                $get_macro!($call_macro, self, on_memory_warning?, event_loop);
            }
        }
    };
}

macro_rules! get_direct {
    ($call_macro: ident, $self: ident, $func: ident, $($args: expr), +) => {
        let Self { $func: $func, .. } = &$self;
        $call_macro!($func $(, $args)+);
    };
    ($call_macro: ident, $self: ident, $func: ident?, $($args: expr), +) => {
        let Self { $func: Some($func), .. } = &$self else { return; };
        $call_macro!($func $(, $args)+);
    };
}

macro_rules! call_fx_sync {
    ($fx: ident, $($args: expr), +) => {
        let result = $fx.call(FnArgs::from(($(From::from($args), )+)));
        handle_res!(result);
    }
}

macro_rules! call_fx_async {
    ($fx: ident, $($args: expr), +) => {
        let result = $fx.call(FnArgs::from(($(From::from($args), )+)));
        handle_rop!(spawn(Some(promise) @ result));
    }
}

impl_with_call_macro!(impl <'scope> UserPayload => OptionsFxHolder<'scope, Unknown<'scope>> | get_direct + call_fx_sync);

impl_with_call_macro!(impl <'scope> UserPayload => OptionsFxHolder<'scope, Option<Promise<()>>> | get_direct + call_fx_async);

macro_rules! get_with_env {
    ($call_macro: ident, $self: ident, $func: ident, $($args: expr), +) => {
        let Self { env, options: OptionsGhostHolder { $func: $func, .. } } = &$self;
        $call_macro!($func@env $(, $args)+);
    };
    ($call_macro: ident, $self: ident, $func: ident?, $($args: expr), +) => {
        let Self { env, options: OptionsGhostHolder { $func: Some($func), .. } } = &$self else { return; };
        $call_macro!($func@env $(, $args)+);
    };
}

macro_rules! call_ref_sync {
    ($fx: ident@$env: ident, $($args: expr), +) => {
        let $fx = $fx.borrow_back($env).unwrap();
        let result = $fx.call(FnArgs::from(($(From::from($args), )+)));
        handle_res!(result);
    }
}

macro_rules! call_ref_async {
    ($fx: ident@$env: ident, $($args: expr), +) => {
        let $fx = $fx.borrow_back($env).unwrap();
        let result = $fx.call(FnArgs::from(($(From::from($args), )+)));
        handle_rop!(spawn(Some(promise) @ result));
    }
}

impl_with_call_macro!(impl <'scope> UserPayload => OptionsRefHolder<Unknown<'scope>> | get_with_env + call_ref_sync);

impl_with_call_macro!(impl <> UserPayload => OptionsRefHolder<Option<Promise<()>>> | get_with_env + call_ref_async);

macro_rules! call_tsfn {
    ($fx: ident, $($args: expr), +) => {
        let result = $fx.call(FnArgs::from(($(From::from($args), )+)), ThreadsafeFunctionCallMode::NonBlocking);
        if Status::Ok != result { dbg!(result); };
    }
}

impl_with_call_macro!(impl <> UserPayload => OptionsSafeHolder<Option<Promise<()>>> | get_direct + call_tsfn);