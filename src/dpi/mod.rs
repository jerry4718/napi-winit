use winit::{
    dpi::LogicalPosition as OriginLogicalPosition,
    dpi::LogicalSize as OriginLogicalSize,
    dpi::LogicalUnit as OriginLogicalUnit,
    dpi::PhysicalPosition as OriginPhysicalPosition,
    dpi::PhysicalSize as OriginPhysicalSize,
    dpi::PhysicalUnit as OriginPhysicalUnit,
    dpi::Pixel as OriginPixel,
    dpi::PixelUnit as OriginPixelUnit,
    dpi::Position as OriginPosition,
    dpi::Size as OriginSize,
};
use crate::mark_ex_into;

#[napi(string_enum)]
pub enum UnitType {
    Physical,
    Logical,
}

#[napi(object)]
#[derive(Clone)]
pub struct Position {
    pub r#type: UnitType,
    pub x: f64,
    pub y: f64,
}

impl From<(f64, f64)> for Position {
    fn from((x, y): (f64, f64)) -> Self {
        Self { r#type: UnitType::Physical, x, y }
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Self {
            r#type: UnitType::Logical,
            x: f64::from(x as u32),
            y: f64::from(y as u32)
        }
    }
}

impl<T> From<OriginPhysicalPosition<T>> for Position
where
    T: OriginPixel,
    f64: From<T>,
{
    fn from(OriginPhysicalPosition { x, y }: OriginPhysicalPosition<T>) -> Self {
        Self {
            r#type: UnitType::Physical,
            x: f64::from(x),
            y: f64::from(y),
        }
    }
}

impl<T> From<OriginLogicalPosition<T>> for Position
where
    T: OriginPixel,
    f64: From<T>,
{
    fn from(OriginLogicalPosition { x, y }: OriginLogicalPosition<T>) -> Self {
        Self {
            r#type: UnitType::Logical,
            x: f64::from(x),
            y: f64::from(y),
        }
    }
}

impl From<OriginPosition> for Position {
    fn from(value: OriginPosition) -> Self {
        match value {
            OriginPosition::Physical(physical_position) => physical_position.into(),
            OriginPosition::Logical(logical_position) => logical_position.into(),
        }
    }
}

impl Into<OriginPosition> for Position {
    fn into(self) -> OriginPosition {
        let Self { x, y, .. } = self;

        match self.r#type {
            UnitType::Physical => OriginPosition::Physical(OriginPhysicalPosition {
                x: i32::from_f64(x),
                y: i32::from_f64(y),
            }),
            UnitType::Logical => OriginPosition::Logical(OriginLogicalPosition { x, y })
        }
    }
}

#[napi(object)]
#[derive(Clone)]
pub struct Size {
    pub r#type: UnitType,
    pub width: f64,
    pub height: f64,
}

impl<T> From<OriginPhysicalSize<T>> for Size
where
    T: OriginPixel,
    f64: From<T>,
{
    fn from(OriginPhysicalSize { width, height }: OriginPhysicalSize<T>) -> Self {
        Self {
            r#type: UnitType::Physical,
            width: f64::from(width),
            height: f64::from(height),
        }
    }
}

impl<T> From<OriginLogicalSize<T>> for Size
where
    T: OriginPixel,
    f64: From<T>,
{
    fn from(OriginLogicalSize { width, height }: OriginLogicalSize<T>) -> Self {
        Self {
            r#type: UnitType::Logical,
            width: f64::from(width),
            height: f64::from(height),
        }
    }
}

impl From<OriginSize> for Size {
    fn from(value: OriginSize) -> Self {
        match value {
            OriginSize::Physical(physical_size) => physical_size.into(),
            OriginSize::Logical(logical_size) => logical_size.into(),
        }
    }
}

impl Into<OriginSize> for Size {
    fn into(self) -> OriginSize {
        let Self { width, height, .. } = self;

        match self.r#type {
            UnitType::Physical => OriginSize::Physical(OriginPhysicalSize {
                width: u32::from_f64(width),
                height: u32::from_f64(height),
            }),
            UnitType::Logical => OriginSize::Logical(OriginLogicalSize { width, height })
        }
    }
}

#[napi(object)]
#[derive(Clone)]
pub struct PixelUnit {
    pub r#type: UnitType,
    pub count: f64,
}

impl<T> From<OriginPhysicalUnit<T>> for PixelUnit
where
    T: OriginPixel,
    f64: From<T>,
{
    fn from(OriginPhysicalUnit(count): OriginPhysicalUnit<T>) -> Self {
        Self {
            r#type: UnitType::Physical,
            count: f64::from(count),
        }
    }
}
impl<T> From<OriginLogicalUnit<T>> for PixelUnit
where
    T: OriginPixel,
    f64: From<T>,
{
    fn from(OriginLogicalUnit(count): OriginLogicalUnit<T>) -> Self {
        Self {
            r#type: UnitType::Physical,
            count: f64::from(count),
        }
    }
}

impl From<OriginPixelUnit> for PixelUnit {
    fn from(value: OriginPixelUnit) -> Self {
        match value {
            OriginPixelUnit::Physical(physical_unit) => physical_unit.into(),
            OriginPixelUnit::Logical(logical_unit) => logical_unit.into(),
        }
    }
}

impl Into<OriginPixelUnit> for PixelUnit {
    fn into(self) -> OriginPixelUnit {
        let count = self.count;

        match self.r#type {
            UnitType::Physical => OriginPixelUnit::Physical(OriginPhysicalUnit(i32::from_f64(count))),
            UnitType::Logical => OriginPixelUnit::Logical(OriginLogicalUnit(count))
        }
    }
}

mark_ex_into!(
    OriginPhysicalSize<u32>,
    OriginPhysicalPosition<i32>,
    OriginPhysicalPosition<f64>,
    OriginPhysicalPosition<f32>,
    // local
    UnitType,
    Position,
    Size,
    PixelUnit
);

mark_ex_into!(
    // f**k
    (usize, usize),
    (f64, f64)
);