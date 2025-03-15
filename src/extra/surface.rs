use std::num::NonZeroU32;
use napi::bindgen_prelude::*;
use softbuffer::{Context, Surface};
use crate::window::Window;

#[napi]
struct SoftSurface<'scope> {
    pub(crate) window: &'scope winit::window::Window,
    pub(crate) context: Option<Context<&'scope winit::window::Window>>,
    pub(crate) surface: Option<Surface<&'scope winit::window::Window, &'scope winit::window::Window>>,
}

#[inline]
fn flat_result<T, E>(result: std::result::Result<T, E>) -> (Option<E>, Option<T>) {
    match result {
        Ok(res) => (None, Some(res)),
        Err(err) => (Some(err), None),
    }
}

#[inline]
fn ensure_something<T, E, F>(with: &mut Option<T>, f: F) -> std::result::Result<&mut T, E>
where
    F: FnOnce() -> std::result::Result<T, E>,
{
    if let Some(ref mut context) = with { return Ok(context); }

    let (err, res) = flat_result(f());

    if let Some(e) = err { return Err(e); }

    Ok(with.insert(res.unwrap()))
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

    #[napi]
    pub fn present(&mut self, input: Uint32Array) -> Result<()> {
        op_present(self, input)
    }
}

fn op_present(surf: &mut SoftSurface, input: Uint32Array) -> Result<()> {
    let context = match surf.context {
        Some(ref mut context) => context,
        None => match Context::new(surf.window) {
            Ok(context) => surf.context.insert(context),
            Err(e) => {
                return Err(Error::from_reason(format!("Failed to create buffer context: {}", e)))
            }
        }
    };

    let surface = match surf.surface {
        Some(ref mut surface) => surface,
        None => match Surface::new(&context, surf.window) {
            Ok(surface) => surf.surface.insert(surface),
            Err(e) => {
                return Err(Error::from_reason(format!("Failed to create buffer surface: {}", e)))
            }
        }
    };

    let size = surf.window.inner_size();

    let Some(width) = NonZeroU32::new(size.width)
    else { return Err(Error::from_reason("invalid window size [width]")) };

    let Some(height) = NonZeroU32::new(size.height)
    else { return Err(Error::from_reason("invalid window size [height]")) };

    if let Err(e) = surface.resize(width, height) {
        return Err(Error::from_reason(format!("failed to resize surface: {}", e)));
    }

    let (err, res) = flat_result(surface.buffer_mut());

    if let Some(e) = err {
        return Err(Error::from_reason(format!("failed to access buffer: {}", e)));
    }

    let Some(mut buffer) = res else { unreachable!("never handled") };

    let src_len = input.len();
    let dest_len = buffer.len();

    if src_len != dest_len {
        return Err(Error::from_reason(format!("source slice length ({src_len}) does not match destination slice length ({dest_len})")))
    }

    buffer.copy_from_slice(input.as_ref());
    buffer.present()
        .map_err(|e| Error::from_reason(format!("failed to access buffer: {}", e)))
}