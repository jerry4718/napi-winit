use proc::proxy_wrap;

/**[winit::monitor::VideoModeHandle]*/
#[proxy_wrap(origin_type = winit::monitor::VideoModeHandle)]
pub struct VideoModeHandle;

/**[winit::monitor::MonitorHandle]*/
#[proxy_wrap(origin_type = winit::monitor::MonitorHandle)]
pub struct MonitorHandle;
