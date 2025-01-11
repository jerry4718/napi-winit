use winit::dpi::{LogicalPosition, LogicalSize, LogicalUnit, PhysicalPosition, PhysicalSize, PhysicalUnit, Pixel, PixelUnit, Position, Size};

#[napi(js_name = "PositionType")]
#[repr(u8)]
pub enum JsPositionType {
    Physical,
    Logical,
}

#[napi(object, js_name = "PositionData")]
pub struct JsPositionData {
    pub x: f64,
    pub y: f64,
}

#[napi(object, js_name = "Position")]
pub struct JsPosition {
    pub r#type: JsPositionType,
    pub r#data: JsPositionData,
}

impl From<Position> for JsPosition {
    fn from(value: Position) -> Self {
        match value {
            Position::Physical(PhysicalPosition { x, y }) => Self {
                r#type: JsPositionType::Physical,
                r#data: JsPositionData {
                    x: f64::from(x),
                    y: f64::from(y),
                },
            },
            Position::Logical(LogicalPosition { x, y }) => Self {
                r#type: JsPositionType::Logical,
                r#data: JsPositionData { x, y },
            },
        }
    }
}

impl Into<Position> for JsPosition {
    fn into(self) -> Position {
        let JsPositionData {x, y } = self.r#data;

        match self.r#type {
            JsPositionType::Physical => Position::Physical(PhysicalPosition {
                x: i32::from_f64(x),
                y: i32::from_f64(y),
            }),
            JsPositionType::Logical => Position::Logical(LogicalPosition { x, y })
        }
    }
}


#[napi(js_name = "SizeType")]
#[repr(u8)]
pub enum JsSizeType {
    Physical,
    Logical,
}

#[napi(object, js_name = "SizeData")]
pub struct JsSizeData {
    pub width: f64,
    pub height: f64,
}

#[napi(object, js_name = "Size")]
pub struct JsSize {
    pub r#type: JsSizeType,
    pub r#data: JsSizeData,
}

impl From<Size> for JsSize {
    fn from(value: Size) -> Self {
        match value {
            Size::Physical(PhysicalSize { width, height }) => Self {
                r#type: JsSizeType::Physical,
                r#data: JsSizeData {
                    width: f64::from(width),
                    height: f64::from(height),
                },
            },
            Size::Logical(LogicalSize { width, height }) => Self {
                r#type: JsSizeType::Logical,
                r#data: JsSizeData { width, height },
            },
        }
    }
}

impl Into<Size> for JsSize {
    fn into(self) -> Size {
        let JsSizeData {width, height } = self.r#data;

        match self.r#type {
            JsSizeType::Physical => Size::Physical(PhysicalSize {
                width: u32::from_f64(width),
                height: u32::from_f64(height),
            }),
            JsSizeType::Logical => Size::Logical(LogicalSize { width, height })
        }
    }
}


#[napi(js_name = "UnitType")]
#[repr(u8)]
pub enum JsUnitType {
    Physical,
    Logical,
}

#[napi(object, js_name = "PixelUnit")]
pub struct JsPixelUnit {
    pub r#type: JsUnitType,
    pub r#data: f64,
}

impl From<PixelUnit> for JsPixelUnit {
    fn from(value: PixelUnit) -> Self {
        match value {
            PixelUnit::Physical(PhysicalUnit(count)) => Self {
                r#type: JsUnitType::Physical,
                r#data: f64::from(count),
            },
            PixelUnit::Logical(LogicalUnit(count)) => Self {
                r#type: JsUnitType::Logical,
                r#data: count,
            },
        }
    }
}

impl Into<PixelUnit> for JsPixelUnit {
    fn into(self) -> PixelUnit {
        let count = self.r#data;

        match self.r#type {
            JsUnitType::Physical => PixelUnit::Physical(PhysicalUnit(i32::from_f64(count))),
            JsUnitType::Logical => PixelUnit::Logical(LogicalUnit(count))
        }
    }
}