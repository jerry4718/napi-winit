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

#[napi]
#[derive(Clone)]
pub enum Position {
    Physical { x: f64, y: f64 },
    Logical { x: f64, y: f64 },
}

impl From<(f64, f64)> for Position {
    fn from((x, y): (f64, f64)) -> Self {
        Self::Physical { x, y }
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Self::Logical {
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
        Self::Physical {
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
        Self::Logical {
            x: f64::from(x),
            y: f64::from(y),
        }
    }
}

impl From<OriginPosition> for Position {
    fn from(value: OriginPosition) -> Self {
        match value {
            OriginPosition::Physical(physical_position) => Self::from(physical_position),
            OriginPosition::Logical(logical_position) => Self::from(logical_position),
        }
    }
}

impl Into<OriginPosition> for Position {
    fn into(self) -> OriginPosition {
        match self {
            Position::Physical { x, y } => OriginPosition::Physical(OriginPhysicalPosition {
                x: i32::from_f64(x),
                y: i32::from_f64(y),
            }),
            Position::Logical { x, y } => OriginPosition::Logical(OriginLogicalPosition { x, y })
        }
    }
}

#[napi]
#[derive(Clone)]
pub enum Size {
    Physical { width: f64, height: f64 },
    Logical { width: f64, height: f64 },
}

impl<T> From<OriginPhysicalSize<T>> for Size
where
    T: OriginPixel,
    f64: From<T>,
{
    fn from(OriginPhysicalSize { width, height }: OriginPhysicalSize<T>) -> Self {
        Self::Physical {
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
        Self::Logical {
            width: f64::from(width),
            height: f64::from(height),
        }
    }
}

impl From<OriginSize> for Size {
    fn from(value: OriginSize) -> Self {
        match value {
            OriginSize::Physical(physical_size) => Self::from(physical_size),
            OriginSize::Logical(logical_size) => Self::from(logical_size),
        }
    }
}

impl Into<OriginSize> for Size {
    fn into(self) -> OriginSize {
        match self {
            Size::Physical { width, height } => OriginSize::Physical(OriginPhysicalSize {
                width: u32::from_f64(width),
                height: u32::from_f64(height),
            }),
            Size::Logical { width, height } => OriginSize::Logical(OriginLogicalSize { width, height })
        }
    }
}

#[napi]
pub enum PixelUnit {
    Physical { count: f64 },
    Logical { count: f64 },
}

impl<T> From<OriginPhysicalUnit<T>> for PixelUnit
where
    T: OriginPixel,
    f64: From<T>,
{
    fn from(OriginPhysicalUnit(count): OriginPhysicalUnit<T>) -> Self {
        Self::Physical { count: f64::from(count) }
    }
}
impl<T> From<OriginLogicalUnit<T>> for PixelUnit
where
    T: OriginPixel,
    f64: From<T>,
{
    fn from(OriginLogicalUnit(count): OriginLogicalUnit<T>) -> Self {
        Self::Physical { count: f64::from(count) }
    }
}

impl From<OriginPixelUnit> for PixelUnit {
    fn from(value: OriginPixelUnit) -> Self {
        match value {
            OriginPixelUnit::Physical(physical_unit) => Self::from(physical_unit),
            OriginPixelUnit::Logical(logical_unit) => Self::from(logical_unit),
        }
    }
}

impl Into<OriginPixelUnit> for PixelUnit {
    fn into(self) -> OriginPixelUnit {
        match self {
            PixelUnit::Physical { count } => OriginPixelUnit::Physical(OriginPhysicalUnit(i32::from_f64(count))),
            PixelUnit::Logical { count } => OriginPixelUnit::Logical(OriginLogicalUnit(count))
        }
    }
}

mark_ex_into!(
    OriginPhysicalSize<u32>,
    OriginPhysicalPosition<i32>,
    OriginPhysicalPosition<f64>,
    OriginPhysicalPosition<f32>,
    // local
    Position,
    Size,
    PixelUnit
);

mark_ex_into!(
    // f**k
    (usize, usize),
    (f64, f64)
);