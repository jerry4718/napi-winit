use napi::bindgen_prelude::*;
use std::time::{Duration as StdDuration, Instant as StdInstant, SystemTime};
use crate::napi_reason;
use std::sync::OnceLock;

struct TimeAnchor {
    instant: StdInstant,
    system: SystemTime,
}

static TIME_ANCHOR: OnceLock<TimeAnchor> = OnceLock::new();

fn anchor() -> &'static TimeAnchor {
    TIME_ANCHOR.get_or_init(|| TimeAnchor {
        instant: StdInstant::now(),
        system: SystemTime::now(),
    })
}

fn instant_to_system(instant: StdInstant) -> SystemTime {
    let delta = instant.duration_since(anchor().instant);
    anchor().system + delta
}

fn system_to_instant(time: SystemTime) -> StdInstant {
    let delta = time
        .duration_since(anchor().system)
        .expect("system time before anchor");
    anchor().instant + delta
}

#[napi(object)]
#[derive(Clone)]
pub struct Duration {
    pub secs: f64,
    pub nanos: u32,
}

#[napi(js_name = "Duration")]
mod duration {
    use super::*;

    const NANOS_PER_SEC: f64 = 1_000_000_000.0;
    const NANOS_PER_MILLI: f64 = 1_000_000.0;
    const NANOS_PER_MICRO: f64 = 1_000.0;

    #[napi]
    pub fn from_secs(secs: f64) -> Duration {
        let i = secs.trunc();
        let f = secs.fract();
        Duration::from(
            StdDuration::from_secs(i as u64)
                + StdDuration::from_nanos((f * NANOS_PER_SEC).round() as u64),
        )
    }

    #[napi]
    pub fn from_millis(millis: f64) -> Duration {
        let i = millis.trunc();
        let f = millis.fract();
        Duration::from(
            StdDuration::from_millis(i as u64)
                + StdDuration::from_nanos((f * NANOS_PER_MILLI).round() as u64),
        )
    }

    #[napi]
    pub fn from_micros(micros: f64) -> Duration {
        let i = micros.trunc();
        let f = micros.fract();
        Duration::from(
            StdDuration::from_micros(i as u64)
                + StdDuration::from_nanos((f * NANOS_PER_MICRO).round() as u64),
        )
    }

    #[napi]
    pub fn from_nanos(nanos: f64) -> Duration {
        Duration::from(StdDuration::from_nanos(nanos.round() as u64))
    }

    #[napi]
    pub fn add(base: Duration, other: Duration) -> Result<Duration> {
        StdDuration::from(base).checked_add(StdDuration::from(other))
            .map(Duration::from)
            .ok_or_else(|| napi_reason!("overflow when adding durations"))
    }

    #[napi]
    pub fn sub(base: Duration, other: Duration) -> Result<Duration> {
        StdDuration::from(base).checked_sub(StdDuration::from(other))
            .map(Duration::from)
            .ok_or_else(|| napi_reason!("overflow when subtracting durations"))
    }

    #[napi]
    pub fn mul(base: Duration, other: f64) -> Duration {
        Duration::from(StdDuration::from(base).mul_f64(other))
    }

    #[napi]
    pub fn div(base: Duration, other: f64) -> Duration {
        Duration::from(StdDuration::from(base).div_f64(other))
    }

    impl From<StdDuration> for Duration {
        fn from(value: StdDuration) -> Self {
            Duration { secs: value.as_secs() as f64, nanos: value.subsec_nanos() }
        }
    }

    impl From<Duration> for StdDuration {
        fn from(value: Duration) -> Self {
            StdDuration::from_secs(value.secs as u64) + StdDuration::from_nanos(value.nanos as u64)
        }
    }

    impl From<&Duration> for StdDuration {
        fn from(value: &Duration) -> Self {
            StdDuration::from(value.clone())
        }
    }
}

#[napi(object)]
#[derive(Clone)]
pub struct Instant {
    pub secs: f64,
    pub nanos: u32,
}

#[napi(js_name = "Instant")]
mod instant {
    use super::*;

    #[napi]
    pub fn now() -> Instant {
        let duration = StdInstant::now().duration_since(anchor().instant);
        Instant { secs: duration.as_secs() as f64, nanos: duration.subsec_nanos() }
    }

    #[napi]
    pub fn after_secs(secs: f64) -> Result<Instant> {
        add(now(), duration::from_secs(secs))
    }

    #[napi]
    pub fn after_millis(millis: f64) -> Result<Instant> {
        add(now(), duration::from_millis(millis))
    }

    #[napi]
    pub fn after_micros(micros: f64) -> Result<Instant> {
        add(now(), duration::from_micros(micros))
    }

    #[napi]
    pub fn after_nanos(nanos: f64) -> Result<Instant> {
        add(now(), duration::from_nanos(nanos))
    }

    #[napi]
    pub fn add(base: Instant, other: Duration) -> Result<Instant> {
        StdInstant::from(base)
            .checked_add(StdDuration::from(other))
            .map(Instant::from)
            .ok_or_else(|| napi_reason!("overflow when adding duration to instant"))
    }

    #[napi]
    pub fn sub(base: Instant, other: Duration) -> Result<Instant> {
        StdInstant::from(base)
            .checked_sub(StdDuration::from(other))
            .map(Instant::from)
            .ok_or_else(|| napi_reason!("overflow when subtracting duration from instant"))
    }

    impl From<StdInstant> for Instant {
        fn from(value: StdInstant) -> Self {
            let duration = value.duration_since(anchor().instant);
            Instant { secs: duration.as_secs() as f64, nanos: duration.subsec_nanos() }
        }
    }

    impl From<Instant> for StdInstant {
        fn from(value: Instant) -> Self {
            let duration = StdDuration::from_secs(value.secs as u64) + StdDuration::from_nanos(value.nanos as u64);
            anchor().instant + duration
        }
    }

    impl From<&Instant> for StdInstant {
        fn from(value: &Instant) -> Self {
            StdInstant::from(value.clone())
        }
    }
}
