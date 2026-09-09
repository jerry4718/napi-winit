use winit::dpi::{
    LogicalPosition as OriginLogicalPosition,
    LogicalSize as OriginLogicalSize,
    LogicalUnit as OriginLogicalUnit,
    PhysicalPosition as OriginPhysicalPosition,
    PhysicalSize as OriginPhysicalSize,
    PhysicalUnit as OriginPhysicalUnit,
    Pixel as OriginPixel,
    PixelUnit as OriginPixelUnit,
    Position as OriginPosition,
    Size as OriginSize
};
use crate::napi_reason;

#[napi]
#[derive(Clone)]
pub enum Position {
    Physical { x: f64, y: f64 },
    Logical { x: f64, y: f64 },
}

// Used by `DeviceEvent::PointerMotion { delta: (f64, f64) }`: winit documents the delta as
// "raw, unfiltered physical motion" in "unspecified units", so it must stay tagged Physical —
// a Logical tag would invite consumers to scale it by the DPI factor.
impl From<(f64, f64)> for Position {
    fn from((x, y): (f64, f64)) -> Self {
        Self::Physical { x, y }
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

impl From<Position> for OriginPosition {
    fn from(value: Position) -> OriginPosition {
        match value {
            Position::Physical { x, y } => OriginPosition::Physical(OriginPhysicalPosition {
                x: i32::from_f64(x),
                y: i32::from_f64(y),
            }),
            Position::Logical { x, y } => OriginPosition::Logical(OriginLogicalPosition { x, y })
        }
    }
}

// JS boundary validation: NaN/infinite coordinates are rejected instead of silently
// saturating to zero inside `Pixel::from_f64`. Negative physical positions are valid
// (multi-monitor coordinates extend past the origin), so only finiteness is enforced.
pub(crate) fn try_std_position(value: &Position) -> napi::Result<OriginPosition> {
    let (x, y) = match value {
        Position::Physical { x, y } | Position::Logical { x, y } => (*x, *y),
    };
    (x.is_finite() && y.is_finite())
        .then(|| value.clone().into())
        .ok_or_else(|| napi_reason!("position must be a finite number"))
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

impl From<Size> for OriginSize {
    fn from(value: Size) -> OriginSize {
        match value {
            Size::Physical { width, height } => OriginSize::Physical(OriginPhysicalSize {
                width: u32::from_f64(width),
                height: u32::from_f64(height),
            }),
            Size::Logical { width, height } => OriginSize::Logical(OriginLogicalSize { width, height })
        }
    }
}

// JS boundary validation: a size is non-negative by definition, so negative and
// NaN/infinite values are rejected instead of silently clamping to zero.
pub(crate) fn try_std_size(value: &Size) -> napi::Result<OriginSize> {
    let (width, height) = match value {
        Size::Physical { width, height } | Size::Logical { width, height } => (*width, *height),
    };
    (width.is_finite() && height.is_finite() && width >= 0.0 && height >= 0.0)
        .then(|| value.clone().into())
        .ok_or_else(|| napi_reason!("size must be a non-negative finite number"))
}

#[napi]
#[derive(Clone)]
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
        Self::Logical { count: f64::from(count) }
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

impl From<PixelUnit> for OriginPixelUnit {
    fn from(value: PixelUnit) -> OriginPixelUnit {
        match value {
            PixelUnit::Physical { count } => OriginPixelUnit::Physical(OriginPhysicalUnit(i32::from_f64(count))),
            PixelUnit::Logical { count } => OriginPixelUnit::Logical(OriginLogicalUnit(count))
        }
    }
}