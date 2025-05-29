#[napi(js_name = "Extra")]
pub mod namespace {
    use std::future::Future;
    use std::time::Duration;
    use napi::{
        bindgen_prelude::*,
        Error,
        threadsafe_function::ThreadsafeFunction,
    };
    use crate::{extra::time::Timeout, handle_res, THREAD_POOL};

    #[napi]
    pub fn tokio_interval(
        timeout: Timeout,
        #[napi(ts_arg_type = "() => (Promise<void> | void)")]
        exec: Function<(), ()>,
    ) {
        let duration = Duration::from(timeout);
        let task = exec.build_threadsafe_function().build().unwrap();
        spawn(inner_loop(duration, task));
    }

    #[napi]
    pub fn thread_interval(
        timeout: Timeout,
        #[napi(ts_arg_type = "() => (Promise<void> | void)")]
        exec: Function<(), ()>,
    ) {
        let duration = Duration::from(timeout);
        let task = exec.build_threadsafe_function().build().unwrap();
        THREAD_POOL.execute(move || block_on(inner_loop(duration, task)))
    }

    async fn inner_loop(duration: Duration, exec: ThreadsafeFunction<(), (), (), false>) -> () {
        loop {
            let sleep = tokio::time::sleep(duration);
            let result = exec.call_async(()).await;
            handle_res!(result);
            sleep.await
        }
    }
}