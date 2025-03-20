use winit::monitor::{
    MonitorHandle as OriginMonitorHandle,
    VideoModeHandle as OriginVideoModeHandle,
};

use crate::{mark_ex_into, wrap_struct};

wrap_struct!(#[derive(Clone)] struct VideoModeHandle(OriginVideoModeHandle));
wrap_struct!(#[derive(Clone)] struct MonitorHandle(OriginMonitorHandle));

mark_ex_into!(
    OriginVideoModeHandle,
    OriginMonitorHandle,
    VideoModeHandle,
    MonitorHandle
);