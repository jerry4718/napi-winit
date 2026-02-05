#[napi(js_name = "Extra")]
pub mod namespace {
    use std::future::Future;
    use std::time::Duration as StdDuration;
    use napi::bindgen_prelude::*;
    use crate::{
        extra::time::Duration,
        handle_res,
        get_thread_pool,
        utils::alias::ThreadsafeNoCallee,
    };
    #[napi]

    pub fn tokio_interval(
        duration: Duration,
        #[napi(ts_arg_type = "() => (Promise<void> | void)")]
        exec: Function<(), ()>,
    ) {
        let duration = StdDuration::from(duration);
        let task = exec.build_threadsafe_function().build().unwrap();
        spawn(inner_loop(duration, task));
    }

    #[napi]
    pub fn thread_interval(
        duration: Duration,
        #[napi(ts_arg_type = "() => (Promise<void> | void)")]
        exec: Function<(), ()>,
    ) {
        let duration = StdDuration::from(duration);
        let task = exec.build_threadsafe_function().build().unwrap();
        get_thread_pool().execute(move || block_on(inner_loop(duration, task)))
    }

    async fn inner_loop(duration: StdDuration, exec: ThreadsafeNoCallee<(), ()>) -> () {
        loop {
            let sleep = tokio::time::sleep(duration);
            let result = exec.call_async(()).await;
            handle_res!(result);
            sleep.await
        }
    }
}