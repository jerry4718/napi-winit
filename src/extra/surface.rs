#[inline]
fn flat_result<T, E>(result: Result<T, E>) -> (Option<E>, Option<T>) {
    match result {
        Ok(res) => (None, Some(res)),
        Err(err) => (Some(err), None),
    }
}

#[inline]
fn get_or_insert_with_result<T, E, F>(with: &mut Option<T>, f: F) -> Result<&mut T, E>
where
    F: FnOnce() -> Result<T, E>,
{
    if let Some(ref mut context) = with { return Ok(context); }

    let (err, res) = flat_result(f());

    if let Some(e) = err { return Err(e); }

    Ok(with.insert(res.unwrap()))
}

#[napi(js_name = "Extra")]
pub mod namespace {
    use super::*;
    use crate::{
        window::Window,
        napi_reason,
        ok_or_reason,
        utils::alias::ThreadsafeNoCallee
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

    #[napi]
    struct SoftSurface<'scope> {
        pub(crate) window: &'scope winit::window::Window,
        pub(crate) context: Option<Context<&'scope winit::window::Window>>,
        pub(crate) surface: Option<Surface<&'scope winit::window::Window, &'scope winit::window::Window>>,
    }

    #[napi]
    impl<'scope> SoftSurface<'scope> {
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
    impl SoftSurface<'_> {
        #[napi]
        pub fn present_with_typed(&mut self, input: Uint32Array) -> Result<()> {
            let src_len = input.len();

            op_present(self, |width, height, buffer| {
                let dest_len = buffer.len();

                if src_len != dest_len {
                    return Err(napi_reason!("source slice length ({src_len}) does not match destination slice length ({dest_len})"));
                }

                buffer.copy_from_slice(input.as_ref());
                Ok(())
            })
        }

        #[napi]
        pub fn present_with_buffer(&mut self, buffer: Buffer) -> Result<()> {
            let buf_len = buffer.len();
            if buf_len % 4 != 0 {
                return Err(napi_reason!("input buffer length not align to 32 bits"));
            }

            let src_ptr = buffer.as_ptr().cast::<u32>().cast_mut();
            let src_len = buf_len / 4;

            op_present(self, |width, height, buffer| {
                let dest_len = buffer.len();

                if src_len != dest_len {
                    return Err(napi_reason!("source slice length ({src_len}) does not match destination slice length ({dest_len})"));
                }

                buffer.copy_from_slice(unsafe { slice::from_raw_parts(src_ptr, src_len) });
                Ok(())
            })
        }

        #[napi]
        pub fn present_with_writer<'scope>(
            &mut self, env: Env,
            #[napi(ts_arg_type = "(width: number, height: number, view: Uint32Array) => void")]
            write: Function<'scope, FnArgs<(u32, u32, Uint32Array)>, ()>,
        ) -> Result<()> {
            op_present(self, |width, height, buffer| {
                let buf_len = buffer.len();

                let layout = ok_or_reason!(Layout::array::<u32>(buf_len); "{}");
                let temp = unsafe { alloc(layout) };
                let view = unsafe {
                    Uint32Array::with_external_data(temp.cast::<u32>(), buf_len, move |ptr, size| {
                        println!("surface view was deallocated!!!!!!");
                        dealloc(temp, layout);
                    })
                };
                ok_or_reason!(write.call(FnArgs::from((width.get(), height.get(), view))));
                buffer.copy_from_slice(unsafe { slice::from_raw_parts(temp as *const u32, buf_len) });
                Ok(())
            })
        }

        #[napi]
        pub fn present_with_alloc<'scope>(
            &mut self, env: Env,
            #[napi(ts_arg_type = "(width: number, height: number, view: Uint32Array) => void")]
            write: Function<'scope, FnArgs<(u32, u32, Uint32Array)>, ()>,
        ) -> Result<()> {
            op_present(self, |width, height, buffer| {
                let buf_len = buffer.len();

                let layout = ok_or_reason!(Layout::array::<u32>(buf_len); "{}");
                let temp = unsafe { alloc(layout) };
                let temp_slice = unsafe { slice::from_raw_parts(temp as *const u32, buf_len) };

                let view = Uint32Array::from(unsafe { Vec::from_raw_parts(temp as *mut u32, buf_len, buf_len) });

                ok_or_reason!(write.call(FnArgs::from((width.get(), height.get(), view))));
                buffer.copy_from_slice(temp_slice);
                Ok(())
            })
        }

        #[napi]
        pub fn present_with_alloc_tsfn<'scope>(
            &mut self, env: Env,
            #[napi(ts_arg_type = "(width: number, height: number, view: Uint32Array) => void")]
            write: ThreadsafeNoCallee<FnArgs<(u32, u32, Uint32Array)>, ()>,
        ) -> Result<()> {
            op_present(self, |width, height, buffer| {
                let buf_len = buffer.len();

                let layout = ok_or_reason!(Layout::array::<u32>(buf_len); "{}");
                let temp = unsafe { alloc(layout) };
                let temp_slice = unsafe { slice::from_raw_parts(temp as *const u32, buf_len) };

                let view = Uint32Array::from(unsafe { Vec::from_raw_parts(temp as *mut u32, buf_len, buf_len) });

                let status = write.call(FnArgs::from((width.get(), height.get(), view)), ThreadsafeFunctionCallMode::NonBlocking);
                if Status::Ok != status { dbg!(status); };
                buffer.copy_from_slice(temp_slice);
                Ok(())
            })
        }
    }

    fn op_present<F>(surf: &mut SoftSurface, mut write_fn: F) -> Result<()>
    where
        F: FnMut(NonZero<u32>, NonZero<u32>, &mut softbuffer::Buffer<&winit::window::Window, &winit::window::Window>) -> Result<()>,
    {
        let context = match surf.context {
            Some(ref mut context) => context,
            None => match Context::new(surf.window) {
                Ok(context) => surf.context.insert(context),
                Err(e) => return Err(napi_reason!("Failed to create buffer context: {e}")),
            }
        };

        let surface = match surf.surface {
            Some(ref mut surface) => surface,
            None => match Surface::new(&context, surf.window) {
                Ok(surface) => surf.surface.insert(surface),
                Err(e) => return Err(napi_reason!("Failed to create buffer surface: {e}")),
            }
        };

        let size = surf.window.inner_size();

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