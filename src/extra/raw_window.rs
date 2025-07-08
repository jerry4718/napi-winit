#[napi(js_name = "Extra")]
pub mod namespace {
    use napi::bindgen_prelude::*;

    #[napi(string_enum = "lowercase")]
    pub enum SurfaceSystem {
        Win32,
        Cocoa,
        X11,
        Wayland,
    }

    #[napi(object)]
    pub struct SurfaceOptions {
        pub system: SurfaceSystem,
        pub window_handle: BigInt,
        pub display_handle: BigInt,
    }
}

#[napi(js_name = "Extra")]
pub mod rwh_05_impl {
    use super::namespace::*;
    use napi::bindgen_prelude::*;
    use rwh_05::{HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle};
    use crate::{window::Window, napi_reason};

    #[napi]
    pub fn get_rwh_05_options(window: &Window) -> Result<SurfaceOptions> {
        let raw_window_handle = window.inner.raw_window_handle();
        let raw_display_handle = window.inner.raw_display_handle();

        match (raw_window_handle, raw_display_handle) {
            #[cfg(target_os = "windows")]
            (RawWindowHandle::Win32(window), _) => {
                Ok(SurfaceOptions {
                    system: SurfaceSystem::Win32,
                    window_handle: BigInt::from(window.hwnd as u64),
                    display_handle: BigInt::from(window.hinstance as u64),
                })
            }
            #[cfg(target_os = "macos")]
            (RawWindowHandle::AppKit(window), _) => {
                Ok(SurfaceOptions {
                    system: SurfaceSystem::Cocoa,
                    window_handle: BigInt::from(window.ns_window as u64),
                    display_handle: BigInt::from(window.ns_view as u64),
                })
            }
            #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
            (
                RawWindowHandle::Xlib(window),
                RawDisplayHandle::Xlib(display)
            ) => {
                Ok(SurfaceOptions {
                    system: SurfaceSystem::X11,
                    window_handle: BigInt::from(window.window as u64),
                    display_handle: BigInt::from(display.display as u64),
                })
            }
            #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
            (
                RawWindowHandle::Wayland(window),
                RawDisplayHandle::Wayland(display)
            ) => {
                Ok(SurfaceOptions {
                    system: SurfaceSystem::Wayland,
                    window_handle: BigInt::from(window.surface as u64),
                    display_handle: BigInt::from(display.display as u64),
                })
            }
            _ => Err(napi_reason!("unimplemented for this platform")),
        }
    }
}

/* just draft */
#[napi(js_name = "Extra")]
mod rwh_06_impl {
    use super::namespace::*;
    use napi::bindgen_prelude::*;
    use rwh_06::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};
    use crate::{ window::Window, napi_reason };

    // #[napi]
    pub fn get_rwh_06_options(window: &Window) -> Result<SurfaceOptions> {
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
            (RawWindowHandle::Win32(window), _) => {
                Ok(SurfaceOptions {
                    system: SurfaceSystem::Win32,
                    window_handle: BigInt::from(window.hwnd.unsigned_abs().get() as u64),
                    display_handle: BigInt::from(window.hinstance.unwrap().unsigned_abs().get() as u64),
                })
            }
            #[cfg(target_os = "macos")]
            (RawWindowHandle::AppKit(window), _) => {
                use objc2::rc::Retained;
                use objc2_app_kit::{NSView, NSWindow};

                let ns_view = window.ns_view.as_ptr();

                let ns_view: Retained<NSView> = unsafe { Retained::retain(ns_view.cast()) }.unwrap();
                let ns_window: Retained<NSWindow> = ns_view.window().expect("view was not installed in a window");

                Ok(SurfaceOptions {
                    system: SurfaceSystem::Cocoa,
                    window_handle: BigInt::from(Retained::as_ptr(&ns_window) as u64),
                    display_handle: BigInt::from(Retained::as_ptr(&ns_view) as u64),
                })
            }
            #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
            (
                RawWindowHandle::Xlib(window),
                RawDisplayHandle::Xlib(display)
            ) => {
                Ok(SurfaceOptions {
                    system: SurfaceSystem::X11,
                    window_handle: BigInt::from(window.window as u64),
                    display_handle: BigInt::from(display.display.unwrap().as_ptr() as u64),
                })
            }
            #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
            (
                RawWindowHandle::Wayland(window),
                RawDisplayHandle::Wayland(display)
            ) => {
                Ok(SurfaceOptions {
                    system: SurfaceSystem::Wayland,
                    window_handle: BigInt::from(window.surface.as_ptr() as u64),
                    display_handle: BigInt::from(display.display.as_ptr() as u64),
                })
            }
            _ => Err(napi_reason!("unimplemented for this platform")),
        }
    }
}

