use napi::bindgen_prelude::*;
use crate::{
    flat_rop,
    handle_rop,
};

#[napi]
pub async fn tokio_sleep(millis: f64) {
    tokio::time::sleep(std::time::Duration::from_millis(millis as u64)).await;
}

#[napi]
pub fn tokio_spawn(promise: Promise<()>) {
    tokio::runtime::Handle::current().spawn(async move {
        let Err(Error { status, reason, .. }) = promise.await else { return };
        println!("status: {}, reason: {}", status, reason);
    });
}

#[napi]
pub fn tokio_block_on(promise: Promise<()>) -> Result<()> {
    tokio::runtime::Handle::current().block_on(promise)
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

#[napi(ts_args_type = "callback: () => (Promise<void> | void)")]
pub fn tokio_call_block_on<'a>(task: Function<'a, (), Option<Promise<()>>>) {
    let result = task.call(());
    handle_rop!(block_on(Some(promise) @ result));
}