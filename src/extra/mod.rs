pub mod convert;
mod manual_convert;
// pub mod iter;

use std::time::Instant;
use crate::mark_ex_into;

#[napi(js_name = "TimeDuration")]
#[derive(Clone)]
pub struct TimeDuration {
    #[napi(js_name = "t_secs")]
    pub t_secs: u32,
    #[napi(js_name = "t_nanos")]
    pub t_nanos: u32,
}

impl From<Instant> for TimeDuration {
    fn from(t: Instant) -> Self {
        let duration = t.elapsed();
        let t_secs = duration.as_secs() as u32;
        let t_nanos = duration.subsec_nanos();

        Self { t_secs, t_nanos }
    }
}

mark_ex_into!(Instant, TimeDuration);