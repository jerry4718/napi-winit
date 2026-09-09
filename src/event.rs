use napi::bindgen_prelude::*;

use proc::{proxy_enum, proxy_struct, proxy_wrap};

use crate::{
    dpi::{Position, Size},
    event_loop::AsyncRequestSerial,
    extra::time::Instant,
    keyboard::{Key, KeyLocation, ModifiersState, PhysicalKey},
    utils::helpers::{option_into, option_map},
    window::{ActivationToken, Theme, WindowId},
};

/** [winit::event::StartCause] */
#[proxy_enum(origin_type = winit::event::StartCause, skip_backward, non_exhaustive)]
pub enum StartCause {
    ResumeTimeReached {
        start: Instant,
        requested_resume: Instant,
    },
    WaitCancelled {
        start: Instant,
        #[proxy_enum(from_origin = option_into)]
        requested_resume: Option<Instant>,
    },
    Poll,
    Init,
}

#[proxy_enum(origin_type = winit::event::WindowEvent, skip_backward, non_exhaustive, skip_from_js)]
pub enum WindowEvent {
    ActivationTokenDone {
        serial: AsyncRequestSerial,
        token: ActivationToken,
    },
    SurfaceResized(#[proxy_enum(field_name = size)] Size),
    Moved(#[proxy_enum(field_name = position)] Position),
    CloseRequested,
    Destroyed,
    DragEntered {
        id: DataTransferId,
        #[proxy_enum(from_origin = option_into)]
        position: Option<Position>,
    },
    DragPosition {
        id: DataTransferId,
        position: Position,
        #[proxy_enum(from_origin = option_into)]
        proposed_action: Option<DndAction>,
    },
    DragDropped {
        id: DataTransferId,
        #[proxy_enum(from_origin = option_into)]
        proposed_action: Option<DndAction>,
    },
    DragLeft {
        id: DataTransferId,
    },
    Focused(#[proxy_enum(field_name = focused)] bool),
    KeyboardInput {
        #[proxy_enum(from_origin = option_into)]
        device_id: Option<DeviceId>,
        event: KeyEvent,
        is_synthetic: bool,
    },
    ModifiersChanged(#[proxy_enum(field_name = modifiers)] Modifiers),
    Ime(#[proxy_enum(field_name = ime)] Ime),
    PointerMoved {
        #[proxy_enum(from_origin = option_into)]
        device_id: Option<DeviceId>,
        position: Position,
        primary: bool,
        source: PointerSource,
    },
    PointerEntered {
        #[proxy_enum(from_origin = option_into)]
        device_id: Option<DeviceId>,
        position: Position,
        primary: bool,
        kind: PointerKind,
    },
    PointerLeft {
        #[proxy_enum(from_origin = option_into)]
        device_id: Option<DeviceId>,
        #[proxy_enum(from_origin = option_into)]
        position: Option<Position>,
        primary: bool,
        kind: PointerKind,
    },
    MouseWheel {
        #[proxy_enum(from_origin = option_into)]
        device_id: Option<DeviceId>,
        delta: MouseScrollDelta,
        phase: TouchPhase,
    },
    PointerButton {
        #[proxy_enum(from_origin = option_into)]
        device_id: Option<DeviceId>,
        state: ElementState,
        position: Position,
        primary: bool,
        button: ButtonSource,
        is_macos_activation_click: bool,
    },
    HoldGesture {
        #[proxy_enum(from_origin = option_into)]
        device_id: Option<DeviceId>,
        phase: TouchPhase,
    },
    PinchGesture {
        #[proxy_enum(from_origin = option_into)]
        device_id: Option<DeviceId>,
        delta: f64,
        phase: TouchPhase,
    },
    PanGesture {
        #[proxy_enum(from_origin = option_into)]
        device_id: Option<DeviceId>,
        delta: Position,
        phase: TouchPhase,
    },
    DoubleTapGesture {
        #[proxy_enum(from_origin = option_into)]
        device_id: Option<DeviceId>,
    },
    RotationGesture {
        #[proxy_enum(from_origin = option_into)]
        device_id: Option<DeviceId>,
        delta: f32,
        phase: TouchPhase,
    },
    TouchpadPressure {
        #[proxy_enum(from_origin = option_into)]
        device_id: Option<DeviceId>,
        pressure: f32,
        stage: i64,
    },
    ScaleFactorChanged {
        scale_factor: f64,
        surface_size_writer: SurfaceSizeWriter,
    },
    ThemeChanged(#[proxy_enum(field_name = theme)] Theme),
    Occluded(#[proxy_enum(field_name = occluded)] bool),
    RedrawRequested,
}

/**[winit::event::DeviceId]*/
#[proxy_wrap(origin_type = winit::event::DeviceId)]
pub struct DeviceId;

/**[winit::event::FingerId]*/
#[proxy_wrap(origin_type = winit::event::FingerId)]
pub struct FingerId;

/**[winit::data_transfer::DataTransferId]*/
#[proxy_wrap(origin_type = winit::data_transfer::DataTransferId)]
pub struct DataTransferId;

/**[winit::event_loop::DndAction]*/
#[proxy_enum(origin_type = winit::event_loop::DndAction, string_enum, non_exhaustive)]
pub enum DndAction {
    Move,
    Copy,
    Link,
    Ask,
    Private,
}

/**[winit::event::PointerKind]*/
#[proxy_enum(origin_type = winit::event::PointerKind, skip_backward, non_exhaustive)]
pub enum PointerKind {
    Mouse,
    Touch(#[proxy_enum(field_name = finger_id)] FingerId),
    TabletTool(#[proxy_enum(field_name = kind)] TabletToolKind),
    Unknown,
}

/**[winit::event::PointerSource]*/
#[proxy_enum(origin_type = winit::event::PointerSource, skip_backward, non_exhaustive)]
pub enum PointerSource {
    Mouse,
    Touch {
        finger_id: FingerId,
        #[proxy_enum(from_origin = option_into)]
        force: Option<Force>,
    },
    TabletTool {
        kind: TabletToolKind,
        data: TabletToolData,
    },
    Unknown,
}

/**[winit::event::ButtonSource]*/
#[proxy_enum(origin_type = winit::event::ButtonSource, skip_backward, non_exhaustive)]
pub enum ButtonSource {
    Mouse(#[proxy_enum(field_name = button)] MouseButton),
    Touch {
        finger_id: FingerId,
        #[proxy_enum(from_origin = option_into)]
        force: Option<Force>,
    },
    TabletTool {
        kind: TabletToolKind,
        button: TabletToolButton,
        data: TabletToolData,
    },
    Unknown(#[proxy_enum(field_name = code)] u16),
}

/**[winit::event::TabletToolKind]*/
#[proxy_enum(origin_type = winit::event::TabletToolKind, string_enum, non_exhaustive)]
pub enum TabletToolKind {
    Pen,
    Eraser,
    Brush,
    Pencil,
    Airbrush,
    Finger,
    Mouse,
    Lens,
}

/**[winit::event::TabletToolButton]*/
#[proxy_enum(origin_type = winit::event::TabletToolButton, string_enum, non_exhaustive)]
pub enum TabletToolButton {
    Contact,
    Barrel,
}

/**[winit::event::TabletToolTilt]*/
#[proxy_struct(origin_type = winit::event::TabletToolTilt, object)]
pub struct TabletToolTilt {
    pub x: i8,
    pub y: i8,
}

/**[winit::event::TabletToolAngle]*/
#[proxy_struct(origin_type = winit::event::TabletToolAngle, object)]
pub struct TabletToolAngle {
    pub altitude: f64,
    pub azimuth: f64,
}

/**[winit::event::TabletToolData]*/
#[proxy_struct(origin_type = winit::event::TabletToolData, object, skip_from_js)]
pub struct TabletToolData {
    #[proxy_struct(from_origin = option_into, into_origin = option_into)]
    pub force: Option<Force>,
    #[proxy_struct(from_origin = option_into, into_origin = option_into)]
    pub tangential_force: Option<f32>,
    #[proxy_struct(from_origin = option_into, into_origin = option_into)]
    pub twist: Option<u16>,
    #[proxy_struct(from_origin = option_into, into_origin = option_into)]
    pub tilt: Option<TabletToolTilt>,
    #[proxy_struct(from_origin = option_into, into_origin = option_into)]
    pub angle: Option<TabletToolAngle>,
}

/**[winit::event::RawKeyEvent]*/
#[proxy_wrap(origin_type = winit::event::RawKeyEvent, skip_into_origin, no_setter)]
pub struct RawKeyEvent {
    pub physical_key: PhysicalKey,
    pub state: ElementState,
}

/** [winit::event::KeyEvent] */
#[proxy_wrap(origin_type = winit::event::KeyEvent, skip_into_origin, no_setter)]
pub struct KeyEvent {
    pub physical_key: PhysicalKey,

    #[proxy_wrap(get_ref, conv_get = [Clone::clone, Into::into])]
    pub logical_key: Key,

    #[proxy_wrap(get_ref, conv_get = [Clone::clone, option_into])]
    pub text: Option<String>,

    pub location: KeyLocation,

    pub state: ElementState,

    pub repeat: bool,
}

/** [winit::event::Modifiers] */
#[proxy_wrap(origin_type = winit::event::Modifiers)]
pub struct Modifiers;

#[napi]
impl Modifiers {
    #[napi(getter)]
    pub fn state(&self) -> ModifiersState {
        self.0.state().into()
    }
}

// winit's `Ime::Preedit(String, Option<(usize, usize)>)` carries the cursor begin/end offsets
// as **byte indices into the preedit string** (UTF-8), not screen coordinates. `Position` would
// wrongly present them as physical/logical pixels, so they get their own data carrier.
#[napi(object)]
pub struct PreeditCursorPosition {
    pub begin: u32,
    pub end: u32,
}

/**[winit::event::Ime]*/
#[proxy_enum(origin_type = winit::event::Ime, skip_backward, non_exhaustive)]
pub enum Ime {
    Enabled,
    Preedit(
        #[proxy_enum(field_name = preedit)] String,
        #[proxy_enum(
            field_name = cursor,
            from_origin = option_map(|(begin, end)| PreeditCursorPosition { begin: begin as u32, end: end as u32 })
        )]
        Option<PreeditCursorPosition>,
    ),
    Commit(#[proxy_enum(field_name = commit)] String),
    Disabled,
}

/**[winit::event::MouseButton]*/
#[proxy_enum(origin_type = winit::event::MouseButton, string_enum, skip_backward)]
pub enum MouseButton {
    /// The primary (usually left) button
    Left = 0,
    /// The secondary (usually right) button
    Right = 1,
    /// The tertiary (usually middle) button
    Middle = 2,
    /// The first side button, frequently assigned a back function
    Back = 3,
    /// The second side button, frequently assigned a forward function
    Forward = 4,
    /// The sixth button
    Button6 = 5,
    /// The seventh button
    Button7 = 6,
    /// The eighth button
    Button8 = 7,
    /// The ninth button
    Button9 = 8,
    /// The tenth button
    Button10 = 9,
    /// The eleventh button
    Button11 = 10,
    /// The twelfth button
    Button12 = 11,
    /// The thirteenth button
    Button13 = 12,
    /// The fourteenth button
    Button14 = 13,
    /// The fifteenth button
    Button15 = 14,
    /// The sixteenth button
    Button16 = 15,
    Button17 = 16,
    Button18 = 17,
    Button19 = 18,
    Button20 = 19,
    Button21 = 20,
    Button22 = 21,
    Button23 = 22,
    Button24 = 23,
    Button25 = 24,
    Button26 = 25,
    Button27 = 26,
    Button28 = 27,
    Button29 = 28,
    Button30 = 29,
    Button31 = 30,
    Button32 = 31,
}

#[proxy_enum(origin_type = winit::event::MouseScrollDelta, skip_backward, non_exhaustive)]
pub enum MouseScrollDelta {
    LineDelta(
        #[proxy_enum(field_name = x)] f64,
        #[proxy_enum(field_name = y)] f64,
    ),
    PixelDelta(#[proxy_enum(field_name = delta)] Position),
}

/**[winit::event::SurfaceSizeWriter]*/
#[proxy_wrap(origin_type = winit::event::SurfaceSizeWriter)]
pub struct SurfaceSizeWriter;

#[proxy_enum(origin_type = winit::event::TouchPhase, string_enum, skip_backward)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    Cancelled,
}

#[proxy_enum(origin_type = winit::event::Force)]
pub enum Force {
    /// On iOS, the force is calibrated so that the same number corresponds to
    /// roughly the same amount of pressure on the screen regardless of the
    /// device.
    Calibrated {
        /// The force of the touch, where a value of 1.0 represents the force of
        /// an average touch (predetermined by the system, not user-specific).
        ///
        /// The force reported by Apple Pencil is measured along the axis of the
        /// pencil. If you want a force perpendicular to the device, you need to
        /// calculate this value using the `altitude_angle` value.
        force: f64,
        /// The maximum possible force for a touch.
        ///
        /// The value of this field is sufficiently high to provide a wide
        /// dynamic range for values of the `force` field.
        max_possible_force: f64,
    },
    /// If the platform reports the force as normalized, we have no way of
    /// knowing how much pressure 1.0 corresponds to – we know it's the maximum
    /// amount of force, but as to how much force, you might either have to
    /// press really really hard, or not hard at all, depending on the device.
    Normalized(#[proxy_enum(field_name = value)] f64),
}

/**[winit::event::DeviceEvent]*/
#[proxy_enum(origin_type = winit::event::DeviceEvent, skip_backward, non_exhaustive)]
pub enum DeviceEvent {
    PointerMotion { delta: Position },
    MouseWheel { delta: MouseScrollDelta },
    Button { button: u32, state: ElementState },
    Key(#[proxy_enum(field_name = raw)] RawKeyEvent),
}

#[proxy_enum(origin_type = winit::event::ElementState, string_enum, skip_backward)]
#[derive(Clone)]
pub enum ElementState {
    Pressed,
    Released,
}
