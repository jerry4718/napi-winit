#[inline]
fn flat_result<T, E>(result: Result<T, E>) -> (Option<E>, Option<T>) {
    match result {
        Ok(res) => (None, Some(res)),
        Err(err) => (Some(err), None),
    }
}

#[napi(js_name = "Extra")]
pub mod namespace {
    use super::*;
    use crate::{
        window::Window,
        napi_reason,
        ok_or_reason,
        utils::alias::ThreadsafeNoCallee,
    };
    use napi::{
        bindgen_prelude::*,
        threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
    };
    use softbuffer::{Context, Surface};
    use std::{
        alloc::{alloc, dealloc, Layout},
        num::{NonZero, NonZeroU32},
        ptr::NonNull,
        slice,
    };
    use std::ops::DerefMut;

    #[napi]
    struct BufferSurface<'scope> {
        pub(crate) window: &'scope winit::window::Window,
        pub(crate) context: Option<Context<&'scope winit::window::Window>>,
        pub(crate) surface: Option<Surface<&'scope winit::window::Window, &'scope winit::window::Window>>,
    }

    #[napi]
    impl<'scope> BufferSurface<'scope> {
        #[napi(constructor)]
        pub fn new(window: &'scope mut Window) -> Self {
            Self {
                window: &mut window.inner,
                context: None,
                surface: None,
            }
        }
    }

    #[napi]
    impl BufferSurface<'_> {
        #[napi]
        pub fn present_with_typed(&mut self, input: Uint32Array) -> Result<()> {
            let src_len = input.len();

            self.present(|width, height, buffer| {
                let dest_len = buffer.len();

                if src_len != dest_len {
                    return Err(napi_reason!("source slice length ({src_len}) does not match destination slice length ({dest_len})"));
                }

                buffer.copy_from_slice(input.as_ref());
                Ok(())
            })
        }

        #[napi]
        pub fn present_with_writer<'scope>(
            &mut self, env: Env,
            #[napi(ts_arg_type = "(width: number, height: number, view: Uint32Array) => void")]
            write: Function<'scope, FnArgs<(u32, u32, Uint32Array)>, ()>,
        ) -> Result<()> {
            self.present(|width, height, buffer| {
                let buf_len = buffer.len();
                let buf_slice = buffer.deref_mut();

                let view = unsafe {
                    Uint32Array::with_external_data(buf_slice.as_mut_ptr(), buf_len, move |ptr, size| {})
                };
                ok_or_reason!(write.call(FnArgs::from((width.get(), height.get(), view))));
                Ok(())
            })
        }

        #[napi]
        pub fn present_with_threadsafe_writer<'scope>(
            &mut self, env: Env,
            #[napi(ts_arg_type = "(width: number, height: number, view: Uint32Array) => void")]
            write: ThreadsafeNoCallee<FnArgs<(u32, u32, Uint32Array)>, ()>,
        ) -> Result<()> {
            self.present(|width, height, buffer| {
                let buf_len = buffer.len();
                let buf_slice = buffer.deref_mut();

                let view = unsafe {
                    Uint32Array::with_external_data(buf_slice.as_mut_ptr(), buf_len, move |ptr, size| {})
                };

                let status = write.call(FnArgs::from((width.get(), height.get(), view)), ThreadsafeFunctionCallMode::Blocking);
                if Status::Ok != status { dbg!(status); };
                Ok(())
            })
        }
    }

    impl BufferSurface<'_> {
        pub(crate) fn present<F>(&mut self, mut write_fn: F) -> Result<()>
        where
            F: FnMut(NonZero<u32>, NonZero<u32>, &mut softbuffer::Buffer<&winit::window::Window, &winit::window::Window>) -> Result<()>,
        {
            let context = match self.context {
                Some(ref mut context) => context,
                None => match Context::new(self.window) {
                    Ok(context) => self.context.insert(context),
                    Err(e) => return Err(napi_reason!("Failed to create buffer context: {e}")),
                }
            };

            let surface = match self.surface {
                Some(ref mut surface) => surface,
                None => match Surface::new(&context, self.window) {
                    Ok(surface) => self.surface.insert(surface),
                    Err(e) => return Err(napi_reason!("Failed to create buffer surface: {e}")),
                }
            };

            let size = self.window.inner_size();

            let Some(width) = NonZeroU32::new(size.width)
            else { return Err(napi_reason!("invalid window size [width: {}]", size.width)) };

            let Some(height) = NonZeroU32::new(size.height)
            else { return Err(napi_reason!("invalid window size [height: {}]", size.height)) };

            if let Err(e) = surface.resize(width, height) {
                return Err(napi_reason!("failed to resize surface: {e}"));
            }

            let (err, res) = flat_result(surface.buffer_mut());

            if let Some(e) = err {
                return Err(napi_reason!("failed to access buffer: {e}"));
            }

            let Some(mut buffer) = res else { unreachable!("never handled") };

            ok_or_reason!(write_fn(width, height, &mut buffer); "{}");

            buffer.present()
                .map_err(|e| napi_reason!("failed to access buffer: {e}"))
        }
    }
}