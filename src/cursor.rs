use napi::bindgen_prelude::*;
use proc::{proxy_enum, proxy_struct};
use crate::napi_reason;

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

#[proxy_struct(origin_type = winit::window::Cursor)]
#[derive(Clone, Default)]
pub struct Cursor;

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

#[proxy_struct(origin_type = winit::window::CustomCursor, field_name = inner)]
#[derive(Clone)]
pub struct CustomCursor;

#[proxy_struct(origin_type = winit::window::CustomCursorSource, field_name = inner)]
pub struct CustomCursorSource;

#[napi]
impl CustomCursor {
    #[napi]
    pub fn from_rgba(rgba: Uint8Array, width: u16, height: u16, hotspot_x: u16, hotspot_y: u16) -> Result<CustomCursorSource> {
        winit::window::CustomCursor::from_rgba(rgba.to_vec(), width, height, hotspot_x, hotspot_y)
            .map(CustomCursorSource::from)
            .map_err(|e| napi_reason!("{e}"))
    }
}