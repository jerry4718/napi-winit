use std::time::Instant;

#[napi(js_name = "TimeDuration")]
pub struct JsTimeDuration {
    pub t_secs: u32,
    pub t_nanos: u32,
}

impl From<Instant> for JsTimeDuration {
    fn from(t: Instant) -> Self {
        let duration = t.elapsed();
        let t_secs = duration.as_secs() as u32;
        let t_nanos = duration.subsec_nanos();

        Self { t_secs, t_nanos }
    }
}