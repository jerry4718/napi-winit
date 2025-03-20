use std::time::Duration;
use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunction;
use crate::{
    flat_rop,
    handle_rop,
    extra::time::Timeout
};

#[napi]
pub async fn tokio_sleep(timeout: Timeout) {
    tokio::time::sleep(timeout.into()).await;
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
pub fn tokio_interval(
    timeout: Timeout,
    #[napi(ts_arg_type = "() => (Promise<void> | void)")]
    exec: ThreadsafeFunction<(), Option<Promise<()>>>
) {
    let duration = Duration::from(timeout);
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