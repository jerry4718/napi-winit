#[napi(js_name = "Extra")]
pub mod namespace {
    use crate::{
        extra::time::{Duration, try_std_duration},
        handle_res,
        utils::alias::ThreadsafeNoCallee,
    };
    use napi::bindgen_prelude::*;
    use std::{
        sync::{
            Arc,
            atomic::{AtomicBool, Ordering},
        },
        time::Duration as StdDuration,
    };

    #[napi]
    pub async fn tokio_sleep(duration: Duration) -> Result<()> {
        tokio::time::sleep(try_std_duration(&duration)?).await;
        Ok(())
    }

    #[napi]
    pub struct IntervalStopper {
        flag: Arc<AtomicBool>,
    }

    #[napi]
    impl IntervalStopper {
        #[napi]
        pub fn stop(&self) {
            self.flag.store(true, Ordering::Relaxed)
        }
    }

    #[napi]
    pub fn tokio_interval(
        duration: Duration,
        #[napi(ts_arg_type = "() => (Promise<void> | void)")] exec: Function<(), ()>,
    ) -> Result<IntervalStopper> {
        let duration = try_std_duration(&duration)?;
        let task = exec.build_threadsafe_function().build()?;
        let flag = Arc::new(AtomicBool::new(false));
        let stop_flag = flag.clone();
        spawn(async move {
            loop {
                if flag.load(Ordering::Relaxed) {
                    break;
                }
                let result = task.call_async(()).await;
                handle_res!(result);
                tokio::time::sleep(duration).await;
            }
        });
        Ok(IntervalStopper { flag: stop_flag })
    }
}
