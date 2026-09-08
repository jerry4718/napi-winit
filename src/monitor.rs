use proc::{proxy_impl, proxy_wrap};
use crate::{
    dpi::{Position, Size},
    utils::helpers::{ref_clone_into, vec_map, option_map, to_option_string, option_into},
};

/**[winit::monitor::VideoMode]*/
#[proxy_wrap(origin_type = winit::monitor::VideoMode)]
#[derive(Clone)]
pub struct VideoMode;

#[proxy_impl(access_expr = self.0)]
impl VideoMode {

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
    #[proxy_impl(conv_return = option_map(|bit_depth: std::num::NonZeroU16| bit_depth.get()))]
    pub fn bit_depth(&self) -> Option<u16>;

    /// Returns the refresh rate of this video mode in mHz.
    #[inline]
    #[proxy_impl(conv_return = option_map(|refresh_rate: std::num::NonZeroU32| refresh_rate.get()))]
    pub fn refresh_rate_millihertz(&self) -> Option<u32>;
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
    #[proxy_impl(conv_return = to_option_string)]
    pub fn name(&self) -> Option<String>;

    /// The monitor refresh rate used by the system.
    ///
    /// Return `Some` if succeed, or `None` if failed, which usually happens when the monitor
    /// the window is on is removed.
    ///
    /// When using exclusive fullscreen, the refresh rate of the [`winit::monitor::VideoMode`] that was
    /// used to enter fullscreen should be used instead.
    #[inline]
    #[proxy_impl(conv_return = option_into)]
    pub fn position(&self) -> Option<Position>;

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
    pub fn video_modes(&self) -> Vec<VideoMode>;
}

#[proxy_impl(access_expr = self.0)]
impl MonitorHandle {
    /// Returns an identifier that persistently changes across a system reboot.
    #[inline]
    pub fn id(&self) -> u128;

    /// Returns a platform-native identifier for the monitor.
    #[inline]
    pub fn native_id(&self) -> u64;

    /// Returns the current video mode of this monitor.
    ///
    /// This is useful to acquire the monitor's size and refresh rate.
    #[inline]
    #[proxy_impl(conv_return = option_into)]
    pub fn current_video_mode(&self) -> Option<VideoMode>;
}