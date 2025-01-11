use winit::dpi::{
    LogicalPosition as WLogicalPosition,
    LogicalSize as WLogicalSize,
    LogicalUnit as WLogicalUnit,
    PhysicalPosition as WPhysicalPosition,
    PhysicalSize as WPhysicalSize,
    PhysicalUnit as WPhysicalUnit,
    Pixel as WPixel,
    PixelUnit as WPixelUnit,
    Position as WPosition,
    Size as WSize
};

#[napi]
#[repr(u8)]
pub enum UnitType {
    Physical,
    Logical,
}

#[napi(object)]
pub struct Position {
    pub r#type: UnitType,
    pub x: f64,
    pub y: f64,
}

impl From<WPosition> for Position {
    fn from(value: WPosition) -> Self {
        match value {
            WPosition::Physical(WPhysicalPosition { x, y }) => Self {
                r#type: UnitType::Physical,
                x: f64::from(x),
                y: f64::from(y),
            },
            WPosition::Logical(WLogicalPosition { x, y }) => Self {
                r#type: UnitType::Logical, x, y
            },
        }
    }
}

impl Into<WPosition> for Position {
    fn into(self) -> WPosition {
        let Self { x, y, .. } = self;

        match self.r#type {
            UnitType::Physical => WPosition::Physical(WPhysicalPosition {
                x: i32::from_f64(x),
                y: i32::from_f64(y),
            }),
            UnitType::Logical => WPosition::Logical(WLogicalPosition { x, y })
        }
    }
}

#[napi(object)]
pub struct Size {
    pub r#type: UnitType,
    pub width: f64,
    pub height: f64,
}

impl From<WSize> for Size {
    fn from(value: WSize) -> Self {
        match value {
            WSize::Physical(WPhysicalSize { width, height }) => Self {
                r#type: UnitType::Physical,
                width: f64::from(width),
                height: f64::from(height),
            },
            WSize::Logical(WLogicalSize { width, height }) => Self {
                r#type: UnitType::Logical, width, height,
            },
        }
    }
}

impl Into<WSize> for Size {
    fn into(self) -> WSize {
        let Self {width, height, .. } = self;

        match self.r#type {
            UnitType::Physical => WSize::Physical(WPhysicalSize {
                width: u32::from_f64(width),
                height: u32::from_f64(height),
            }),
            UnitType::Logical => WSize::Logical(WLogicalSize { width, height })
        }
    }
}

#[napi(object, js_name = "PixelUnit")]
pub struct PixelUnit {
    pub r#type: UnitType,
    pub count: f64,
}

impl From<WPixelUnit> for PixelUnit {
    fn from(value: WPixelUnit) -> Self {
        match value {
            WPixelUnit::Physical(WPhysicalUnit(count)) => Self {
                r#type: UnitType::Physical,
                count: f64::from(count),
            },
            WPixelUnit::Logical(WLogicalUnit(count)) => Self {
                r#type: UnitType::Logical, count,
            },
        }
    }
}

impl Into<WPixelUnit> for PixelUnit {
    fn into(self) -> WPixelUnit {
        let count = self.count;

        match self.r#type {
            UnitType::Physical => WPixelUnit::Physical(WPhysicalUnit(i32::from_f64(count))),
            UnitType::Logical => WPixelUnit::Logical(WLogicalUnit(count))
        }
    }
}