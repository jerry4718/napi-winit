#[napi(js_name = "Extra")]
pub mod namespace {
    use napi::bindgen_prelude::*;

    #[napi(
        discriminant = "system",
        discriminant_case = "lowercase",
        object_from_js = false
    )]
    pub enum SurfaceOptions {
        Win32 {
            window_handle: usize,
            // Some: rwh delivered the instance handle; None: rwh reports it
            // absent. Surface consumers reject the window without it.
            display_handle: Option<usize>,
        },
        Cocoa {
            window_handle: usize,
        },
        X11 {
            window_handle: usize,
            // Some: rwh delivered the Display pointer; None: rwh reports it
            // absent. Surface consumers reject the window without it.
            display_handle: Option<usize>,
        },
        Wayland {
            window_handle: usize,
            display_handle: usize,
        },
    }
}

#[napi(js_name = "Extra")]
mod rwh_impl {
    use super::namespace::*;
    use crate::{napi_reason, window::Window};
    use napi::bindgen_prelude::*;
    use rwh::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};

    #[napi]
    pub fn get_rwh_options(window: &Window) -> Result<SurfaceOptions> {
        let window_handle = match window.inner.window_handle() {
            Err(e) => return Err(napi_reason!("{e}")),
            Ok(handle) => handle.as_raw(),
        };

        let display_handle = match window.inner.display_handle() {
            Err(e) => return Err(napi_reason!("{e}")),
            Ok(handle) => handle.as_raw(),
        };

        match (window_handle, display_handle) {
            #[cfg(target_os = "windows")]
            (RawWindowHandle::Win32(window), _) => Ok(SurfaceOptions::Win32 {
                window_handle: window.hwnd.unsigned_abs().get(),
                // Windows has no display object; rwh delivers the instance
                // handle as the display-side value.
                display_handle: window
                    .hinstance
                    .map(|hinstance| hinstance.unsigned_abs().get()),
            }),
            #[cfg(target_os = "macos")]
            (RawWindowHandle::AppKit(window), _) => Ok(SurfaceOptions::Cocoa {
                // NSView pointer; consumers resolve the CAMetalLayer from it.
                window_handle: window.ns_view.as_ptr() as usize,
            }),
            #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
            (RawWindowHandle::Xlib(window), RawDisplayHandle::Xlib(display)) => {
                // XWindow is c_ulong: pointer-width, so the cast to usize is
                // lossless on every supported target.
                Ok(SurfaceOptions::X11 {
                    window_handle: window.window as usize,
                    display_handle: display.display.map(|display| display.as_ptr() as usize),
                })
            }
            #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
            (RawWindowHandle::Wayland(window), RawDisplayHandle::Wayland(display)) => {
                Ok(SurfaceOptions::Wayland {
                    window_handle: window.surface.as_ptr() as usize,
                    display_handle: display.display.as_ptr() as usize,
                })
            }
            _ => Err(napi_reason!("unimplemented for this platform")),
        }
    }
}
