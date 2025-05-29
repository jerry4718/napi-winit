use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunction;
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
pub struct ApplicationOptions<'env> {
    #[napi(ts_type = "(eventLoop: ActiveEventLoop, cause: StartCause) => (void | Promise<void>)")]
    pub on_new_events: Option<Unknown<'env>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => (void | Promise<void>)")]
    pub on_resumed: Unknown<'env>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop, event: UserPayload) => (void | Promise<void>)")]
    pub on_user_event: Option<Unknown<'env>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop, windowId: WindowId, event: WindowEvent) => (void | Promise<void>)")]
    pub on_window_event: Unknown<'env>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop, deviceId: DeviceId, event: DeviceEvent) => (void | Promise<void>)")]
    pub on_device_event: Option<Unknown<'env>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => (void | Promise<void>)")]
    pub on_about_to_wait: Option<Unknown<'env>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => (void | Promise<void>)")]
    pub on_suspended: Option<Unknown<'env>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => (void | Promise<void>)")]
    pub on_exiting: Option<Unknown<'env>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => (void | Promise<void>)")]
    pub on_memory_warning: Option<Unknown<'env>>,
}

#[napi(object, object_to_js = false)]
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

#[napi(object, object_to_js = false)]
pub struct ApplicationOptionsFxSync<'scope> {
    pub on_new_events: Option<Function<'scope, FnArgs<(ActiveEventLoop, StartCause)>, Unknown<'scope>>>,
    pub on_resumed: Function<'scope, FnArgs<(ActiveEventLoop,)>, Unknown<'scope>>,
    pub on_user_event: Option<Function<'scope, FnArgs<(ActiveEventLoop, UserPayload)>, Unknown<'scope>>>,
    pub on_window_event: Function<'scope, FnArgs<(ActiveEventLoop, WindowId, WindowEvent)>, Unknown<'scope>>,
    pub on_device_event: Option<Function<'scope, FnArgs<(ActiveEventLoop, DeviceId, DeviceEvent)>, Unknown<'scope>>>,
    pub on_about_to_wait: Option<Function<'scope, FnArgs<(ActiveEventLoop,)>, Unknown<'scope>>>,
    pub on_suspended: Option<Function<'scope, FnArgs<(ActiveEventLoop,)>, Unknown<'scope>>>,
    pub on_exiting: Option<Function<'scope, FnArgs<(ActiveEventLoop,)>, Unknown<'scope>>>,
    pub on_memory_warning: Option<Function<'scope, FnArgs<(ActiveEventLoop,)>, Unknown<'scope>>>,
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
pub struct ApplicationOptionsRefSync<'scope> {
    pub on_new_events: Option<FunctionRef<FnArgs<(ActiveEventLoop, StartCause)>, Unknown<'scope>>>,
    pub on_resumed: FunctionRef<FnArgs<(ActiveEventLoop,)>, Unknown<'scope>>,
    pub on_user_event: Option<FunctionRef<FnArgs<(ActiveEventLoop, UserPayload)>, Unknown<'scope>>>,
    pub on_window_event: FunctionRef<FnArgs<(ActiveEventLoop, WindowId, WindowEvent)>, Unknown<'scope>>,
    pub on_device_event: Option<FunctionRef<FnArgs<(ActiveEventLoop, DeviceId, DeviceEvent)>, Unknown<'scope>>>,
    pub on_about_to_wait: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, Unknown<'scope>>>,
    pub on_suspended: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, Unknown<'scope>>>,
    pub on_exiting: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, Unknown<'scope>>>,
    pub on_memory_warning: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, Unknown<'scope>>>,
}

pub(crate) struct OptionsGhostHolder<Return: FromNapiValue> {
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

pub(crate) struct OptionsRefHolder<Return: FromNapiValue> {
    pub(crate) env: Env,
    pub(crate) options: OptionsGhostHolder<Return>,
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

type FunctionSafe<T, R> = ThreadsafeFunction<T, R, T, false>;

pub(crate) struct OptionsSafeHolder<Return: 'static + FromNapiValue> {
    pub on_new_events: Option<FunctionSafe<FnArgs<(ActiveEventLoop, StartCause)>, Return>>,
    pub on_resumed: FunctionSafe<FnArgs<(ActiveEventLoop,)>, Return>,
    pub on_user_event: Option<FunctionSafe<FnArgs<(ActiveEventLoop, UserPayload)>, Return>>,
    pub on_window_event: FunctionSafe<FnArgs<(ActiveEventLoop, WindowId, WindowEvent)>, Return>,
    pub on_device_event: Option<FunctionSafe<FnArgs<(ActiveEventLoop, DeviceId, DeviceEvent)>, Return>>,
    pub on_about_to_wait: Option<FunctionSafe<FnArgs<(ActiveEventLoop,)>, Return>>,
    pub on_suspended: Option<FunctionSafe<FnArgs<(ActiveEventLoop,)>, Return>>,
    pub on_exiting: Option<FunctionSafe<FnArgs<(ActiveEventLoop,)>, Return>>,
    pub on_memory_warning: Option<FunctionSafe<FnArgs<(ActiveEventLoop,)>, Return>>,
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

impl<Return: FromNapiValue> OptionsRefHolder<Return> {
    #[inline]
    pub(crate) fn borrow_back(&self) -> OptionsFxHolder<Return> {
        let Self { env, options } = self;
        borrow_back!(OptionsFxHolder { options @ env })
    }
}

impl<Return: FromNapiValue> OptionsGhostHolder<Return> {
    #[inline]
    pub(crate) fn borrow_back<'scope>(&self, env: &'scope Env) -> OptionsFxHolder<'scope, Return> {
        borrow_back!(OptionsFxHolder { self @ env })
    }

    #[inline]
    pub(crate) fn with_env(&self, env: Env) -> OptionsRefHolder<Return> {
        let options = self.borrow_back(&env).create_ref();
        OptionsRefHolder { env, options }
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
    pub(crate) fn create_ref(&self) -> OptionsGhostHolder<Return> {
        create_ref!(OptionsGhostHolder { self @ env })
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

impl<'scope> From<ApplicationOptionsFxSync<'scope>> for OptionsFxHolder<'scope, Unknown<'scope>> {
    fn from(options: ApplicationOptionsFxSync<'scope>) -> Self {
        direct_refs!(Self { options })
    }
}

impl From<ApplicationOptionsRefAsync> for OptionsGhostHolder<Option<Promise<()>>> {
    fn from(options: ApplicationOptionsRefAsync) -> Self {
        direct_refs!(Self { options })
    }
}

impl<'scope> From<ApplicationOptionsRefSync<'scope>> for OptionsGhostHolder<Unknown<'scope>> {
    fn from(options: ApplicationOptionsRefSync<'scope>) -> Self {
        direct_refs!(Self { options })
    }
}

macro_rules! build_threadsafe {
    ($ty: ident { $from: ident }) => {
        $ty {
            on_new_events: build_threadsafe!($from ( on_new_events ? )),
            on_resumed: build_threadsafe!($from ( on_resumed )),
            on_user_event: build_threadsafe!($from ( on_user_event ? )),
            on_window_event: build_threadsafe!($from ( on_window_event )),
            on_device_event: build_threadsafe!($from ( on_device_event ? )),
            on_about_to_wait: build_threadsafe!($from ( on_about_to_wait ? )),
            on_suspended: build_threadsafe!($from ( on_suspended ? )),
            on_exiting: build_threadsafe!($from ( on_exiting ? )),
            on_memory_warning: build_threadsafe!($from ( on_memory_warning ? )),
        }
    };
    ($from: ident ($name: ident?)) => {
        if let Some(cb) = (&$from.$name) { Some(cb.build_threadsafe_function().build().unwrap()) } else { None }
    };
    ($from: ident ($name: ident)) => {
        (&$from.$name).build_threadsafe_function().build().unwrap()
    };
}

impl<'scope> From<ApplicationOptionsFxAsync<'scope>> for OptionsSafeHolder<Option<Promise<()>>> {
    fn from(options: ApplicationOptionsFxAsync<'scope>) -> Self {
        build_threadsafe!(OptionsSafeHolder { options })
    }
}

pub(crate) enum Runner<'env> {
    AsyncFx(OptionsFxHolder<'env, Option<Promise<()>>>),
    AsyncRef(OptionsRefHolder<Option<Promise<()>>>),
    AsyncFx2Ref(OptionsGhostHolder<Option<Promise<()>>>),
    AsyncRef2Fx(OptionsGhostHolder<Option<Promise<()>>>),
    AsyncFxSafe(OptionsSafeHolder<Option<Promise<()>>>),
    SyncFx(OptionsFxHolder<'env, Unknown<'env>>),
    SyncRef(OptionsRefHolder<Unknown<'env>>),
    SyncFx2Ref(OptionsGhostHolder<Unknown<'env>>),
    SyncRef2Fx(OptionsGhostHolder<Unknown<'env>>),
}

#[napi]
pub struct Application<'env> {
    pub(crate) runner: Runner<'env>,
}

#[napi]
impl<'env> Application<'env> {
    #[napi(factory)]
    pub fn with_async_ref(env: Env, #[napi(ts_arg_type = "ApplicationOptions")] options: ApplicationOptionsRefAsync) -> Self {
        let runner = Runner::AsyncRef(OptionsRefHolder { env, options: From::from(options) });
        Self { runner }
    }
    #[napi(factory)]
    pub fn with_sync_ref(env: Env, #[napi(ts_arg_type = "ApplicationOptions")] options: ApplicationOptionsRefSync<'env>) -> Self {
        let runner = Runner::SyncRef(OptionsRefHolder { env, options: From::from(options) });
        Self { runner }
    }
}

#[napi]
impl<'env> Application<'env> {
    #[napi(factory)]
    pub fn with_async_ref_2_fx(env: Env, #[napi(ts_arg_type = "ApplicationOptions")] options: ApplicationOptionsRefAsync) -> Self {
        let runner = Runner::AsyncRef2Fx(From::from(options));
        Self { runner }
    }
    #[napi(factory)]
    pub fn with_sync_ref_2_fx(env: Env, #[napi(ts_arg_type = "ApplicationOptions")] options: ApplicationOptionsRefSync<'env>) -> Self {
        let runner = Runner::SyncRef2Fx(From::from(options));
        Self { runner }
    }
}

#[napi]
impl<'env> Application<'env> {
    #[napi(factory)]
    pub fn with_async_fx(env: Env, #[napi(ts_arg_type = "ApplicationOptions")] options: ApplicationOptionsFxAsync<'env>) -> Self {
        let runner = Runner::AsyncFx(From::from(options));
        Self { runner }
    }
    #[napi(factory)]
    pub fn with_sync_fx(env: Env, #[napi(ts_arg_type = "ApplicationOptions")] options: ApplicationOptionsFxSync<'env>) -> Self {
        let runner = Runner::SyncFx(From::from(options));
        Self { runner }
    }
}

#[napi]
impl<'env> Application<'env> {
    #[napi(factory)]
    pub fn with_async_fx_2_ref(env: Env, #[napi(ts_arg_type = "ApplicationOptions")] options: ApplicationOptionsRefAsync) -> Self {
        let runner = Runner::AsyncFx2Ref(From::from(options));
        Self { runner }
    }
    #[napi(factory)]
    pub fn with_sync_fx_2_ref(env: Env, #[napi(ts_arg_type = "ApplicationOptions")] options: ApplicationOptionsRefSync<'env>) -> Self {
        let runner = Runner::SyncFx2Ref(From::from(options));
        Self { runner }
    }
}

#[napi]
impl<'env> Application<'env> {
    #[napi(factory)]
    pub fn with_async_fx_2_safe(env: Env, #[napi(ts_arg_type = "ApplicationOptions")] options: ApplicationOptionsFxAsync<'env>) -> Self {
        let runner = Runner::AsyncFxSafe(From::from(options));
        Self { runner }
    }
}