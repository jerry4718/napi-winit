use std::ops::Deref;
use napi::bindgen_prelude::{FunctionRef, Promise};
use napi::{Env, Error};
use crate::{flat_rop, THREAD_POOL};

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