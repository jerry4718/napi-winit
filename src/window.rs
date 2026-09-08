use napi::bindgen_prelude::*;

use winit::{
    monitor::{MonitorHandle as OriginMonitorHandle, VideoMode as OriginVideoModeHandle},
    monitor::Fullscreen as OriginFullscreen,
};

use proc::{proxy_enum, proxy_flags, proxy_impl, proxy_wrap};

use crate::{
    utils::helpers::{option_map, option_into, pipe, ref_clone_into, result_map, result_into, result_err_reason, vec_map, vec_map_into},
    cursor::{Cursor, CursorIcon},
    dpi::{Position, Size},
    monitor::{MonitorHandle, VideoMode},
    napi_reason,
};

#[napi]
#[derive(Clone)]
pub struct WindowAttributes {
    pub(crate) surface_size: Option<Size>,
    pub(crate) min_surface_size: Option<Size>,
    pub(crate) max_surface_size: Option<Size>,
    pub(crate) position: Option<Position>,
    pub(crate) resizable: bool,
    pub(crate) enabled_buttons: WindowButtons,
    pub(crate) title: String,
    pub(crate) maximized: bool,
    pub(crate) visible: bool,
    pub(crate) transparent: bool,
    pub(crate) blur: bool,
    pub(crate) decorations: bool,
    pub(crate) window_icon: Option<Icon>,
    pub(crate) preferred_theme: Option<Theme>,
    pub(crate) surface_resize_increments: Option<Size>,
    pub(crate) content_protected: bool,
    pub(crate) window_level: WindowLevel,
    pub(crate) active: bool,
    pub(crate) cursor: Cursor,
    // #[cfg(feature = "rwh_06")]
    // pub(crate) parent_window: Option<SendSyncRawWindowHandle>,
    pub(crate) fullscreen: Option<OriginFullscreen>,
    // Platform-specific configuration.
    // #[allow(dead_code)]
    // pub(crate) platform_specific: PlatformSpecificWindowAttributes,
}

impl Default for WindowAttributes {
    #[inline]
    fn default() -> Self {
        Self {
            surface_size: None,
            min_surface_size: None,
            max_surface_size: None,
            position: None,
            resizable: true,
            enabled_buttons: WindowButtons::all(),
            title: "winit window".to_owned(),
            maximized: false,
            fullscreen: None,
            visible: true,
            transparent: false,
            blur: false,
            decorations: true,
            window_level: WindowLevel::Normal,
            window_icon: None,
            preferred_theme: None,
            surface_resize_increments: None,
            content_protected: false,
            cursor: Cursor::default(),
            active: true,
            // #[cfg(feature = "rwh_06")]
            // parent_window: None,
            // platform_specific: Default::default(),
        }
    }
}

impl Into<winit::window::WindowAttributes> for WindowAttributes {
    fn into(self) -> winit::window::WindowAttributes {
        let attrs = winit::window::WindowAttributes::default()
            .with_resizable(self.resizable)
            .with_enabled_buttons(self.enabled_buttons.into())
            .with_title(self.title)
            .with_maximized(self.maximized)
            .with_visible(self.visible)
            .with_transparent(self.transparent)
            .with_blur(self.blur)
            .with_decorations(self.decorations)
            .with_window_level(self.window_level.into())
            .with_content_protected(self.content_protected)
            .with_cursor(self.cursor);

        let attrs = match self.surface_size {
            Some(surface_size) => attrs.with_surface_size(surface_size),
            None => attrs,
        };

        let attrs = match self.min_surface_size {
            Some(min_surface_size) => attrs.with_min_surface_size(min_surface_size),
            None => attrs,
        };

        let attrs = match self.max_surface_size {
            Some(max_surface_size) => attrs.with_max_surface_size(max_surface_size),
            None => attrs,
        };

        let attrs = match self.position {
            Some(position) => attrs.with_position(position),
            None => attrs,
        };

        let attrs = match self.fullscreen {
            Some(fullscreen) => attrs.with_fullscreen(Some(fullscreen.into())),
            None => attrs,
        };

        let attrs = match self.window_icon {
            Some(window_icon) => attrs.with_window_icon(Some(window_icon.into())),
            None => attrs,
        };

        let attrs = match self.preferred_theme {
            Some(preferred_theme) => attrs.with_theme(Some(preferred_theme.into())),
            None => attrs,
        };

        let attrs = match self.surface_resize_increments {
            Some(surface_resize_increments) => attrs.with_surface_resize_increments(surface_resize_increments),
            None => attrs,
        };

        attrs
    }
}

#[napi]
impl WindowAttributes {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    #[napi(ts_return_type = "this")]
    pub fn with_surface_size(&mut self, size: Size) -> &Self {
        self.surface_size = Some(size);
        self
    }

    #[napi(ts_return_type = "this")]
    pub fn with_min_surface_size(&mut self, min_size: Size) -> &Self {
        self.min_surface_size = Some(min_size);
        self
    }

    #[napi(ts_return_type = "this")]
    pub fn with_max_surface_size(&mut self, max_size: Size) -> &Self {
        self.max_surface_size = Some(max_size);
        self
    }

    #[napi(ts_return_type = "this")]
    pub fn with_position(&mut self, position: Position) -> &Self {
        self.position = Some(position);
        self
    }

    #[napi(ts_return_type = "this")]
    pub fn with_resizable(&mut self, resizable: bool) -> &Self {
        self.resizable = resizable;
        self
    }

    #[napi(ts_return_type = "this")]
    pub fn with_enabled_buttons(&mut self, buttons: &WindowButtons) -> &Self {
        self.enabled_buttons = buttons.clone();
        self
    }

    #[napi(ts_return_type = "this")]
    pub fn with_title(&mut self, title: String) -> &Self {
        self.title = title;
        self
    }

    #[napi(ts_return_type = "this")]
    pub fn with_fullscreen(&mut self, fullscreen: Option<Fullscreen>) -> &Self {
        self.fullscreen = fullscreen.map(Fullscreen::into_origin);
        self
    }

    #[napi(ts_return_type = "this")]
    pub fn with_maximized(&mut self, maximized: bool) -> &Self {
        self.maximized = maximized;
        self
    }

    #[napi(ts_return_type = "this")]
    pub fn with_visible(&mut self, visible: bool) -> &Self {
        self.visible = visible;
        self
    }

    #[napi(ts_return_type = "this")]
    pub fn with_transparent(&mut self, transparent: bool) -> &Self {
        self.transparent = transparent;
        self
    }

    #[napi(getter)]
    pub fn transparent(&self) -> bool {
        self.transparent
    }

    #[napi(ts_return_type = "this")]
    pub fn with_blur(&mut self, blur: bool) -> &Self {
        self.blur = blur;
        self
    }

    #[napi(ts_return_type = "this")]
    pub fn with_decorations(&mut self, decorations: bool) -> &Self {
        self.decorations = decorations;
        self
    }

    #[napi(ts_return_type = "this")]
    pub fn with_window_level(&mut self, level: WindowLevel) -> &Self {
        self.window_level = level;
        self
    }

    // #[inline]
    pub fn with_window_icon(&mut self, window_icon: Option<Icon>) -> &Self {
        self.window_icon = window_icon;
        self
    }

    #[napi(ts_return_type = "this")]
    pub fn with_theme(&mut self, theme: Option<Theme>) -> &Self {
        self.preferred_theme = theme;
        self
    }

    #[napi(ts_return_type = "this")]
    pub fn with_surface_resize_increments(&mut self, resize_increments: Size) -> &Self {
        self.surface_resize_increments = Some(resize_increments.into());
        self
    }

    #[napi(ts_return_type = "this")]
    pub fn with_content_protected(&mut self, protected: bool) -> &Self {
        self.content_protected = protected;
        self
    }

    #[napi(ts_return_type = "this")]
    pub fn with_active(&mut self, active: bool) -> &Self {
        self.active = active;
        self
    }

    #[napi(ts_return_type = "this")]
    pub fn with_cursor(&mut self, cursor: &Cursor) -> &Self {
        self.cursor = cursor.clone();
        self
    }

    // #[cfg(feature = "rwh_06")]
    // #[inline]
    // pub unsafe fn with_parent_window(
    //     mut self,
    //     parent_window: Option<rwh_06::RawWindowHandle>,
    // ) -> Self {
    //     self.parent_window = parent_window.map(SendSyncRawWindowHandle);
    //     self
    // }

    // #[cfg(feature = "rwh_06")]
    // pub fn parent_window(&self) -> Option<&rwh_06::RawWindowHandle> {
    //     self.parent_window.as_ref().map(|handle| &handle.0)
    // }
}

#[napi]
pub enum Fullscreen {
    Exclusive { monitor: Reference<MonitorHandle>, video_mode: Reference<VideoMode> },
    Borderless { monitor: Option<Reference<MonitorHandle>> },
}

impl Fullscreen {
    pub(crate) fn from_origin(origin: OriginFullscreen, env: Env) -> Result<Self> {
        match origin {
            OriginFullscreen::Exclusive(monitor, video_mode) => {
                let monitor = MonitorHandle::from(monitor).into_reference(env)?;
                VideoMode::from(video_mode).into_reference(env)
                    .map(|video_mode| Self::Exclusive { monitor, video_mode })
            }
            OriginFullscreen::Borderless(monitor) => {
                let Some(monitor) = monitor else { return Ok(Self::Borderless { monitor: None }) };
                MonitorHandle::from(monitor).into_reference(env)
                    .map(|monitor| Self::Borderless { monitor: Some(monitor) })
            }
            _ => Err(napi_reason!("unknown fullscreen variant")),
        }
    }
    pub(crate) fn into_origin(self) -> OriginFullscreen {
        match self {
            Self::Exclusive { monitor, video_mode } => {
                OriginFullscreen::Exclusive((*monitor).clone().into(), (*video_mode).clone().into())
            }
            Self::Borderless { monitor } => {
                let Some(monitor) = monitor else { return OriginFullscreen::Borderless(None) };
                OriginFullscreen::Borderless(Some((*monitor).clone().into()))
            }
        }
    }
}

/**[winit::window::WindowButtons]*/
#[proxy_flags(origin = winit::window::WindowButtons, flags = (CLOSE, MINIMIZE, MAXIMIZE))]
#[derive(Clone)]
pub struct WindowButtons;

#[proxy_enum(origin_type = winit::window::WindowLevel, string_enum)]
#[derive(Clone)]
pub enum WindowLevel { AlwaysOnBottom, Normal, AlwaysOnTop }

#[proxy_enum(origin_type = winit::window::Theme, string_enum)]
#[derive(Clone)]
pub enum Theme { Light, Dark }

#[proxy_wrap(origin_type = winit::icon::Icon)]
#[derive(Clone)]
pub struct Icon;

#[napi]
impl Icon {
    #[napi(factory, ts_return_type = "Icon")]
    pub fn from_rgba(env: Env, rgba: Uint8Array, width: u32, height: u32) -> Result<Self> {
        winit::icon::RgbaIcon::new(rgba.to_vec(), width, height)
            .map(winit::icon::Icon::from)
            .map(Self::from)
            .map_err(|e| napi_reason!("{e}"))
    }
}

/**[winit::window::WindowId]*/
#[proxy_wrap(origin_type = winit::window::WindowId)]
pub struct WindowId;

#[napi]
impl WindowId {
    pub fn raw_u64(&self) -> u64 {
        self.0.into_raw() as u64
    }

    #[napi]
    pub fn raw(&self) -> BigInt {
        BigInt::from(self.raw_u64())
    }
    #[napi]
    pub fn raw_string(&self) -> String {
        self.raw_u64().to_string()
    }
}

/**[winit::window::ActivationToken]*/
#[proxy_wrap(origin_type = winit::window::ActivationToken)]
pub struct ActivationToken;

/**[winit::window::Window]*/
pub(crate) type DynWindow = Box<dyn winit::window::Window>;

#[proxy_wrap(origin_type = DynWindow, field_name = inner)]
pub struct Window;

#[napi]
impl Window {
    #[napi]
    pub fn default_attributes() -> WindowAttributes {
        WindowAttributes::default()
    }
}

#[proxy_impl(access_expr = self.inner)]
impl Window {
    fn id(&self) -> WindowId;
    fn scale_factor(&self) -> f64;
    fn request_redraw(&self);
    fn pre_present_notify(&self);
    fn reset_dead_keys(&self);
}

#[proxy_impl(access_expr = self.inner)]
impl Window {
    #[proxy_impl(conv_return = Into::into)]
    fn surface_position(&self) -> Position;

    #[proxy_impl(conv_return = [ result_map(Into::into), result_err_reason ])]
    fn outer_position(&self) -> Result<Position>;

    fn set_outer_position(&self, position: Position);

    #[proxy_impl(conv_return = Into::into)]
    fn surface_size(&self) -> Size;

    #[proxy_impl(conv_return = option_map(Into::into))]
    fn request_surface_size(&self, size: Size) -> Option<Size>;

    fn outer_size(&self) -> Size;

    fn set_min_surface_size(&self, #[proxy_impl(conv_arg = option_into)] min_size: Option<Size>);

    fn set_max_surface_size(&self, #[proxy_impl(conv_arg = option_into)] max_size: Option<Size>);

    #[proxy_impl(conv_return = option_into)]
    fn surface_resize_increments(&self) -> Option<Size>;

    fn set_surface_resize_increments(&self, #[proxy_impl(conv_arg = option_into)] increments: Option<Size>);
}

#[proxy_impl(access_expr = self.inner)]
impl Window {
    fn set_title(&self, #[proxy_impl(conv_arg = title.as_str())] title: String);

    fn set_transparent(&self, transparent: bool);

    fn set_blur(&self, blur: bool);

    fn set_visible(&self, visible: bool);

    fn is_visible(&self) -> Option<bool>;

    fn set_resizable(&self, resizable: bool);

    fn is_resizable(&self) -> bool;

    fn set_enabled_buttons(&self, #[proxy_impl(conv_arg = buttons.clone().into())] buttons: &WindowButtons);

    fn enabled_buttons(&self) -> WindowButtons;

    fn set_minimized(&self, minimized: bool);

    fn is_minimized(&self) -> Option<bool>;

    fn set_maximized(&self, maximized: bool);
    fn is_maximized(&self) -> bool;

    // fn set_fullscreen(&self, #[proxy_impl(conv_arg = option_into)] fullscreen: Option<Fullscreen>);

    // #[proxy_impl(conv_return = option_into)]
    // fn fullscreen(&self) -> Option<Fullscreen>;

    fn set_decorations(&self, decorations: bool);
    fn is_decorated(&self) -> bool;

    fn set_window_level(&self, level: WindowLevel);

    fn set_window_icon(&self, #[proxy_impl(conv_arg = [ window_icon.map(|icon| icon.clone().into()) ])] window_icon: Option<&Icon>);

    fn set_ime_cursor_area(&self, position: Position, size: Size);

    fn set_ime_allowed(&self, allowed: bool);

    fn set_ime_purpose(&self, #[proxy_impl(conv_arg = purpose.into())] purpose: ImePurpose);

    fn focus_window(&self);

    fn has_focus(&self) -> bool;

    fn request_user_attention(&self, #[proxy_impl(conv_arg = option_into)] request_type: Option<UserAttentionType>);

    fn set_theme(&self, #[proxy_impl(conv_arg = option_into)] theme: Option<Theme>);

    #[proxy_impl(conv_return = option_into)]
    fn theme(&self) -> Option<Theme>;

    fn set_content_protected(&self, protected: bool);

    fn title(&self) -> String;
}

#[napi]
impl Window {
    #[napi]
    pub fn set_fullscreen(&self, fullscreen: Option<Fullscreen>) {
        self.inner.set_fullscreen(fullscreen.map(Fullscreen::into_origin));
    }

    #[napi]
    pub fn fullscreen(&self, env: Env) -> Result<Option<Fullscreen>> {
        let Some(fullscreen) = self.inner.fullscreen() else { return Ok(None) };
        Fullscreen::from_origin(fullscreen, env).map(|inner| Some(inner))
    }
}

#[proxy_impl(access_expr = self.inner)]
impl Window {
    fn set_cursor(&self, #[proxy_impl(conv_arg = [ Clone::clone, Into::into ])] cursor: &Cursor);

    // fn set_cursor_icon(&self, icon: CursorIcon);

    #[proxy_impl(conv_return = [ result_err_reason ])]
    fn set_cursor_position(&self, position: Position) -> Result<()>;

    #[proxy_impl(conv_return = [ result_err_reason ])]
    fn set_cursor_grab(&self, #[proxy_impl(conv_arg = mode.into())] mode: CursorGrabMode) -> Result<()>;

    fn set_cursor_visible(&self, visible: bool);

    #[proxy_impl(conv_return = [ result_err_reason ])]
    fn drag_window(&self) -> Result<()>;

    #[proxy_impl(conv_return = [ result_err_reason ])]
    fn drag_resize_window(&self, #[proxy_impl(conv_arg = direction.into())] direction: ResizeDirection) -> Result<()>;

    fn show_window_menu(&self, position: Position);

    #[proxy_impl(conv_return = [ result_err_reason ])]
    fn set_cursor_hittest(&self, hittest: bool) -> Result<()>;
}

#[proxy_impl(access_expr = self.inner)]
impl Window {
    #[proxy_impl(conv_return = option_into)]
    fn current_monitor(&self) -> Option<MonitorHandle>;

    #[proxy_impl(conv_return = [ Iterator::collect::<Vec<_>>, vec_map(ref_clone_into) ])]
    fn available_monitors(&self) -> Vec<MonitorHandle>;

    #[proxy_impl(conv_return = option_into)]
    fn primary_monitor(&self) -> Option<MonitorHandle>;
}

#[proxy_enum(origin_type = winit::window::ImePurpose, string_enum, non_exhaustive)]
pub enum ImePurpose { Normal, Password, Terminal }

#[proxy_enum(origin_type = winit::window::UserAttentionType, string_enum)]
pub enum UserAttentionType { Critical, Informational }

#[proxy_enum(origin_type = winit::window::CursorGrabMode, string_enum)]
pub enum CursorGrabMode { None, Confined, Locked }

#[proxy_enum(origin_type = winit::window::ResizeDirection, string_enum)]
pub enum ResizeDirection { East, North, NorthEast, NorthWest, South, SouthEast, SouthWest, West }