use proc::proxy_struct;

#[proxy_struct(origin_type = winit::monitor::VideoModeHandle)]
pub struct VideoModeHandle;

#[proxy_struct(origin_type = winit::monitor::MonitorHandle)]
pub struct MonitorHandle;
