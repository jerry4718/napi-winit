use crate::wrap_struct;

wrap_struct!(struct VideoModeHandle(winit::monitor::VideoModeHandle));
wrap_struct!(struct MonitorHandle(winit::monitor::MonitorHandle));