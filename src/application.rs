use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, StartCause, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

pub struct Application {
    pub new_events: Option<fn(&mut Application, event_loop: &ActiveEventLoop, cause: StartCause)>,
    pub resumed: Option<fn(&mut Application, event_loop: &ActiveEventLoop)>,
    pub user_event: Option<fn(&mut Application, event_loop: &ActiveEventLoop, event: ())>,
    pub window_event: Option<fn(&mut Application, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent)>,
    pub device_event: Option<fn(&mut Application, event_loop: &ActiveEventLoop, device_id: DeviceId, event: DeviceEvent)>,
    pub about_to_wait: Option<fn(&mut Application, event_loop: &ActiveEventLoop)>,
    pub suspended: Option<fn(&mut Application, event_loop: &ActiveEventLoop)>,
    pub exiting: Option<fn(&mut Application, event_loop: &ActiveEventLoop)>,
    pub memory_warning: Option<fn(&mut Application, event_loop: &ActiveEventLoop)>,
}

impl ApplicationHandler for Application {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        todo!()
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        todo!()
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: ()) {
        todo!()
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        todo!()
    }

    fn device_event(&mut self, event_loop: &ActiveEventLoop, device_id: DeviceId, event: DeviceEvent) {
        todo!()
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        todo!()
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        todo!()
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        todo!()
    }

    fn memory_warning(&mut self, event_loop: &ActiveEventLoop) {
        todo!()
    }
}