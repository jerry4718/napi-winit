use proc::proxy_wrap;

#[proxy_wrap(origin_type = winit::monitor::VideoModeHandle)]
pub struct VideoModeHandle;

#[proxy_wrap(origin_type = winit::monitor::MonitorHandle)]
pub struct MonitorHandle;
