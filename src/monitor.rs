use crate::{mark_ex_into, wrap_struct};

use winit::monitor::{
    VideoModeHandle as OriginVideoModeHandle,
    MonitorHandle as OriginMonitorHandle,
};

wrap_struct!(#[derive(Clone)] struct VideoModeHandle(OriginVideoModeHandle));
wrap_struct!(#[derive(Clone)] struct MonitorHandle(OriginMonitorHandle));

mark_ex_into!(
    OriginVideoModeHandle,
    OriginMonitorHandle,
    VideoModeHandle,
    MonitorHandle
);