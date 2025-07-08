use winit::monitor::{
    MonitorHandle as OriginMonitorHandle,
    VideoModeHandle as OriginVideoModeHandle,
};

use crate::wrap_struct;

wrap_struct!(#[derive(Clone)] struct VideoModeHandle(OriginVideoModeHandle));
wrap_struct!(#[derive(Clone)] struct MonitorHandle(OriginMonitorHandle));