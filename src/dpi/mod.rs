use winit::dpi::{LogicalPosition as WLogicalPosition};
use winit::dpi::LogicalSize as WLogicalSize;
use winit::dpi::LogicalUnit as WLogicalUnit;
use winit::dpi::PhysicalPosition as WPhysicalPosition;
use winit::dpi::PhysicalSize as WPhysicalSize;
use winit::dpi::PhysicalUnit as WPhysicalUnit;
use winit::dpi::Pixel as WPixel;
use winit::dpi::PixelUnit as WPixelUnit;
use winit::dpi::Position as WPosition;
use winit::dpi::Size as WSize;

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

impl From<(f64, f64)> for Position {
    fn from((x, y): (f64, f64)) -> Self {
        Self { r#type: UnitType::Logical, x, y }
    }
}

impl<T> From<WPhysicalPosition<T>> for Position
where
    T: WPixel,
    f64: From<T>
{
    fn from(WPhysicalPosition { x, y }: WPhysicalPosition<T>) -> Self {
        Self {
            r#type: UnitType::Physical,
            x: f64::from(x),
            y: f64::from(y),
        }
    }
}

impl<T> From<WLogicalPosition<T>> for Position
where
    T: WPixel,
    f64: From<T>
{
    fn from(WLogicalPosition { x, y }: WLogicalPosition<T>) -> Self {
        Self {
            r#type: UnitType::Logical,
            x: f64::from(x),
            y: f64::from(y),
        }
    }
}

impl From<WPosition> for Position {
    fn from(value: WPosition) -> Self {
        match value {
            WPosition::Physical(physical_position) => physical_position.into(),
            WPosition::Logical(logical_position) => logical_position.into(),
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

impl<T> From<WPhysicalSize<T>> for Size
where
    T: WPixel,
    f64: From<T>
{
    fn from(WPhysicalSize { width, height }: WPhysicalSize<T>) -> Self {
        Self {
            r#type: UnitType::Physical,
            width: f64::from(width),
            height: f64::from(height),
        }
    }
}

impl<T> From<WLogicalSize<T>> for Size
where
    T: WPixel,
    f64: From<T>
{
    fn from(WLogicalSize { width, height }: WLogicalSize<T>) -> Self {
        Self {
            r#type: UnitType::Logical,
            width: f64::from(width),
            height: f64::from(height),
        }
    }
}

impl From<WSize> for Size {
    fn from(value: WSize) -> Self {
        match value {
            WSize::Physical(physical_size) => physical_size.into(),
            WSize::Logical(logical_size) => logical_size.into(),
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

impl<T> From<WPhysicalUnit<T>> for PixelUnit
where
    T: WPixel,
    f64: From<T>
{
    fn from(WPhysicalUnit(count): WPhysicalUnit<T>) -> Self {
        Self {
            r#type: UnitType::Physical,
            count: f64::from(count),
        }
    }
}
impl<T> From<WLogicalUnit<T>> for PixelUnit
where
    T: WPixel,
    f64: From<T>
{
    fn from(WLogicalUnit(count): WLogicalUnit<T>) -> Self {
        Self {
            r#type: UnitType::Physical,
            count: f64::from(count),
        }
    }
}

impl From<WPixelUnit> for PixelUnit {
    fn from(value: WPixelUnit) -> Self {
        match value {
            WPixelUnit::Physical(physical_unit) => physical_unit.into(),
            WPixelUnit::Logical(logical_unit) => logical_unit.into(),
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