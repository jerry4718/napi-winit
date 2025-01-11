use crate::dpi::JsPosition;
use crate::dpi::JsSize;
use crate::event_loop::JsEventLoop;
use std::cell::RefCell;
use std::fmt;
use std::ops::{BitAnd, DerefMut};
use std::ptr::NonNull;
use std::rc::Rc;

use winit::{
    dpi::{Position, Size},
    event_loop::EventLoop,
    window::{Cursor, CursorIcon, Fullscreen, Icon, Theme, Window, WindowAttributes, WindowButtons, WindowLevel}
};

use napi::bindgen_prelude::*;
use napi::{JsObject, NapiRaw, NapiValue};
use napi::sys::{napi_env, napi_value};
use crate::cursor::JsCursor;

#[napi(js_name = "WindowAttributes")]
pub struct JsWindowAttributes {
    pub(crate) inner_size: Option<JsSize>,
    pub(crate) min_inner_size: Option<JsSize>,
    pub(crate) max_inner_size: Option<JsSize>,
    pub(crate) position: Option<JsPosition>,
    pub(crate) resizable: bool,
    pub(crate) enabled_buttons: JsWindowButtons,
    pub(crate) title: String,
    pub(crate) maximized: bool,
    pub(crate) visible: bool,
    pub(crate) transparent: bool,
    pub(crate) blur: bool,
    pub(crate) decorations: bool,
    pub(crate) window_icon: Option<JsIcon>,
    pub(crate) preferred_theme: Option<JsTheme>,
    pub(crate) resize_increments: Option<JsSize>,
    pub(crate) content_protected: bool,
    pub(crate) window_level: JsWindowLevel,
    pub(crate) active: bool,
    pub(crate) cursor: JsCursor,
    // #[cfg(feature = "rwh_06")]
    // pub(crate) parent_window: Option<SendSyncRawWindowHandle>,
    pub(crate) fullscreen: Option<JsFullscreen>,
    // Platform-specific configuration.
    // #[allow(dead_code)]
    // pub(crate) platform_specific: PlatformSpecificWindowAttributes,
}

impl Default for JsWindowAttributes {
    #[inline]
    fn default() -> Self {
        Self {
            inner_size: None,
            min_inner_size: None,
            max_inner_size: None,
            position: None,
            resizable: true,
            enabled_buttons: JsWindowButtons::all(),
            title: "winit window".to_owned(),
            maximized: false,
            fullscreen: None,
            visible: true,
            transparent: false,
            blur: false,
            decorations: true,
            window_level: Default::default(),
            window_icon: None,
            preferred_theme: None,
            resize_increments: None,
            content_protected: false,
            cursor: JsCursor::default(),
            active: true,
            // #[cfg(feature = "rwh_06")]
            // parent_window: None,
            // platform_specific: Default::default(),
        }
    }
}

impl Into<WindowAttributes> for JsWindowAttributes {
    fn into(self) -> WindowAttributes {
        let attrs = WindowAttributes::default()
            .with_resizable(self.resizable)
            .with_enabled_buttons(self.enabled_buttons.into())
            .with_title(self.title)
            .with_maximized(self.maximized)
            .with_visible(self.visible)
            .with_transparent(self.transparent)
            .with_blur(self.blur)
            .with_decorations(self.decorations)
            .with_window_level(self.window_level.into())
            .with_content_protected(self.content_protected);

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

        let cursor = self.cursor;

        attrs
    }
}

#[napi]
impl JsWindowAttributes {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    #[napi(ts_return_type = "this")]
    pub fn with_inner_size(&mut self, this: This<JsObject>, size: JsSize) -> This<JsObject> {
        self.inner_size = Some(size);
        this
    }

    #[napi(ts_return_type = "this")]
    pub fn with_min_inner_size(&mut self, this: This<JsObject>, min_size: JsSize) -> This<JsObject> {
        self.min_inner_size = Some(min_size);
        this
    }

    #[napi(ts_return_type = "this")]
    pub fn with_max_inner_size(&mut self, this: This<JsObject>, max_size: JsSize) -> This<JsObject> {
        self.max_inner_size = Some(max_size);
        this
    }

    #[napi(ts_return_type = "this")]
    pub fn with_position(&mut self, this: This<JsObject>, position: JsPosition) -> This<JsObject> {
        self.position = Some(position);
        this
    }

    #[napi(ts_return_type="this")]
    pub fn with_resizable(&mut self, this: This<JsObject>, resizable: bool) -> This<JsObject> {
        self.resizable = resizable;
        this
    }

    #[napi(ts_return_type="this")]
    pub fn with_enabled_buttons(
        &mut self, this: This<JsObject>,
        #[napi(ts_arg_type = "WindowButtons")] buttons: &JsWindowButtons
    ) -> This<JsObject> {
        self.enabled_buttons = *buttons;
        this
    }

    #[napi(ts_return_type="this")]
    pub fn with_title(&mut self, this: This<JsObject>, title: String) -> This<JsObject> {
        self.title = title;
        this
    }

    #[napi(ts_return_type="this")]
    pub fn with_fullscreen(&mut self, this: This<JsObject>, #[napi(ts_arg_type = "Fullscreen | null")] fullscreen: Option<JsFullscreen>) -> This<JsObject> {
        self.fullscreen = fullscreen;
        this
    }

    #[napi(ts_return_type="this")]
    pub fn with_maximized(&mut self, this: This<JsObject>, maximized: bool) -> This<JsObject> {
        self.maximized = maximized;
        this
    }

    #[napi(ts_return_type="this")]
    pub fn with_visible(&mut self, this: This<JsObject>, visible: bool) -> This<JsObject> {
        self.visible = visible;
        this
    }

    #[napi(ts_return_type="this")]
    pub fn with_transparent(&mut self, this: This<JsObject>, transparent: bool) -> This<JsObject> {
        self.transparent = transparent;
        this
    }

    #[napi(getter)]
    pub fn transparent(&self) -> bool {
        self.transparent
    }

    #[napi(ts_return_type="this")]
    pub fn with_blur(&mut self, this: This<JsObject>, blur: bool) -> This<JsObject> {
        self.blur = blur;
        this
    }

    #[napi(ts_return_type="this")]
    pub fn with_decorations(&mut self, this: This<JsObject>, decorations: bool) -> This<JsObject> {
        self.decorations = decorations;
        this
    }

    #[napi(ts_return_type="this")]
    pub fn with_window_level(&mut self, this: This<JsObject>, #[napi(ts_arg_type = "WindowLevel")] level: JsWindowLevel) -> This<JsObject> {
        self.window_level = level;
        this
    }

    // #[inline]
    pub fn with_window_icon(&mut self, this: This<JsObject>, window_icon: Option<JsIcon>) -> This<JsObject> {
        self.window_icon = window_icon;
        this
    }

    #[napi(ts_return_type="this")]
    pub fn with_theme(&mut self, this: This<JsObject>, #[napi(ts_arg_type = "Theme")] theme: Option<JsTheme>) -> This<JsObject> {
        self.preferred_theme = theme;
        this
    }

    #[napi(ts_return_type="this")]
    pub fn with_resize_increments(&mut self, this: This<JsObject>, resize_increments: JsSize) -> This<JsObject> {
        self.resize_increments = Some(resize_increments.into());
        this
    }

    #[napi(ts_return_type="this")]
    pub fn with_content_protected(&mut self, this: This<JsObject>, protected: bool) -> This<JsObject> {
        self.content_protected = protected;
        this
    }

    #[napi(ts_return_type="this")]
    pub fn with_active(&mut self, this: This<JsObject>, active: bool) -> This<JsObject> {
        self.active = active;
        this
    }

    // #[inline]
    // pub fn with_cursor(&mut self, cursor: impl Into<Cursor>) -> Self {
    //     self.cursor = cursor.into();
    //     self
    // }

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

#[napi(js_name = "Fullscreen")]
#[repr(u8)]
pub enum JsFullscreen {
    Exclusive,
    Borderless
}

impl Into<Fullscreen> for JsFullscreen {
    fn into(self) -> Fullscreen {
        match self {
            Self::Exclusive => unimplemented!("Fullscreen::Exclusive has not implemented"),
            Self::Borderless => Fullscreen::Borderless(None),
        }
    }
}

#[napi(js_name = "WindowButtons")]
#[derive(Copy, Clone)]
pub struct JsWindowButtons {
    pub(crate) close: bool,
    pub(crate) minimize: bool,
    pub(crate) maximize: bool,
}

impl Into<WindowButtons> for JsWindowButtons {
    fn into(self) -> WindowButtons {
        let mut buttons = WindowButtons::empty();
        let Self { close, minimize, maximize } = self;
        if close { buttons.insert(WindowButtons::CLOSE) }
        if minimize { buttons.insert(WindowButtons::MINIMIZE) }
        if maximize { buttons.insert(WindowButtons::MAXIMIZE) }
        buttons
    }
}

#[napi]
impl JsWindowButtons {
    #[napi(factory)]
    pub fn all() -> Self {
        Self { close: true, minimize: true, maximize: true }
    }
    #[napi(factory)]
    pub fn empty() -> Self {
        Self { close: false, minimize: false, maximize: false }
    }
    #[napi(ts_return_type="this")]
    pub fn toggle_close(&mut self, this: This<JsObject>) -> This<JsObject> {
        self.close = !self.close;
        this
    }
    #[napi(ts_return_type="this")]
    pub fn toggle_minimize(&mut self, this: This<JsObject>) -> This<JsObject> {
        self.minimize = !self.minimize;
        this
    }
    #[napi(ts_return_type="this")]
    pub fn toggle_maximize(&mut self, this: This<JsObject>) -> This<JsObject> {
        self.maximize = !self.maximize;
        this
    }
    #[napi(ts_return_type="this")]
    pub fn insert_close(&mut self, this: This<JsObject>) -> This<JsObject> {
        self.close = true;
        this
    }
    #[napi(ts_return_type="this")]
    pub fn insert_minimize(&mut self, this: This<JsObject>) -> This<JsObject> {
        self.minimize = true;
        this
    }
    #[napi(ts_return_type="this")]
    pub fn insert_maximize(&mut self, this: This<JsObject>) -> This<JsObject> {
        self.maximize = true;
        this
    }
    #[napi(ts_return_type="this")]
    pub fn remove_close(&mut self, this: This<JsObject>) -> This<JsObject> {
        self.close = false;
        this
    }
    #[napi(ts_return_type="this")]
    pub fn remove_minimize(&mut self, this: This<JsObject>) -> This<JsObject> {
        self.minimize = false;
        this
    }
    #[napi(ts_return_type="this")]
    pub fn remove_maximize(&mut self, this: This<JsObject>) -> This<JsObject> {
        self.maximize = false;
        this
    }
}

#[napi(js_name = "WindowLevel")]
pub enum JsWindowLevel {
    AlwaysOnBottom,
    Normal,
    AlwaysOnTop,
}

impl Default for JsWindowLevel {
    fn default() -> Self {
        Self::Normal
    }
}

impl Into<WindowLevel> for JsWindowLevel {
    fn into(self) -> WindowLevel {
        match self {
            JsWindowLevel::AlwaysOnBottom => WindowLevel::AlwaysOnBottom,
            JsWindowLevel::Normal => WindowLevel::Normal,
            JsWindowLevel::AlwaysOnTop => WindowLevel::AlwaysOnTop,
        }
    }
}

#[napi(js_name = "Theme")]
pub enum JsTheme {
    Light,
    Dark,
}

impl Into<Theme> for JsTheme {
    fn into(self) -> Theme {
        match self {
            JsTheme::Light => Theme::Light,
            JsTheme::Dark => Theme::Dark,
        }
    }
}

#[napi(js_name = "Icon")]
pub struct JsIcon {
    pub(crate) inner: Icon,
}

impl Into<Icon> for JsIcon {
    fn into(self) -> Icon {
        self.inner
    }
}

#[napi]
impl JsIcon {
    #[napi(factory, ts_return_type = "Icon")]
    pub fn from_rgba(env: Env, rgba: Uint8Array, width: u32, height: u32) -> Result<Self> {
        match Icon::from_rgba(rgba.to_vec(), width, height) {
            Ok(icon) => Ok(Self { inner: icon }),
            Err(bad_icon) => Err(Error::from_reason(format!("{}", bad_icon))),
        }
    }
}