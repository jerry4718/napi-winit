use napi::bindgen_prelude::*;

use winit::window::{
    Fullscreen as OriginFullscreen,
};

use proc::{proxy_enum, proxy_flags, proxy_struct};

use crate::{
    cursor::{Cursor, CursorIcon},
    dpi::{Position, Size},
    monitor::MonitorHandle,
    napi_reason,
};

#[napi]
#[derive(Clone)]
pub struct WindowAttributes {
    pub(crate) inner_size: Option<Size>,
    pub(crate) min_inner_size: Option<Size>,
    pub(crate) max_inner_size: Option<Size>,
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
    pub(crate) resize_increments: Option<Size>,
    pub(crate) content_protected: bool,
    pub(crate) window_level: WindowLevel,
    pub(crate) active: bool,
    pub(crate) cursor: Cursor,
    // #[cfg(feature = "rwh_06")]
    // pub(crate) parent_window: Option<SendSyncRawWindowHandle>,
    pub(crate) fullscreen: Option<Fullscreen>,
    // Platform-specific configuration.
    // #[allow(dead_code)]
    // pub(crate) platform_specific: PlatformSpecificWindowAttributes,
}

impl Default for WindowAttributes {
    #[inline]
    fn default() -> Self {
        Self {
            inner_size: None,
            min_inner_size: None,
            max_inner_size: None,
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
            resize_increments: None,
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

        let attrs = match self.inner_size {
            Some(inner_size) => attrs.with_inner_size(inner_size),
            None => attrs,
        };

        let attrs = match self.min_inner_size {
            Some(min_inner_size) => attrs.with_min_inner_size(min_inner_size),
            None => attrs,
        };

        let attrs = match self.max_inner_size {
            Some(max_inner_size) => attrs.with_max_inner_size(max_inner_size),
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

        let attrs = match self.resize_increments {
            Some(resize_increments) => attrs.with_resize_increments(resize_increments),
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
    pub fn with_inner_size(&mut self, size: Size) -> &Self {
        self.inner_size = Some(size);
        self
    }

    #[napi(ts_return_type = "this")]
    pub fn with_min_inner_size(&mut self, min_size: Size) -> &Self {
        self.min_inner_size = Some(min_size);
        self
    }

    #[napi(ts_return_type = "this")]
    pub fn with_max_inner_size(&mut self, max_size: Size) -> &Self {
        self.max_inner_size = Some(max_size);
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
        self.fullscreen = fullscreen;
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
    pub fn with_resize_increments(&mut self, resize_increments: Size) -> &Self {
        self.resize_increments = Some(resize_increments.into());
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
#[derive(Clone)]
pub enum Fullscreen {
    Exclusive,
    Borderless,
}

impl Into<OriginFullscreen> for Fullscreen {
    fn into(self) -> OriginFullscreen {
        match self {
            Self::Exclusive => unimplemented!("Fullscreen::Exclusive has not implemented"),
            Self::Borderless => OriginFullscreen::Borderless(None),
        }
    }
}

impl From<OriginFullscreen> for Fullscreen {
    fn from(value: OriginFullscreen) -> Self {
        match value {
            OriginFullscreen::Exclusive(_) => Self::Exclusive,
            OriginFullscreen::Borderless(_) => Self::Borderless,
        }
    }
}

#[proxy_flags(origin = winit::window::WindowButtons, flags = (CLOSE, MINIMIZE, MAXIMIZE))]
#[derive(Clone)]
pub struct WindowButtons;

#[proxy_enum(origin_enum = winit::window::WindowLevel, string_enum)]
#[derive(Clone)]
pub enum WindowLevel { AlwaysOnBottom, Normal, AlwaysOnTop }

#[proxy_enum(origin_enum = winit::window::Theme, string_enum)]
#[derive(Clone)]
pub enum Theme { Light, Dark }

#[proxy_struct(origin_type = winit::window::Icon)]
#[derive(Clone)]
pub struct Icon;

#[napi]
impl Icon {
    #[napi(factory, ts_return_type = "Icon")]
    pub fn from_rgba(env: Env, rgba: Uint8Array, width: u32, height: u32) -> Result<Self> {
        winit::window::Icon::from_rgba(rgba.to_vec(), width, height)
            .map(Self::from)
            .map_err(|e| napi_reason!("{e}"))
    }
}

#[proxy_struct(origin_type = winit::window::WindowId)]
pub struct WindowId;

#[proxy_struct(origin_type = winit::window::ActivationToken)]
pub struct ActivationToken;

#[proxy_struct(origin_type = winit::window::Window, field_name = inner)]
pub struct Window;

#[napi]
impl Window {
    #[napi]
    pub fn default_attributes() -> WindowAttributes {
        WindowAttributes::default()
    }

    #[napi]
    pub fn id(&self) -> WindowId {
        self.inner.id().into()
    }

    #[napi]
    pub fn scale_factor(&self) -> f64 {
        self.inner.scale_factor()
    }

    #[napi]
    pub fn request_redraw(&self) {
        self.inner.request_redraw();
    }

    #[napi]
    pub fn pre_present_notify(&self) {
        self.inner.pre_present_notify();
    }

    #[napi]
    pub fn reset_dead_keys(&self) {
        self.inner.reset_dead_keys();
    }
}

#[napi]
impl Window {
    #[napi]
    pub fn inner_position(&self) -> Result<Position> {
        self.inner.inner_position()
            .map(Position::from)
            .map_err(|e| napi_reason!("{e}"))
    }

    #[napi]
    pub fn outer_position(&self) -> Result<Position> {
        self.inner.outer_position()
            .map(Position::from)
            .map_err(|e| napi_reason!("{e}"))
    }

    #[napi]
    pub fn set_outer_position(&self, position: Position) {
        self.inner.set_outer_position(position);
    }

    #[napi]
    pub fn inner_size(&self) -> Size {
        Size::from(self.inner.inner_size())
    }

    #[napi]
    pub fn request_inner_size(&self, size: Size) -> Option<Size> {
        self.inner.request_inner_size(size).map(Size::from)
    }

    #[napi]
    pub fn outer_size(&self) -> Size {
        self.inner.outer_size().into()
    }

    #[napi]
    pub fn set_min_inner_size(&self, min_size: Option<Size>) {
        self.inner.set_min_inner_size(min_size)
    }

    #[napi]
    pub fn set_max_inner_size(&self, min_size: Option<Size>) {
        self.inner.set_max_inner_size(min_size)
    }

    #[napi]
    pub fn resize_increments(&self) -> Option<Size> {
        self.inner.resize_increments().map(Size::from)
    }

    #[napi]
    pub fn set_resize_increments(&self, increments: Option<Size>) {
        self.inner.set_resize_increments(increments)
    }
}

#[napi]
impl Window {
    #[napi]
    pub fn set_title(&self, title: String) {
        self.inner.set_title(title.as_str())
    }
    #[napi]
    pub fn set_transparent(&self, transparent: bool) {
        self.inner.set_transparent(transparent)
    }
    #[napi]
    pub fn set_blur(&self, blur: bool) {
        self.inner.set_blur(blur)
    }
    #[napi]
    pub fn set_visible(&self, visible: bool) {
        self.inner.set_visible(visible)
    }
    #[napi]
    pub fn is_visible(&self) -> Option<bool> {
        self.inner.is_visible()
    }
    #[napi]
    pub fn set_resizable(&self, resizable: bool) {
        self.inner.set_resizable(resizable)
    }
    #[napi]
    pub fn is_resizable(&self) -> bool {
        self.inner.is_resizable()
    }
    #[napi]
    pub fn set_enabled_buttons(&self, buttons: &WindowButtons) {
        self.inner.set_enabled_buttons(buttons.clone().into())
    }
    #[napi]
    pub fn enabled_buttons(&self) -> WindowButtons {
        self.inner.enabled_buttons().into()
    }
    #[napi]
    pub fn set_minimized(&self, minimized: bool) {
        self.inner.set_minimized(minimized)
    }
    #[napi]
    pub fn is_minimized(&self) -> Option<bool> {
        self.inner.is_minimized()
    }
    #[napi]
    pub fn set_maximized(&self, maximized: bool) {
        self.inner.set_maximized(maximized)
    }
    #[napi]
    pub fn is_maximized(&self) -> bool {
        self.inner.is_maximized()
    }
    #[napi]
    pub fn set_fullscreen(&self, fullscreen: Option<Fullscreen>) {
        self.inner.set_fullscreen(fullscreen.map(Into::into));
    }
    #[napi]
    pub fn fullscreen(&self) -> Option<Fullscreen> {
        self.inner.fullscreen().map(Fullscreen::from)
    }
    #[napi]
    pub fn set_decorations(&self, decorations: bool) {
        self.inner.set_decorations(decorations)
    }
    #[napi]
    pub fn is_decorated(&self) -> bool {
        self.inner.is_decorated()
    }
    #[napi]
    pub fn set_window_level(&self, level: WindowLevel) {
        self.inner.set_window_level(level.into())
    }
    #[napi]
    pub fn set_window_icon(&self, window_icon: Option<&Icon>) {
        self.inner.set_window_icon(window_icon.map(|icon| icon.clone().into()))
    }
    #[napi]
    pub fn set_ime_cursor_area(&self, position: Position, size: Size) {
        self.inner.set_ime_cursor_area(position, size)
    }
    #[napi]
    pub fn set_ime_allowed(&self, allowed: bool) {
        self.inner.set_ime_allowed(allowed)
    }
    #[napi]
    pub fn set_ime_purpose(&self, purpose: ImePurpose) {
        self.inner.set_ime_purpose(purpose.into())
    }
    #[napi]
    pub fn focus_window(&self) {
        self.inner.focus_window()
    }
    #[napi]
    pub fn has_focus(&self) -> bool {
        self.inner.has_focus()
    }
    #[napi]
    pub fn request_user_attention(&self, request_type: Option<UserAttentionType>) {
        self.inner.request_user_attention(request_type.map(Into::into))
    }
    #[napi]
    pub fn set_theme(&self, theme: Option<Theme>) {
        self.inner.set_theme(theme.map(Into::into))
    }
    #[napi]
    pub fn theme(&self) -> Option<Theme> {
        self.inner.theme().map(Into::into)
    }
    #[napi]
    pub fn set_content_protected(&self, protected: bool) {
        self.inner.set_content_protected(protected)
    }
    #[napi]
    pub fn title(&self) -> String {
        self.inner.title()
    }
}

#[napi]
impl Window {
    #[napi]
    pub fn set_cursor(&self, cursor: &Cursor) {
        self.inner.set_cursor(cursor.clone())
    }
    // #[napi]
    // pub fn set_cursor_icon(&self, icon: CursorIcon) {
    //     self.inner.set_cursor_icon(icon.into())
    // }
    #[napi]
    pub fn set_cursor_position(&self, position: Position) -> Result<()> {
        self.inner.set_cursor_position(position)
            .map_err(|e| napi_reason!("{e}"))
    }
    #[napi]
    pub fn set_cursor_grab(&self, mode: CursorGrabMode) -> Result<()> {
        self.inner.set_cursor_grab(mode.into())
            .map_err(|e| napi_reason!("{e}"))
    }
    #[napi]
    pub fn set_cursor_visible(&self, visible: bool) {
        self.inner.set_cursor_visible(visible)
    }
    #[napi]
    pub fn drag_window(&self) -> Result<()> {
        self.inner.drag_window()
            .map_err(|e| napi_reason!("{e}"))
    }
    #[napi]
    pub fn drag_resize_window(&self, direction: ResizeDirection) -> Result<()> {
        self.inner.drag_resize_window(direction.into())
            .map_err(|e| napi_reason!("{e}"))
    }
    #[napi]
    pub fn show_window_menu(&self, position: Position) {
        self.inner.show_window_menu(position)
    }
    #[napi]
    pub fn set_cursor_hittest(&self, hittest: bool) -> Result<()> {
        self.inner.set_cursor_hittest(hittest)
            .map_err(|e| napi_reason!("{e}"))
    }
}

#[napi]
impl Window {
    #[napi]
    pub fn current_monitor(&self) -> Option<MonitorHandle> {
        self.inner.current_monitor().map(|m| m.into())
    }
    #[napi]
    pub fn available_monitors(&self) -> Vec<MonitorHandle> {
        self.inner.available_monitors().map(|m| m.into()).collect()
    }
    #[napi]
    pub fn primary_monitor(&self) -> Option<MonitorHandle> {
        self.inner.primary_monitor().map(|m| m.into())
    }
}

#[proxy_enum(origin_enum = winit::window::ImePurpose, string_enum, non_exhaustive)]
pub enum ImePurpose { Normal, Password, Terminal }

#[proxy_enum(origin_enum = winit::window::UserAttentionType, string_enum)]
pub enum UserAttentionType { Critical, Informational }

#[proxy_enum(origin_enum = winit::window::CursorGrabMode, string_enum)]
pub enum CursorGrabMode { None, Confined, Locked }

#[proxy_enum(origin_enum = winit::window::ResizeDirection, string_enum)]
pub enum ResizeDirection { East, North, NorthEast, NorthWest, South, SouthEast, SouthWest, West }