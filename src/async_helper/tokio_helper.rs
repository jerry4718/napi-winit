use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunction;
use crate::{
    flat_rop,
    handle_rop,
};

#[napi]
pub async fn tokio_sleep(timeout: &Timeout) {
    tokio::time::sleep(timeout.delay).await;
}

#[napi(ts_args_type = "callback: () => void")]
pub fn tokio_call_spawn(env: Env, task: FunctionRef<(), Option<Promise<()>>>) {
    let tsfn = task
        .borrow_back(&env).unwrap()
        .build_threadsafe_function()
        .build_callback(|ctx| ctx.env.get_undefined())
        .unwrap();

    spawn(async move {
        let result = tsfn.call_async(()).await;

        match flat_rop!(result: Some(promise) => promise.await) {
            Ok(_) => (),
            Err(Error { status, reason, .. }) => {
                println!("status: {}, reason: {}", status, reason);
            }
        };
        ()
    });
}

#[napi]
pub struct Timeout {
    pub(crate) delay: std::time::Duration,
}

#[napi]
impl Timeout {
    #[napi(factory)]
    pub fn from_millis(millis: f64) -> Self {
        Self { delay: std::time::Duration::from_millis(millis as u64) }
    }
    #[napi(factory)]
    pub fn from_micros(micros: f64) -> Self {
        Self { delay: std::time::Duration::from_micros(micros as u64) }
    }
    #[napi(factory)]
    pub fn from_nanos(nanos: f64) -> Self {
        Self { delay: std::time::Duration::from_nanos(nanos as u64) }
    }
}

#[napi]
pub fn tokio_interval(
    timeout: &Timeout,
    #[napi(ts_arg_type = "() => (Promise<void> | void)")]
    exec: ThreadsafeFunction<(), Option<Promise<()>>>
) {
    let duration = timeout.delay.clone();
    spawn(async move {
        loop {
            let sleep = tokio::time::sleep(duration);
            let result = exec.call_async(Ok(())).await;

            match flat_rop!(result: Some(promise) => promise.await) {
                Ok(_) => (),
                Err(Error { status, reason, .. }) => {
                    println!("status: {}, reason: {}", status, reason);
                    break;
                }
            };

            sleep.await
        }
    });
}