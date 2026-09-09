#[napi(js_name = "Extra")]
pub mod namespace {
    use crate::{
        extra::time::{Duration, try_std_duration},
        get_thread_pool, handle_res,
        utils::alias::ThreadsafeNoCallee,
    };
    use napi::bindgen_prelude::*;
    use std::future::Future;
    use std::time::Duration as StdDuration;
    #[napi]

    pub fn tokio_interval(
        duration: Duration,
        #[napi(ts_arg_type = "() => (Promise<void> | void)")] exec: Function<(), ()>,
    ) -> Result<()> {
        let duration = try_std_duration(&duration)?;
        let task = exec.build_threadsafe_function().build().unwrap();
        spawn(inner_loop(duration, task));
        Ok(())
    }

    #[napi]
    pub fn thread_interval(
        duration: Duration,
        #[napi(ts_arg_type = "() => (Promise<void> | void)")] exec: Function<(), ()>,
    ) -> Result<()> {
        let duration = try_std_duration(&duration)?;
        let task = exec.build_threadsafe_function().build().unwrap();
        get_thread_pool().execute(move || block_on(inner_loop(duration, task)));
        Ok(())
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
