use napi::bindgen_prelude::*;

use winit::window::{
    Cursor as OriginCursor,
    CursorIcon as OriginCursorIcon,
    CustomCursor as OriginCustomCursor,
    CustomCursorSource as OriginCustomCursorSource,
    Icon as OriginIcon
};

use crate::{mark_ex_into, string_enum, wrap_struct};

string_enum!(
    enum CursorIcon => OriginCursorIcon {
        Default, ContextMenu, Help, Pointer, Progress, Wait, Cell, Crosshair, Text, VerticalText,
        Alias, Copy, Move, NoDrop, NotAllowed, Grab, Grabbing, EResize, NResize, NeResize, NwResize,
        SResize, SeResize, SwResize, WResize, EwResize, NsResize, NeswResize, NwseResize, ColResize,
        RowResize, AllScroll, ZoomIn, ZoomOut
    }
    "never reach here"
);

impl Default for CursorIcon {
    fn default() -> Self {
        CursorIcon::Default
    }
}

wrap_struct!(#[derive(Clone, Default)] struct Cursor ( OriginCursor ));

#[napi]
impl Cursor {
    #[napi(factory)]
    pub fn from_icon(icon: CursorIcon) -> Self {
        Self(OriginCursor::Icon(icon.into()))
    }
    #[napi(factory)]
    pub fn from_custom(custom: &CustomCursor) -> Self {
        Self(OriginCursor::Custom(custom.clone().into()))
    }
}

wrap_struct!(#[derive(Clone)] struct CustomCursor { inner: OriginCustomCursor });
wrap_struct!(struct CustomCursorSource { inner: OriginCustomCursorSource });

#[napi]
impl CustomCursor {
    #[napi]
    pub fn from_rgba(rgba: Uint8Array, width: u16, height: u16, hotspot_x: u16, hotspot_y: u16) -> Result<CustomCursorSource> {
        OriginCustomCursor::from_rgba(rgba.to_vec(), width, height, hotspot_x, hotspot_y)
            .map(CustomCursorSource::from)
            .map_err(|e| Error::from_reason(format!("{}", e)))
    }
}

mark_ex_into!(
    OriginCursorIcon,
    OriginCursor,
    OriginCustomCursor,
    OriginCustomCursorSource,
    CursorIcon,
    Cursor,
    CustomCursor,
    CustomCursorSource
);