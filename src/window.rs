use crate::dpi::Position;
use crate::dpi::Size;

use winit::{
    window::{
        CursorIcon as OriginCursorIcon,
        Fullscreen as OriginFullscreen,
        Icon as OriginIcon,
        Theme as OriginTheme,
        Window as OriginWindow,
        WindowId as OriginWindowId,
        WindowAttributes as OriginWindowAttributes,
        WindowButtons as OriginWindowButtons,
        WindowLevel as OriginWindowLevel
    }
};

use napi::bindgen_prelude::*;
use napi::{JsObject, NapiRaw, NapiValue};
use napi::sys::{napi_env, napi_value};
use proc::{mapping_bitflags, mapping_enum, simple_enum};
use crate::cursor::Cursor;
use crate::extra::convert::{ExFrom, ExInto};

#[napi]
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

impl Into<OriginWindowAttributes> for WindowAttributes {
    fn into(self) -> OriginWindowAttributes {
        let attrs = OriginWindowAttributes::default()
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
impl WindowAttributes {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    #[napi(ts_return_type = "this")]
    pub fn with_inner_size(&mut self, this: This<JsObject>, size: Size) -> This<JsObject> {
        self.inner_size = Some(size);
        this
    }

    #[napi(ts_return_type = "this")]
    pub fn with_min_inner_size(&mut self, this: This<JsObject>, min_size: Size) -> This<JsObject> {
        self.min_inner_size = Some(min_size);
        this
    }

    #[napi(ts_return_type = "this")]
    pub fn with_max_inner_size(&mut self, this: This<JsObject>, max_size: Size) -> This<JsObject> {
        self.max_inner_size = Some(max_size);
        this
    }

    #[napi(ts_return_type = "this")]
    pub fn with_position(&mut self, this: This<JsObject>, position: Position) -> This<JsObject> {
        self.position = Some(position);
        this
    }

    #[napi(ts_return_type="this")]
    pub fn with_resizable(&mut self, this: This<JsObject>, resizable: bool) -> This<JsObject> {
        self.resizable = resizable;
        this
    }

    #[napi(ts_return_type="this")]
    pub fn with_enabled_buttons(&mut self, this: This<JsObject>, buttons: &WindowButtons) -> This<JsObject> {
        self.enabled_buttons = *buttons;
        this
    }

    #[napi(ts_return_type="this")]
    pub fn with_title(&mut self, this: This<JsObject>, title: String) -> This<JsObject> {
        self.title = title;
        this
    }

    #[napi(ts_return_type="this")]
    pub fn with_fullscreen(&mut self, this: This<JsObject>, fullscreen: Option<Fullscreen>) -> This<JsObject> {
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
    pub fn with_window_level(&mut self, this: This<JsObject>, level: WindowLevel) -> This<JsObject> {
        self.window_level = level;
        this
    }

    // #[inline]
    pub fn with_window_icon(&mut self, this: This<JsObject>, window_icon: Option<Icon>) -> This<JsObject> {
        self.window_icon = window_icon;
        this
    }

    #[napi(ts_return_type="this")]
    pub fn with_theme(&mut self, this: This<JsObject>, theme: Option<Theme>) -> This<JsObject> {
        self.preferred_theme = theme;
        this
    }

    #[napi(ts_return_type="this")]
    pub fn with_resize_increments(&mut self, this: This<JsObject>, resize_increments: Size) -> This<JsObject> {
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
pub enum Fullscreen {
    Exclusive,
    Borderless
}

impl Into<OriginFullscreen> for Fullscreen {
    fn into(self) -> OriginFullscreen {
        match self {
            Self::Exclusive => unimplemented!("Fullscreen::Exclusive has not implemented"),
            Self::Borderless => OriginFullscreen::Borderless(None),
        }
    }
}

mapping_bitflags!(WindowButtons: CLOSE; MINIMIZE; MAXIMIZE);

simple_enum!(
    enum WindowLevel {
        AlwaysOnBottom,
        Normal,
        AlwaysOnTop,
    }
);

simple_enum!(
    enum Theme {
        Light,
        Dark,
    }
);

#[napi(js_name = "Icon")]
pub struct Icon {
    pub(crate) inner: OriginIcon,
}

impl Into<OriginIcon> for Icon {
    fn into(self) -> OriginIcon {
        self.inner
    }
}

#[napi]
impl Icon {
    #[napi(factory, ts_return_type = "Icon")]
    pub fn from_rgba(env: Env, rgba: Uint8Array, width: u32, height: u32) -> Result<Self> {
        match OriginIcon::from_rgba(rgba.to_vec(), width, height) {
            Ok(icon) => Ok(Self { inner: icon }),
            Err(bad_icon) => Err(Error::from_reason(format!("{}", bad_icon))),
        }
    }
}

pub struct WindowId {
    pub(crate) inner: OriginWindowId,
}