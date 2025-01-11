use winit::dpi::{LogicalPosition, LogicalSize, LogicalUnit, PhysicalPosition, PhysicalSize, PhysicalUnit, Pixel, PixelUnit, Position, Size};

#[napi(js_name = "UnitType")]
#[repr(u8)]
pub enum JsUnitType {
    Physical,
    Logical,
}

#[napi(object, js_name = "Position")]
pub struct JsPosition {
    pub r#type: JsUnitType,
    pub x: f64,
    pub y: f64,
}

impl From<Position> for JsPosition {
    fn from(value: Position) -> Self {
        match value {
            Position::Physical(PhysicalPosition { x, y }) => Self {
                r#type: JsUnitType::Physical,
                x: f64::from(x),
                y: f64::from(y),
            },
            Position::Logical(LogicalPosition { x, y }) => Self {
                r#type: JsUnitType::Logical, x, y
            },
        }
    }
}

impl Into<Position> for JsPosition {
    fn into(self) -> Position {
        let Self { x, y, .. } = self;

        match self.r#type {
            JsUnitType::Physical => Position::Physical(PhysicalPosition {
                x: i32::from_f64(x),
                y: i32::from_f64(y),
            }),
            JsUnitType::Logical => Position::Logical(LogicalPosition { x, y })
        }
    }
}

#[napi(object, js_name = "Size")]
pub struct JsSize {
    pub r#type: JsUnitType,
    pub width: f64,
    pub height: f64,
}

impl From<Size> for JsSize {
    fn from(value: Size) -> Self {
        match value {
            Size::Physical(PhysicalSize { width, height }) => Self {
                r#type: JsUnitType::Physical,
                width: f64::from(width),
                height: f64::from(height),
            },
            Size::Logical(LogicalSize { width, height }) => Self {
                r#type: JsUnitType::Logical, width, height,
            },
        }
    }
}

impl Into<Size> for JsSize {
    fn into(self) -> Size {
        let Self {width, height, .. } = self;

        match self.r#type {
            JsUnitType::Physical => Size::Physical(PhysicalSize {
                width: u32::from_f64(width),
                height: u32::from_f64(height),
            }),
            JsUnitType::Logical => Size::Logical(LogicalSize { width, height })
        }
    }
}

#[napi(object, js_name = "PixelUnit")]
pub struct JsPixelUnit {
    pub r#type: JsUnitType,
    pub count: f64,
}

impl From<PixelUnit> for JsPixelUnit {
    fn from(value: PixelUnit) -> Self {
        match value {
            PixelUnit::Physical(PhysicalUnit(count)) => Self {
                r#type: JsUnitType::Physical,
                count: f64::from(count),
            },
            PixelUnit::Logical(LogicalUnit(count)) => Self {
                r#type: JsUnitType::Logical, count,
            },
        }
    }
}

impl Into<PixelUnit> for JsPixelUnit {
    fn into(self) -> PixelUnit {
        let count = self.count;

        match self.r#type {
            JsUnitType::Physical => PixelUnit::Physical(PhysicalUnit(i32::from_f64(count))),
            JsUnitType::Logical => PixelUnit::Logical(LogicalUnit(count))
        }
    }
}