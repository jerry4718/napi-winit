use winit::dpi::{LogicalPosition, PhysicalPosition, Pixel, Position};

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
