use napi::bindgen_prelude::*;
use std::time::{Duration, Instant};

#[napi(object)]
pub struct Timeout {
    pub secs: BigInt,
    pub nanos: u32,
}

#[napi(js_name = "Timeout")]
pub mod timeout {
    use super::*;

    #[napi]
    pub fn from_millis(millis: f64) -> Timeout {
        Timeout::from(Duration::from_millis(millis as u64))
    }
    #[napi]
    pub fn from_micros(micros: f64) -> Timeout {
        Timeout::from(Duration::from_micros(micros as u64))
    }
    #[napi]
    pub fn from_nanos(nanos: f64) -> Timeout {
        Timeout::from(Duration::from_nanos(nanos as u64))
    }

    impl From<Duration> for Timeout {
        fn from(duration: Duration) -> Self {
            let secs = duration.as_secs();
            let nanos = duration.subsec_nanos();
            Self { secs: BigInt::from(secs), nanos }
        }
    }

    impl From<Timeout> for Duration {
        fn from(timeout: Timeout) -> Self {
            let (signed, secs, lossless) = timeout.secs.get_u64();
            if signed { panic!("get_u64 has signed") }
            if !lossless { panic!("get_u64 not lossless") }
            Duration::new(secs, timeout.nanos)
        }
    }

    impl From<Instant> for Timeout {
        fn from(instant: Instant) -> Self {
            Self::from(Instant::now() - instant)
        }
    }

    impl From<Timeout> for Instant {
        fn from(timeout: Timeout) -> Self {
            Instant::now() + Duration::from(timeout)
        }
    }
}