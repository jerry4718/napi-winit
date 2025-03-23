use std::ops::Deref;
use std::time::Duration;
use napi::bindgen_prelude::{block_on, spawn_blocking, FunctionRef, Promise};
use napi::{Env, Error};
use napi::threadsafe_function::ThreadsafeFunction;
use crate::{flat_rop, THREAD_POOL};
use crate::extra::time::Timeout;

#[napi]
pub struct ThreadPool {
    pub(crate) pool: threadpool::ThreadPool
}

#[napi]
impl ThreadPool {
    #[napi(constructor)]
    pub fn new(num_threads: u32) -> ThreadPool {
        Self { pool: threadpool::ThreadPool::new(num_threads as usize) }
    }
    #[napi(factory)]
    pub fn default() -> ThreadPool {
        Self { pool: Default::default() }
    }
    #[napi(factory)]
    pub fn main() -> Self {
        Self { pool: THREAD_POOL.clone() }
    }

    #[napi(ts_args_type = "callback: () => (Promise<void> | void)")]
    pub fn execute(&self, env: Env, task: FunctionRef<(), Option<Promise<()>>>) {
        let tsfn = task.borrow_back(&env).unwrap()
            .build_threadsafe_function()
            .build_callback(|ctx| ctx.env.get_undefined())
            .unwrap();

        self.pool.execute(move || {
            let result = pollster::block_on(tsfn.call_async(()));
            match flat_rop!(result: Some(promise) => pollster::block_on(promise)) {
                Ok(_) => (),
                Err(Error { status, reason, .. }) => {
                    println!("status: {}, reason: {}", status, reason);
                }
            }
        });
    }
}

#[napi]
pub fn thread_interval(
    timeout: Timeout,
    #[napi(ts_arg_type = "() => (Promise<void> | void)")]
    exec: ThreadsafeFunction<(), Option<Promise<()>>>,
) {
    let duration = Duration::from(timeout);
    THREAD_POOL.execute(move || {
        block_on(async {
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
    })
}

#[napi]
pub fn pollster_interval(
    timeout: Timeout,
    #[napi(ts_arg_type = "() => (Promise<void> | void)")]
    exec: ThreadsafeFunction<(), Option<Promise<()>>>,
) {
    let duration = Duration::from(timeout);
    THREAD_POOL.execute(move || {
        loop {
            let sleep = tokio::time::sleep(duration);
            let result = pollster::block_on(exec.call_async(Ok(())));

            match flat_rop!(result: Some(promise) => pollster::block_on(promise)) {
                Ok(_) => (),
                Err(Error { status, reason, .. }) => {
                    println!("status: {}, reason: {}", status, reason);
                    break;
                }
            };

            pollster::block_on(sleep)
        }
    })
}