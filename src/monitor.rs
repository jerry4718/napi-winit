use proc::{proxy_impl, proxy_wrap};
use crate::{
    dpi::{Position, Size},
    utils::helpers::{ref_clone_into, vec_map},
};

/**[winit::monitor::VideoModeHandle]*/
#[proxy_wrap(origin_type = winit::monitor::VideoModeHandle)]
#[derive(Clone)]
pub struct VideoModeHandle;

#[proxy_impl(access_expr = self.0)]
impl VideoModeHandle {

    /// Returns the resolution of this video mode.
    #[inline]
    pub fn size(&self) -> Size;

    /// Returns the bit depth of this video mode, as in how many bits you have
    /// available per color. This is generally 24 bits or 32 bits on modern
    /// systems, depending on whether the alpha channel is counted or not.
    ///
    /// ## Platform-specific
    ///
    /// - **Wayland / Orbital:** Always returns 32.
    /// - **iOS:** Always returns 32.
    #[inline]
    pub fn bit_depth(&self) -> u16;

    /// Returns the refresh rate of this video mode in mHz.
    #[inline]
    pub fn refresh_rate_millihertz(&self) -> u32;

    /// Returns the monitor that this video mode is valid for. Each monitor has
    /// a separate set of valid video modes.
    #[inline]
    pub fn monitor(&self) -> MonitorHandle;
}

/**[winit::monitor::MonitorHandle]*/
#[proxy_wrap(origin_type = winit::monitor::MonitorHandle)]
#[derive(Clone)]
pub struct MonitorHandle;

#[proxy_impl(access_expr = self.0)]
impl MonitorHandle {
    /// Returns a human-readable name of the monitor.
    ///
    /// Returns `None` if the monitor doesn't exist anymore.
    #[inline]
    pub fn name(&self) -> Option<String>;

    /// Returns the monitor's resolution.
    #[inline]
    pub fn size(&self) -> Size;

    /// Returns the top-left corner position of the monitor relative to the larger full
    /// screen area.
    #[inline]
    pub fn position(&self) -> Position;

    /// The monitor refresh rate used by the system.
    ///
    /// Return `Some` if succeed, or `None` if failed, which usually happens when the monitor
    /// the window is on is removed.
    ///
    /// When using exclusive fullscreen, the refresh rate of the [`winit::monitor::VideoModeHandle`] that was
    /// used to enter fullscreen should be used instead.
    #[inline]
    pub fn refresh_rate_millihertz(&self) -> Option<u32>;

    /// Returns the scale factor of the underlying monitor. To map logical pixels to physical
    /// pixels and vice versa, use [`Window::scale_factor`].
    ///
    /// See the [`dpi`] module for more information.
    ///
    /// ## Platform-specific
    ///
    /// - **X11:** Can be overridden using the `WINIT_X11_SCALE_FACTOR` environment variable.
    /// - **Wayland:** May differ from [`Window::scale_factor`].
    /// - **Android:** Always returns 1.0.
    ///
    /// [`Window::scale_factor`]: crate::window::Window::scale_factor
    #[inline]
    pub fn scale_factor(&self) -> f64;


    /// Returns all fullscreen video modes supported by this monitor.
    ///
    /// ## Platform-specific
    ///
    /// - **Web:** Always returns an empty iterator
    #[inline]
    #[proxy_impl(conv_return = [ Iterator::collect::<Vec<_>>, vec_map(ref_clone_into) ])]
    pub fn video_modes(&self) -> Vec<VideoModeHandle>;
}