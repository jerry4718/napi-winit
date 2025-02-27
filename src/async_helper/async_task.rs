use napi::{
    bindgen_prelude::AsyncTask,
    Env,
    JsUndefined,
    Task,
};
use crate::flat_rop;

struct AsyncSleep {
    millis: f64,
}

impl Task for AsyncSleep {
    type Output = ();
    type JsValue = JsUndefined;

    fn compute(&mut self) -> napi::Result<Self::Output> {
        std::thread::sleep(std::time::Duration::from_millis(self.millis as u64));
        Ok(())
    }

    fn resolve(&mut self, env: Env, output: Self::Output) -> napi::Result<Self::JsValue> {
        env.get_undefined()
    }
}

#[napi]
fn async_sleep(millis: f64) -> AsyncTask<AsyncSleep> {
    AsyncTask::new(AsyncSleep { millis })
}