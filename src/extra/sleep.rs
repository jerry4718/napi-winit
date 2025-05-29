#[napi(js_name = "Extra")]
pub mod namespace {
    use napi::bindgen_prelude::*;
    use crate::extra::time::Timeout;

    #[napi]
    pub async fn tokio_sleep(timeout: Timeout) {
        tokio::time::sleep(timeout.into()).await;
    }

    struct AsyncSleep {
        millis: f64,
    }

    impl Task for AsyncSleep {
        type Output = ();
        type JsValue = ();

        fn compute(&mut self) -> Result<Self::Output> {
            std::thread::sleep(std::time::Duration::from_millis(self.millis as u64));
            Ok(())
        }

        fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
            Ok(())
        }
    }

    #[napi]
    fn async_sleep(millis: f64) -> AsyncTask<AsyncSleep> {
        AsyncTask::new(AsyncSleep { millis })
    }
}