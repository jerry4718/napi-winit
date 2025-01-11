use std::cell::{RefCell};
use std::ptr::NonNull;
use std::rc::Rc;
use winit::dpi::{Position, Size};
use winit::event_loop::EventLoop;
use winit::window::{Cursor, CursorIcon, Fullscreen, Icon, Theme, Window, WindowAttributes, WindowButtons, WindowLevel};
use crate::dpi::JsPosition;
use crate::event_loop::JsEventLoop;

use napi::bindgen_prelude::*;

#[napi(js_name = "WindowAttributes")]
pub struct JsWindowAttributes {
    pub (crate) inner: Rc<RefCell<WindowAttributes>>,
}

#[napi]
impl JsWindowAttributes {
    #[napi(constructor)]
    pub fn new() -> Self {
        let attrs = WindowAttributes::default();
        Self { inner: Rc::new(RefCell::new(attrs)) }
    }

    // #[inline]
    // pub fn with_inner_size<S: Into<Size>>(mut self, size: S) -> Self {
    //     self.inner_size = Some(size.into());
    //     self
    // }

    // #[inline]
    // pub fn with_min_inner_size<S: Into<Size>>(mut self, min_size: S) -> Self {
    //     self.min_inner_size = Some(min_size.into());
    //     self
    // }

    // #[inline]
    // pub fn with_max_inner_size<S: Into<Size>>(mut self, max_size: S) -> Self {
    //     self.max_inner_size = Some(max_size.into());
    //     self
    // }

    #[napi]
    pub fn with_position(&mut self, position: JsPosition)  {
        let rc = Rc::clone(&self.inner);
        let cell = rc.borrow_mut();
        cell.clone().with_position(position);
        // this
    }

    // #[napi]
    // pub fn with_resizable(&mut self, this: This<JsWindowAttributes>, resizable: bool) -> This<JsWindowAttributes> {
    //     Rc::clone(&self.inner).get_mut().with_resizable(resizable);
    //     this
    // }

    // #[inline]
    // pub fn with_enabled_buttons(mut self, buttons: WindowButtons) -> Self {
    //     self.enabled_buttons = buttons;
    //     self
    // }

    // #[napi]
    // pub fn with_title(&mut self, this: This<JsWindowAttributes>, title: String) -> This<JsWindowAttributes> {
    //     unsafe { self.inner.as_mut() }.with_title(title);
    //     this
    // }

    // #[inline]
    // pub fn with_fullscreen(mut self, fullscreen: Option<Fullscreen>) -> Self {
    //     self.fullscreen = fullscreen;
    //     self
    // }

    // #[inline]
    // pub fn with_maximized(mut self, maximized: bool) -> Self {
    //     self.maximized = maximized;
    //     self
    // }

    // #[inline]
    // pub fn with_visible(mut self, visible: bool) -> Self {
    //     self.visible = visible;
    //     self
    // }

    // #[inline]
    // pub fn with_transparent(mut self, transparent: bool) -> Self {
    //     self.transparent = transparent;
    //     self
    // }

    // #[inline]
    // pub fn transparent(&self) -> bool {
    //     self.transparent
    // }

    // #[inline]
    // pub fn with_blur(mut self, blur: bool) -> Self {
    //     self.blur = blur;
    //     self
    // }

    // #[inline]
    // pub fn with_decorations(mut self, decorations: bool) -> Self {
    //     self.decorations = decorations;
    //     self
    // }

    // #[inline]
    // pub fn with_window_level(mut self, level: WindowLevel) -> Self {
    //     self.window_level = level;
    //     self
    // }

    // #[inline]
    // pub fn with_window_icon(mut self, window_icon: Option<Icon>) -> Self {
    //     self.window_icon = window_icon;
    //     self
    // }

    // #[inline]
    // pub fn with_theme(mut self, theme: Option<Theme>) -> Self {
    //     self.preferred_theme = theme;
    //     self
    // }

    // #[inline]
    // pub fn with_resize_increments<S: Into<Size>>(mut self, resize_increments: S) -> Self {
    //     self.resize_increments = Some(resize_increments.into());
    //     self
    // }

    // #[inline]
    // pub fn with_content_protected(mut self, protected: bool) -> Self {
    //     self.content_protected = protected;
    //     self
    // }

    // #[inline]
    // pub fn with_active(mut self, active: bool) -> Self {
    //     self.active = active;
    //     self
    // }

    // #[inline]
    // pub fn with_cursor(mut self, cursor: impl Into<Cursor>) -> Self {
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