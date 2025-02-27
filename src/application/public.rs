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
pub struct ApplicationOptions<'scope> {
    #[napi(ts_type = "(eventLoop: ActiveEventLoop, cause: StartCause) => void")]
    pub on_new_events: Option<Function<'scope, FnArgs<(ActiveEventLoop, StartCause)>, Option<Promise<()>>>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => void")]
    pub on_resumed: Function<'scope, FnArgs<(ActiveEventLoop,)>, Option<Promise<()>>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop, event: UserPayload) => void")]
    pub on_user_event: Option<Function<'scope, FnArgs<(ActiveEventLoop, UserPayload)>, Option<Promise<()>>>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop, windowId: WindowId, event: WindowEvent) => void")]
    pub on_window_event: Function<'scope, FnArgs<(ActiveEventLoop, WindowId, WindowEvent)>, Option<Promise<()>>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop, deviceId: DeviceId, event: DeviceEvent) => void")]
    pub on_device_event: Option<Function<'scope, FnArgs<(ActiveEventLoop, DeviceId, DeviceEvent)>, Option<Promise<()>>>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => void")]
    pub on_about_to_wait: Option<Function<'scope, FnArgs<(ActiveEventLoop,)>, Option<Promise<()>>>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => void")]
    pub on_suspended: Option<Function<'scope, FnArgs<(ActiveEventLoop,)>, Option<Promise<()>>>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => void")]
    pub on_exiting: Option<Function<'scope, FnArgs<(ActiveEventLoop,)>, Option<Promise<()>>>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => void")]
    pub on_memory_warning: Option<Function<'scope, FnArgs<(ActiveEventLoop,)>, Option<Promise<()>>>>,
}

#[napi(object, object_to_js = false)]
pub struct ApplicationOptionRefs {
    #[napi(ts_type = "(eventLoop: ActiveEventLoop, cause: StartCause) => void")]
    pub on_new_events: Option<FunctionRef<FnArgs<(ActiveEventLoop, StartCause)>, Option<Promise<()>>>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => void")]
    pub on_resumed: FunctionRef<FnArgs<(ActiveEventLoop,)>, Option<Promise<()>>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop, event: UserPayload) => void")]
    pub on_user_event: Option<FunctionRef<FnArgs<(ActiveEventLoop, UserPayload)>, Option<Promise<()>>>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop, windowId: WindowId, event: WindowEvent) => void")]
    pub on_window_event: FunctionRef<FnArgs<(ActiveEventLoop, WindowId, WindowEvent)>, Option<Promise<()>>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop, deviceId: DeviceId, event: DeviceEvent) => void")]
    pub on_device_event: Option<FunctionRef<FnArgs<(ActiveEventLoop, DeviceId, DeviceEvent)>, Option<Promise<()>>>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => void")]
    pub on_about_to_wait: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, Option<Promise<()>>>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => void")]
    pub on_suspended: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, Option<Promise<()>>>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => void")]
    pub on_exiting: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, Option<Promise<()>>>>,
    #[napi(ts_type = "(eventLoop: ActiveEventLoop) => void")]
    pub on_memory_warning: Option<FunctionRef<FnArgs<(ActiveEventLoop,)>, Option<Promise<()>>>>,
}