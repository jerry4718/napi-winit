use winit::application::ApplicationHandler as WApplicationHandler;
use winit::event::{
    DeviceEvent as WDeviceEvent,
    DeviceId as WDeviceId,
    StartCause as WStartCause,
    WindowEvent as WWindowEvent
};
use winit::event_loop::ActiveEventLoop as WActiveEventLoop;
use winit::window::WindowId as WWindowId;

pub struct Application {
    pub new_events: Option<fn(&mut Application, event_loop: &WActiveEventLoop, cause: WStartCause)>,
    pub resumed: Option<fn(&mut Application, event_loop: &WActiveEventLoop)>,
    pub user_event: Option<fn(&mut Application, event_loop: &WActiveEventLoop, event: ())>,
    pub window_event: Option<fn(&mut Application, event_loop: &WActiveEventLoop, window_id: WWindowId, event: WWindowEvent)>,
    pub device_event: Option<fn(&mut Application, event_loop: &WActiveEventLoop, device_id: WDeviceId, event: WDeviceEvent)>,
    pub about_to_wait: Option<fn(&mut Application, event_loop: &WActiveEventLoop)>,
    pub suspended: Option<fn(&mut Application, event_loop: &WActiveEventLoop)>,
    pub exiting: Option<fn(&mut Application, event_loop: &WActiveEventLoop)>,
    pub memory_warning: Option<fn(&mut Application, event_loop: &WActiveEventLoop)>,
}

impl WApplicationHandler for Application {
    fn new_events(&mut self, event_loop: &WActiveEventLoop, cause: WStartCause) {
        todo!()
    }

    fn resumed(&mut self, event_loop: &WActiveEventLoop) {
        todo!()
    }

    fn user_event(&mut self, event_loop: &WActiveEventLoop, event: ()) {
        todo!()
    }

    fn window_event(&mut self, event_loop: &WActiveEventLoop, window_id: WWindowId, event: WWindowEvent) {
        todo!()
    }

    fn device_event(&mut self, event_loop: &WActiveEventLoop, device_id: WDeviceId, event: WDeviceEvent) {
        todo!()
    }

    fn about_to_wait(&mut self, event_loop: &WActiveEventLoop) {
        todo!()
    }

    fn suspended(&mut self, event_loop: &WActiveEventLoop) {
        todo!()
    }

    fn exiting(&mut self, event_loop: &WActiveEventLoop) {
        todo!()
    }

    fn memory_warning(&mut self, event_loop: &WActiveEventLoop) {
        todo!()
    }
}