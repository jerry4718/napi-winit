// use winit::event::DeviceId;
// use proc::enum_dsl;
// use proc::struct_dsl;
// use crate::window::WindowId;
//
// pub struct UserPayload {}
//
// enum_dsl!(
//     enum Event {
//         NewEvents(StartCause),
//         WindowEvent {
//             window_id: WindowId,
//             event: WindowEvent,
//         },
//         DeviceEvent {
//             device_id: DeviceId,
//             event: DeviceEvent,
//         },
//         UserEvent(UserPayload),
//         Suspended,
//         Resumed,
//         AboutToWait,
//         LoopExiting,
//         MemoryWarning,
//     }
// );
//
// enum_dsl!(
//     enum StartCause {
//         ResumeTimeReached {
//             start: Instant,
//             requested_resume: Instant,
//         },
//         WaitCancelled {
//             start: Instant,
//             requested_resume: Option<Instant>,
//         },
//         Poll,
//         Init,
//     }
// );
//
// enum_dsl!(
//     enum WindowEvent {
//         ActivationTokenDone {
//             serial: AsyncRequestSerial,
//             token: ActivationToken,
//         },
//         Resized(PhysicalSize<u32>),
//         Moved(PhysicalPosition<i32>),
//         CloseRequested,
//         Destroyed,
//         DroppedFile(PathBuf),
//         HoveredFile(PathBuf),
//         HoveredFileCancelled,
//         Focused(bool),
//         KeyboardInput {
//             device_id: DeviceId,
//             event: KeyEvent,
//             is_synthetic: bool,
//         },
//         ModifiersChanged(Modifiers),
//         Ime(Ime),
//         CursorMoved {
//             device_id: DeviceId,
//             position: PhysicalPosition<f64>,
//         },
//         CursorEntered {
//             device_id: DeviceId,
//         },
//         CursorLeft {
//             device_id: DeviceId,
//         },
//         MouseWheel {
//             device_id: DeviceId,
//             delta: MouseScrollDelta,
//             phase: TouchPhase,
//         },
//         MouseInput {
//             device_id: DeviceId,
//             state: ElementState,
//             button: MouseButton,
//         },
//         PinchGesture {
//             device_id: DeviceId,
//             delta: f64,
//             phase: TouchPhase,
//         },
//         PanGesture {
//             device_id: DeviceId,
//             delta: PhysicalPosition<f32>,
//             phase: TouchPhase,
//         },
//         DoubleTapGesture {
//             device_id: DeviceId,
//         },
//         RotationGesture {
//             device_id: DeviceId,
//             delta: f32,
//             phase: TouchPhase,
//         },
//         TouchpadPressure {
//             device_id: DeviceId,
//             pressure: f32,
//             stage: i64,
//         },
//         AxisMotion {
//             device_id: DeviceId,
//             axis: AxisId,
//             value: f64,
//         },
//         Touch(Touch),
//         ScaleFactorChanged {
//             scale_factor: f64,
//             inner_size_writer: InnerSizeWriter,
//         },
//         ThemeChanged(Theme),
//         Occluded(bool),
//         RedrawRequested,
//     }
// );
//
// enum_dsl!(
//     enum DeviceEvent {
//         Added,
//         Removed,
//         MouseMotion {
//             delta: (f64, f64),
//         },
//         MouseWheel {
//             delta: MouseScrollDelta,
//         },
//         Motion {
//             axis: AxisId,
//             value: f64,
//         },
//         Button {
//             button: ButtonId,
//             state: ElementState,
//         },
//         Key(RawKeyEvent),
//     }
// );