#[napi(js_name = "Extra")]
pub mod namespace {
    use crate::{handle_res, THREAD_POOL};
    use napi::bindgen_prelude::*;
    use napi::Env;

    #[napi(ts_args_type = "callback: () => (Promise<void> | void)")]
    pub fn tokio_call_spawn(env: Env, exec: Function<(), ()>) {
        let task = exec.build_threadsafe_function().build().unwrap();
        spawn(async move {
            let result = task.call_async(()).await;
            handle_res!(result);
        });
    }

    #[napi]
    pub struct ThreadPool {
        pub(crate) pool: threadpool::ThreadPool,
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
        pub fn execute(&self, env: Env, exec: Function<(), ()>) {
            let task = exec.build_threadsafe_function().build().unwrap();
            self.pool.execute(move ||
                block_on(async {
                    let result = task.call_async(()).await;
                    handle_res!(result);
                })
            );
        }
    }
}