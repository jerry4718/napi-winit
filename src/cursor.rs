use napi::bindgen_prelude::*;
use winit::window::{
    Cursor as WCursor,
    CursorIcon as WCursorIcon,
    CustomCursor as WCustomCursor,
    Icon as WIcon
};

#[napi(string_enum)]
pub enum CursorIcon {
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

impl Default for CursorIcon {
    fn default() -> Self {
        CursorIcon::Default
    }
}

impl Into<WCursorIcon> for CursorIcon {
    fn into(self) -> WCursorIcon {
       match self {
           CursorIcon::Default => WCursorIcon::Default,
           CursorIcon::ContextMenu => WCursorIcon::ContextMenu,
           CursorIcon::Help => WCursorIcon::Help,
           CursorIcon::Pointer => WCursorIcon::Pointer,
           CursorIcon::Progress => WCursorIcon::Progress,
           CursorIcon::Wait => WCursorIcon::Wait,
           CursorIcon::Cell => WCursorIcon::Cell,
           CursorIcon::Crosshair => WCursorIcon::Crosshair,
           CursorIcon::Text => WCursorIcon::Text,
           CursorIcon::VerticalText => WCursorIcon::VerticalText,
           CursorIcon::Alias => WCursorIcon::Alias,
           CursorIcon::Copy => WCursorIcon::Copy,
           CursorIcon::Move => WCursorIcon::Move,
           CursorIcon::NoDrop => WCursorIcon::NoDrop,
           CursorIcon::NotAllowed => WCursorIcon::NotAllowed,
           CursorIcon::Grab => WCursorIcon::Grab,
           CursorIcon::Grabbing => WCursorIcon::Grabbing,
           CursorIcon::EResize => WCursorIcon::EResize,
           CursorIcon::NResize => WCursorIcon::NResize,
           CursorIcon::NeResize => WCursorIcon::NeResize,
           CursorIcon::NwResize => WCursorIcon::NwResize,
           CursorIcon::SResize => WCursorIcon::SResize,
           CursorIcon::SeResize => WCursorIcon::SeResize,
           CursorIcon::SwResize => WCursorIcon::SwResize,
           CursorIcon::WResize => WCursorIcon::WResize,
           CursorIcon::EwResize => WCursorIcon::EwResize,
           CursorIcon::NsResize => WCursorIcon::NsResize,
           CursorIcon::NeswResize => WCursorIcon::NeswResize,
           CursorIcon::NwseResize => WCursorIcon::NwseResize,
           CursorIcon::ColResize => WCursorIcon::ColResize,
           CursorIcon::RowResize => WCursorIcon::RowResize,
           CursorIcon::AllScroll => WCursorIcon::AllScroll,
           CursorIcon::ZoomIn => WCursorIcon::ZoomIn,
           CursorIcon::ZoomOut => WCursorIcon::ZoomOut,
       }
    }
}


#[napi]
pub struct Cursor {
    pub(crate) inner: WCursor
}

impl Into<WCursor> for Cursor {
    fn into(self) -> WCursor {
        self.inner
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self { inner: WCursor::default() }
    }
}

#[napi]
impl Cursor {
    #[napi(factory, ts_return_type = "Cursor")]
    pub fn from_icon(#[napi(ts_arg_type = "CursorIcon")] icon: CursorIcon) -> Self {
        Self { inner: WCursor::Icon(icon.into()) }
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