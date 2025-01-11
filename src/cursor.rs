use napi::bindgen_prelude::*;
use winit::window::{Cursor, CursorIcon, CustomCursor, Icon};

#[napi(js_name = "CursorIcon")]
pub enum JsCursorIcon {
    Default,
    ContextMenu,
    Help,
    Pointer,
    Progress,
    Wait,
    Cell,
    Crosshair,
    Text,
    VerticalText,
    Alias,
    Copy,
    Move,
    NoDrop,
    NotAllowed,
    Grab,
    Grabbing,
    EResize,
    NResize,
    NeResize,
    NwResize,
    SResize,
    SeResize,
    SwResize,
    WResize,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
    ColResize,
    RowResize,
    AllScroll,
    ZoomIn,
    ZoomOut,
}

impl Default for JsCursorIcon {
    fn default() -> Self {
        JsCursorIcon::Default
    }
}

impl Into<CursorIcon> for JsCursorIcon {
    fn into(self) -> CursorIcon {
       match self {
           JsCursorIcon::Default => CursorIcon::Default,
           JsCursorIcon::ContextMenu => CursorIcon::ContextMenu,
           JsCursorIcon::Help => CursorIcon::Help,
           JsCursorIcon::Pointer => CursorIcon::Pointer,
           JsCursorIcon::Progress => CursorIcon::Progress,
           JsCursorIcon::Wait => CursorIcon::Wait,
           JsCursorIcon::Cell => CursorIcon::Cell,
           JsCursorIcon::Crosshair => CursorIcon::Crosshair,
           JsCursorIcon::Text => CursorIcon::Text,
           JsCursorIcon::VerticalText => CursorIcon::VerticalText,
           JsCursorIcon::Alias => CursorIcon::Alias,
           JsCursorIcon::Copy => CursorIcon::Copy,
           JsCursorIcon::Move => CursorIcon::Move,
           JsCursorIcon::NoDrop => CursorIcon::NoDrop,
           JsCursorIcon::NotAllowed => CursorIcon::NotAllowed,
           JsCursorIcon::Grab => CursorIcon::Grab,
           JsCursorIcon::Grabbing => CursorIcon::Grabbing,
           JsCursorIcon::EResize => CursorIcon::EResize,
           JsCursorIcon::NResize => CursorIcon::NResize,
           JsCursorIcon::NeResize => CursorIcon::NeResize,
           JsCursorIcon::NwResize => CursorIcon::NwResize,
           JsCursorIcon::SResize => CursorIcon::SResize,
           JsCursorIcon::SeResize => CursorIcon::SeResize,
           JsCursorIcon::SwResize => CursorIcon::SwResize,
           JsCursorIcon::WResize => CursorIcon::WResize,
           JsCursorIcon::EwResize => CursorIcon::EwResize,
           JsCursorIcon::NsResize => CursorIcon::NsResize,
           JsCursorIcon::NeswResize => CursorIcon::NeswResize,
           JsCursorIcon::NwseResize => CursorIcon::NwseResize,
           JsCursorIcon::ColResize => CursorIcon::ColResize,
           JsCursorIcon::RowResize => CursorIcon::RowResize,
           JsCursorIcon::AllScroll => CursorIcon::AllScroll,
           JsCursorIcon::ZoomIn => CursorIcon::ZoomIn,
           JsCursorIcon::ZoomOut => CursorIcon::ZoomOut,
       }
    }
}


#[napi(js_name = "Cursor")]
pub struct JsCursor {
    pub(crate) inner: Cursor
}

impl Into<Cursor> for JsCursor {
    fn into(self) -> Cursor {
        self.inner
    }
}

impl Default for JsCursor {
    fn default() -> Self {
        Self { inner: Cursor::default() }
    }
}

#[napi]
impl JsCursor {
    #[napi(factory, ts_return_type = "Cursor")]
    pub fn from_icon(#[napi(ts_arg_type = "CursorIcon")] icon: JsCursorIcon) -> Self {
        Self { inner: Cursor::Icon(icon.into()) }
    }
    // #[napi(factory)]
    // pub fn from_rgba(
    //     rgba: Uint8Array,
    //     width: u16,
    //     height: u16,
    //     hotspot_x: u16,
    //     hotspot_y: u16
    // ) -> Result<Self> {
    //     match CustomCursor::from_rgba(rgba.to_vec(), width, height, hotspot_x, hotspot_y) {
    //         Ok(image) => Ok(Self { inner: Cursor::Custom(image) }),
    //         Err(bad_image) => Err(Error::from_reason(format!("{}", bad_image))),
    //     }
    // }
}