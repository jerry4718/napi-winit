use napi::bindgen_prelude::*;
use proc::proxy_enum;
use crate::{napi_reason, wrap_struct};

/** [`winit::window::CursorIcon`] */
#[proxy_enum(origin_enum = winit::window::CursorIcon, string_enum, non_exhaustive)]
pub enum CursorIcon {
    Default, ContextMenu, Help, Pointer, Progress, Wait, Cell, Crosshair, Text, VerticalText,
    Alias, Copy, Move, NoDrop, NotAllowed, Grab, Grabbing, EResize, NResize, NeResize, NwResize,
    SResize, SeResize, SwResize, WResize, EwResize, NsResize, NeswResize, NwseResize, ColResize,
    RowResize, AllScroll, ZoomIn, ZoomOut
}

impl Default for CursorIcon {
    fn default() -> Self {
        CursorIcon::Default
    }
}

wrap_struct!(#[derive(Clone, Default)] struct Cursor (winit::window::Cursor));

#[napi]
impl Cursor {
    #[napi(factory)]
    pub fn from_icon(icon: CursorIcon) -> Self {
        Self(winit::window::Cursor::Icon(icon.into()))
    }
    #[napi(factory)]
    pub fn from_custom(custom: &CustomCursor) -> Self {
        Self(winit::window::Cursor::Custom(custom.clone().into()))
    }
}

wrap_struct!(#[derive(Clone)] struct CustomCursor { inner: winit::window::CustomCursor });
wrap_struct!(struct CustomCursorSource { inner: winit::window::CustomCursorSource });

#[napi]
impl CustomCursor {
    #[napi]
    pub fn from_rgba(rgba: Uint8Array, width: u16, height: u16, hotspot_x: u16, hotspot_y: u16) -> Result<CustomCursorSource> {
        winit::window::CustomCursor::from_rgba(rgba.to_vec(), width, height, hotspot_x, hotspot_y)
            .map(CustomCursorSource::from)
            .map_err(|e| napi_reason!("{e}"))
    }
}