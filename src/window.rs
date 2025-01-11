use crate::dpi::JsPosition;
use crate::dpi::JsSize;
use crate::event_loop::JsEventLoop;
use std::cell::RefCell;
use std::ops::{BitAnd, DerefMut};
use std::ptr::NonNull;
use std::rc::Rc;

use winit::{
    dpi::{Position, Size},
    event_loop::EventLoop,
    window::{Cursor, CursorIcon, Fullscreen, Icon, Theme, Window, WindowAttributes, WindowButtons, WindowLevel}
};

use napi::bindgen_prelude::*;
use napi::JsObject;

#[napi(js_name = "WindowAttributes")]
pub struct JsWindowAttributes {
    pub(crate)  inner_size: Option<JsSize>,
    pub(crate)  min_inner_size: Option<JsSize>,
    pub(crate)  max_inner_size: Option<JsSize>,
    pub(crate)  position: Option<JsPosition>,
    pub(crate)  resizable: bool,
    pub(crate)  enabled_buttons: JsWindowButtons,
    pub(crate)  title: String,
    pub(crate)  maximized: bool,
    pub(crate)  visible: bool,
    pub(crate)  transparent: bool,
    pub(crate)  blur: bool,
    pub(crate)  decorations: bool,
    // pub window_icon: Option<Icon>,
    // pub preferred_theme: Option<Theme>,
    pub(crate)  resize_increments: Option<JsSize>,
    pub(crate)  content_protected: bool,
    // pub window_level: WindowLevel,
    pub(crate)  active: bool,
    pub(crate)  cursor: Cursor,
    // #[cfg(feature = "rwh_06")]
    // pub(crate) parent_window: Option<SendSyncRawWindowHandle>,
    pub(crate)  fullscreen: Option<JsFullscreen>,
    // Platform-specific configuration.
    // #[allow(dead_code)]
    // pub(crate) platform_specific: PlatformSpecificWindowAttributes,
}

impl Default for JsWindowAttributes {
    #[inline]
    fn default() -> JsWindowAttributes {
        JsWindowAttributes {
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
            // window_level: Default::default(),
            // window_icon: None,
            // preferred_theme: None,
            resize_increments: None,
            content_protected: false,
            cursor: Cursor::default(),
            // #[cfg(feature = "rwh_06")]
            // parent_window: None,
            active: true,
            // platform_specific: Default::default(),
        }
    }
}

impl Into<WindowAttributes> for JsWindowAttributes {
    fn into(self) -> WindowAttributes {
        let attrs = WindowAttributes::default()
            .with_resizable(self.resizable)
            .with_title(self.title)
            .with_maximized(self.maximized)
            .with_visible(self.visible)
            .with_transparent(self.transparent)
            .with_blur(self.blur)
            .with_decorations(self.decorations)
            .with_content_protected(self.content_protected);

        let attrs = if let Some(inner_size) = self.inner_size {
            attrs.with_inner_size(inner_size)
        } else { attrs };
        let attrs = if let Some(min_inner_size) = self.min_inner_size {
            attrs.with_min_inner_size(min_inner_size)
        } else { attrs };
        let attrs = if let Some(max_inner_size) = self.max_inner_size {
            attrs.with_max_inner_size(max_inner_size)
        } else { attrs };
        let attrs = if let Some(position) = self.position {
            attrs.with_position(position)
        } else { attrs };
        let attrs = if let Some(fullscreen) = self.fullscreen {
            attrs.with_fullscreen(Some(fullscreen.into()))
        } else { attrs };
        let attrs = if let Some(resize_increments) = self.resize_increments {
            attrs.with_resize_increments(resize_increments)
        } else { attrs };

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
    pub fn with_enabled_buttons(&mut self, this: This<JsObject>, #[napi(ts_arg_type = "WindowButtons")] buttons: JsWindowButtons) -> This<JsObject> {
        self.enabled_buttons = buttons;
        this
    }

    #[napi(ts_return_type="this")]
    pub fn with_title(&mut self, this: This<JsObject>, title: String) -> This<JsObject> {
        self.title = title;
        this
    }

    #[napi(ts_return_type="this")]
    pub fn with_fullscreen(&mut self, this: This<JsObject>, #[napi(ts_arg_type = "Fullscreen")] fullscreen: Option<JsFullscreen>) -> This<JsObject> {
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

    // #[inline]
    // pub fn with_window_level(&mut self, level: WindowLevel) -> Self {
    //     self.window_level = level;
    //     self
    // }

    // #[inline]
    // pub fn with_window_icon(&mut self, window_icon: Option<Icon>) -> Self {
    //     self.window_icon = window_icon;
    //     self
    // }

    // #[inline]
    // pub fn with_theme(&mut self, theme: Option<Theme>) -> Self {
    //     self.preferred_theme = theme;
    //     self
    // }

    // #[inline]
    // pub fn with_resize_increments<S: Into<Size>>(&mut self, resize_increments: S) -> Self {
    //     self.resize_increments = Some(resize_increments.into());
    //     self
    // }

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
pub struct JsWindowButtons {
    pub(crate) buttons: WindowButtons,
}

impl Into<WindowButtons> for JsWindowButtons {
    fn into(self) -> WindowButtons {
        self.buttons
    }
}

#[napi]
impl JsWindowButtons {
    #[napi(factory)]
    pub fn all() -> Self {
        Self { buttons: WindowButtons::all() }
    }
    #[napi(factory)]
    pub fn empty() -> Self {
        Self { buttons: WindowButtons::empty() }
    }
    #[napi(ts_return_type="this")]
    pub fn remove_close(&mut self, this: This<JsObject>) -> This<JsObject> {
        self.buttons.remove(WindowButtons::CLOSE);
        this
    }
    #[napi(ts_return_type="this")]
    pub fn remove_minimize(&mut self, this: This<JsObject>) -> This<JsObject> {
        self.buttons.remove(WindowButtons::MINIMIZE);
        this
    }
    #[napi(ts_return_type="this")]
    pub fn remove_maximize(&mut self, this: This<JsObject>) -> This<JsObject> {
        self.buttons.remove(WindowButtons::MAXIMIZE);
        this
    }
    #[napi(ts_return_type="this")]
    pub fn insert_close(&mut self, this: This<JsObject>) -> This<JsObject> {
        self.buttons.insert(WindowButtons::CLOSE);
        this
    }
    #[napi(ts_return_type="this")]
    pub fn insert_minimize(&mut self, this: This<JsObject>) -> This<JsObject> {
        self.buttons.insert(WindowButtons::MINIMIZE);
        this
    }
    #[napi(ts_return_type="this")]
    pub fn insert_maximize(&mut self, this: This<JsObject>) -> This<JsObject> {
        self.buttons.insert(WindowButtons::MAXIMIZE);
        this
    }
}
