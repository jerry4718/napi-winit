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
};

#[napi(object, object_to_js = false)]
pub struct ApplicationOptions {
    #[napi(ts_type = "(eventLoop: ActiveEventLoop, cause: StartCause) => (void | Promise<void>)")]
    pub on_new_events: Option<Unknown>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => (void | Promise<void>)")]
    pub on_resumed: Unknown,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop, event: UserPayload) => (void | Promise<void>)")]
    pub on_user_event: Option<Unknown>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop, windowId: WindowId, event: WindowEvent) => (void | Promise<void>)")]
    pub on_window_event: Unknown,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop, deviceId: DeviceId, event: DeviceEvent) => (void | Promise<void>)")]
    pub on_device_event: Option<Unknown>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => (void | Promise<void>)")]
    pub on_about_to_wait: Option<Unknown>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => (void | Promise<void>)")]
    pub on_suspended: Option<Unknown>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => (void | Promise<void>)")]
    pub on_exiting: Option<Unknown>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => (void | Promise<void>)")]
    pub on_memory_warning: Option<Unknown>,
}

pub struct ApplicationOptionsFxAsync<'scope> {
    pub on_new_events: Option<Function<'scope, FnArgs<(ActiveEventLoop, StartCause)>, Option<Promise<()>>>>,
    pub on_resumed: Function<'scope, FnArgs<(ActiveEventLoop,)>, Option<Promise<()>>>,
    pub on_user_event: Option<Function<'scope, FnArgs<(ActiveEventLoop, UserPayload)>, Option<Promise<()>>>>,
    pub on_window_event: Function<'scope, FnArgs<(ActiveEventLoop, WindowId, WindowEvent)>, Option<Promise<()>>>,
    pub on_device_event: Option<Function<'scope, FnArgs<(ActiveEventLoop, DeviceId, DeviceEvent)>, Option<Promise<()>>>>,
    pub on_about_to_wait: Option<Function<'scope, FnArgs<(ActiveEventLoop,)>, Option<Promise<()>>>>,
    pub on_suspended: Option<Function<'scope, FnArgs<(ActiveEventLoop,)>, Option<Promise<()>>>>,
    pub on_exiting: Option<Function<'scope, FnArgs<(ActiveEventLoop,)>, Option<Promise<()>>>>,
    pub on_memory_warning: Option<Function<'scope, FnArgs<(ActiveEventLoop,)>, Option<Promise<()>>>>,
}

pub struct ApplicationOptionsFxSync<'scope> {
    pub on_new_events: Option<Function<'scope, FnArgs<(ActiveEventLoop, StartCause)>, ()>>,
    pub on_resumed: Function<'scope, FnArgs<(ActiveEventLoop,)>, ()>,
    pub on_user_event: Option<Function<'scope, FnArgs<(ActiveEventLoop, UserPayload)>, ()>>,
    pub on_window_event: Function<'scope, FnArgs<(ActiveEventLoop, WindowId, WindowEvent)>, ()>,
    pub on_device_event: Option<Function<'scope, FnArgs<(ActiveEventLoop, DeviceId, DeviceEvent)>, ()>>,
    pub on_about_to_wait: Option<Function<'scope, FnArgs<(ActiveEventLoop,)>, ()>>,
    pub on_suspended: Option<Function<'scope, FnArgs<(ActiveEventLoop,)>, ()>>,
    pub on_exiting: Option<Function<'scope, FnArgs<(ActiveEventLoop,)>, ()>>,
    pub on_memory_warning: Option<Function<'scope, FnArgs<(ActiveEventLoop,)>, ()>>,
}

#[napi(object, object_to_js = false)]
pub struct ApplicationOptionsRefAsync {
    pub on_new_events: Option<FunctionRef<FnArgs<(ActiveEventLoop, StartCause)>, Option<Promise<()>>>>,
    pub on_resumed: FunctionRef<FnArgs<(ActiveEventLoop,)>, Option<Promise<()>>>,
    pub on_user_event: Option<FunctionRef<FnArgs<(ActiveEventLoop, UserPayload)>, Option<Promise<()>>>>,
    pub on_window_event: FunctionRef<FnArgs<(ActiveEventLoop, WindowId, WindowEvent)>, Option<Promise<()>>>,
    pub on_device_event: Option<FunctionRef<FnArgs<(ActiveEventLoop, DeviceId, DeviceEvent)>, Option<Promise<()>>>>,
    pub on_about_to_wait: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, Option<Promise<()>>>>,
    pub on_suspended: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, Option<Promise<()>>>>,
    pub on_exiting: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, Option<Promise<()>>>>,
    pub on_memory_warning: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, Option<Promise<()>>>>,
}

#[napi(object, object_to_js = false)]
pub struct ApplicationOptionsRefSync {
    pub on_new_events: Option<FunctionRef<FnArgs<(ActiveEventLoop, StartCause)>, ()>>,
    pub on_resumed: FunctionRef<FnArgs<(ActiveEventLoop,)>, ()>,
    pub on_user_event: Option<FunctionRef<FnArgs<(ActiveEventLoop, UserPayload)>, ()>>,
    pub on_window_event: FunctionRef<FnArgs<(ActiveEventLoop, WindowId, WindowEvent)>, ()>,
    pub on_device_event: Option<FunctionRef<FnArgs<(ActiveEventLoop, DeviceId, DeviceEvent)>, ()>>,
    pub on_about_to_wait: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, ()>>,
    pub on_suspended: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, ()>>,
    pub on_exiting: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, ()>>,
    pub on_memory_warning: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, ()>>,
}

pub(crate) struct OptionsRefHolder<Return: FromNapiValue> {
    pub(crate) on_new_events: Option<FunctionRef<FnArgs<(ActiveEventLoop, StartCause)>, Return>>,
    pub(crate) on_resumed: FunctionRef<FnArgs<(ActiveEventLoop,)>, Return>,
    pub(crate) on_user_event: Option<FunctionRef<FnArgs<(ActiveEventLoop, UserPayload)>, Return>>,
    pub(crate) on_window_event: FunctionRef<FnArgs<(ActiveEventLoop, WindowId, WindowEvent)>, Return>,
    pub(crate) on_device_event: Option<FunctionRef<FnArgs<(ActiveEventLoop, DeviceId, DeviceEvent)>, Return>>,
    pub(crate) on_about_to_wait: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, Return>>,
    pub(crate) on_suspended: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, Return>>,
    pub(crate) on_exiting: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, Return>>,
    pub(crate) on_memory_warning: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, Return>>,
}

pub(crate) struct OptionsRefWithEnv<Return: FromNapiValue> {
    pub(crate) env: Env,
    pub(crate) options: OptionsRefHolder<Return>,
}

pub(crate) struct OptionsFxHolder<'scope, Return: FromNapiValue> {
    pub on_new_events: Option<Function<'scope, FnArgs<(ActiveEventLoop, StartCause)>, Return>>,
    pub on_resumed: Function<'scope, FnArgs<(ActiveEventLoop,)>, Return>,
    pub on_user_event: Option<Function<'scope, FnArgs<(ActiveEventLoop, UserPayload)>, Return>>,
    pub on_window_event: Function<'scope, FnArgs<(ActiveEventLoop, WindowId, WindowEvent)>, Return>,
    pub on_device_event: Option<Function<'scope, FnArgs<(ActiveEventLoop, DeviceId, DeviceEvent)>, Return>>,
    pub on_about_to_wait: Option<Function<'scope, FnArgs<(ActiveEventLoop,)>, Return>>,
    pub on_suspended: Option<Function<'scope, FnArgs<(ActiveEventLoop,)>, Return>>,
    pub on_exiting: Option<Function<'scope, FnArgs<(ActiveEventLoop,)>, Return>>,
    pub on_memory_warning: Option<Function<'scope, FnArgs<(ActiveEventLoop,)>, Return>>,
}

macro_rules! borrow_back {
    ($ty: ident { $from: ident @ $env: ident }) => {
        $ty {
            on_new_events: borrow_back!($from ( on_new_events ? @ $env )),
            on_resumed: borrow_back!($from ( on_resumed @ $env )),
            on_user_event: borrow_back!($from ( on_user_event ? @ $env )),
            on_window_event: borrow_back!($from ( on_window_event @ $env )),
            on_device_event: borrow_back!($from ( on_device_event ? @ $env )),
            on_about_to_wait: borrow_back!($from ( on_about_to_wait ? @ $env )),
            on_suspended: borrow_back!($from ( on_suspended ? @ $env )),
            on_exiting: borrow_back!($from ( on_exiting ? @ $env )),
            on_memory_warning: borrow_back!($from ( on_memory_warning ? @ $env )),
        }
    };
    ($from: ident ($name: ident? @ $env: ident)) => {
        if let Some(cb) = (&$from.$name) { Some(cb.borrow_back(&$env).unwrap()) } else { None }
    };
    ($from: ident ($name: ident @ $env: ident)) => {
        (&$from.$name).borrow_back(&$env).unwrap()
    };
}

impl<Return: FromNapiValue> OptionsRefWithEnv<Return> {
    #[inline]
    pub(crate) fn borrow_back(&self) -> OptionsFxHolder<Return> {
        let Self { env, options } = self;
        borrow_back!(OptionsFxHolder { options @ env })
    }
}

impl<Return: FromNapiValue> OptionsRefHolder<Return> {
    #[inline]
    pub(crate) fn borrow_back<'scope>(&self, env: &'scope Env) -> OptionsFxHolder<'scope, Return> {
        borrow_back!(OptionsFxHolder { self @ env })
    }

    #[inline]
    pub(crate) fn with_env(&self, env: Env) -> OptionsRefWithEnv<Return> {
        let options = self.borrow_back(&env).create_ref();
        OptionsRefWithEnv { env, options }
    }
}

macro_rules! create_ref {
    ($ty: ident { $from: ident @ $env: ident }) => {
        $ty {
            on_new_events: create_ref!($from ( on_new_events ? @ $env )),
            on_resumed: create_ref!($from ( on_resumed @ $env )),
            on_user_event: create_ref!($from ( on_user_event ? @ $env )),
            on_window_event: create_ref!($from ( on_window_event @ $env )),
            on_device_event: create_ref!($from ( on_device_event ? @ $env )),
            on_about_to_wait: create_ref!($from ( on_about_to_wait ? @ $env )),
            on_suspended: create_ref!($from ( on_suspended ? @ $env )),
            on_exiting: create_ref!($from ( on_exiting ? @ $env )),
            on_memory_warning: create_ref!($from ( on_memory_warning ? @ $env )),
        }
    };
    ($from: ident ($name: ident? @ $env: ident)) => {
        (&$from.$name).as_ref().map(|r| r.create_ref().unwrap())
    };
    ($from: ident ($name: ident @ $env: ident)) => {
        (&$from.$name).create_ref().unwrap()
    };
}

impl<'scope, Return: FromNapiValue> OptionsFxHolder<'scope, Return> {
    #[inline]
    pub(crate) fn create_ref(&self) -> OptionsRefHolder<Return> {
        create_ref!(OptionsRefHolder { self @ env })
    }
}

macro_rules! direct_refs {
    ($ty: ident { $from: ident }) => {
        $ty {
            on_new_events: $from.on_new_events,
            on_resumed: $from.on_resumed,
            on_user_event: $from.on_user_event,
            on_window_event: $from.on_window_event,
            on_device_event: $from.on_device_event,
            on_about_to_wait: $from.on_about_to_wait,
            on_suspended: $from.on_suspended,
            on_exiting: $from.on_exiting,
            on_memory_warning: $from.on_memory_warning,
        }
    };
}

impl<'scope> From<ApplicationOptionsFxAsync<'scope>> for OptionsFxHolder<'scope, Option<Promise<()>>> {
    fn from(options: ApplicationOptionsFxAsync<'scope>) -> Self {
        direct_refs!(Self { options })
    }
}

impl<'scope> From<ApplicationOptionsFxSync<'scope>> for OptionsFxHolder<'scope, ()> {
    fn from(options: ApplicationOptionsFxSync<'scope>) -> Self {
        direct_refs!(Self { options })
    }
}

impl From<ApplicationOptionsRefAsync> for OptionsRefHolder<Option<Promise<()>>> {
    fn from(options: ApplicationOptionsRefAsync) -> Self {
        direct_refs!(Self { options })
    }
}

impl From<ApplicationOptionsRefSync> for OptionsRefHolder<()> {
    fn from(options: ApplicationOptionsRefSync) -> Self {
        direct_refs!(Self { options })
    }
}

pub(crate) enum Runner {
    AsyncEnvRef(OptionsRefWithEnv<Option<Promise<()>>>),
    Async(OptionsRefHolder<Option<Promise<()>>>),
    AsyncRef(OptionsRefHolder<Option<Promise<()>>>),
    SyncEnvRef(OptionsRefWithEnv<()>),
    Sync(OptionsRefHolder<()>),
    SyncRef(OptionsRefHolder<()>),
}

#[napi]
pub struct Application {
    pub(crate) runner: Runner,
}

#[napi]
impl Application {
    #[napi(factory)]
    pub fn with_async_env_ref(env: Env, #[napi(ts_arg_type = "ApplicationOptions")] options: ApplicationOptionsRefAsync) -> Self {
        let runner = Runner::AsyncEnvRef(OptionsRefWithEnv { env, options: From::from(options) });
        Self { runner }
    }
    #[napi(factory)]
    pub fn with_sync_env_ref(env: Env, #[napi(ts_arg_type = "ApplicationOptions")] options: ApplicationOptionsRefSync) -> Self {
        let runner = Runner::SyncEnvRef(OptionsRefWithEnv { env, options: From::from(options) });
        Self { runner }
    }
}

#[napi]
impl Application {
    #[napi(factory)]
    pub fn with_async_ref(env: Env, #[napi(ts_arg_type = "ApplicationOptions")] options: ApplicationOptionsRefAsync) -> Self {
        let runner = Runner::AsyncRef(From::from(options));
        Self { runner }
    }
    #[napi(factory)]
    pub fn with_sync_ref(env: Env, #[napi(ts_arg_type = "ApplicationOptions")] options: ApplicationOptionsRefSync) -> Self {
        let runner = Runner::SyncRef(From::from(options));
        Self { runner }
    }
}

#[napi]
impl Application {
    #[napi(factory)]
    pub fn with_async(env: Env, #[napi(ts_arg_type = "ApplicationOptions")] options: ApplicationOptionsRefAsync) -> Self {
        let runner = Runner::Async(From::from(options));
        Self { runner }
    }
    #[napi(factory)]
    pub fn with_sync(env: Env, #[napi(ts_arg_type = "ApplicationOptions")] options: ApplicationOptionsRefSync) -> Self {
        let runner = Runner::Sync(From::from(options));
        Self { runner }
    }
}