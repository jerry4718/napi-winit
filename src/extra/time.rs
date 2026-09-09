use crate::napi_reason;
use napi::bindgen_prelude::*;
use std::{
    sync::OnceLock,
    time::{Duration as StdDuration, Instant as StdInstant, SystemTime},
};

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

// JS boundary validation: reject negative, NaN, infinite values instead of silently
// saturating them to zero (which `as u64` would do).
pub(crate) fn try_std_duration(value: &Duration) -> Result<StdDuration> {
    (value.secs.is_finite() && value.secs >= 0.0)
        .then(|| {
            StdDuration::from_secs(value.secs as u64) + StdDuration::from_nanos(value.nanos as u64)
        })
        .ok_or_else(|| napi_reason!("duration must be a non-negative finite number of seconds"))
}

pub(crate) fn try_std_instant(value: &Instant) -> Result<StdInstant> {
    (value.secs.is_finite() && value.secs >= 0.0)
        .then(|| StdDuration::from_secs(value.secs as u64) + StdDuration::from_nanos(value.nanos as u64))
        .and_then(|duration| anchor().instant.checked_add(duration))
        .ok_or_else(|| napi_reason!("instant must be a non-negative finite number of seconds within the monotonic clock range"))
}

fn validate_non_negative_f64(value: f64) -> Result<()> {
    (value.is_finite() && value >= 0.0)
        .then_some(())
        .ok_or_else(|| napi_reason!("duration must be a non-negative finite number"))
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
    pub fn from_secs(secs: f64) -> Result<Duration> {
        validate_non_negative_f64(secs)?;
        let i = secs.trunc();
        let f = secs.fract();
        Ok(Duration::from(
            StdDuration::from_secs(i as u64)
                + StdDuration::from_nanos((f * NANOS_PER_SEC).round() as u64),
        ))
    }

    #[napi]
    pub fn from_millis(millis: f64) -> Result<Duration> {
        validate_non_negative_f64(millis)?;
        let i = millis.trunc();
        let f = millis.fract();
        Ok(Duration::from(
            StdDuration::from_millis(i as u64)
                + StdDuration::from_nanos((f * NANOS_PER_MILLI).round() as u64),
        ))
    }

    #[napi]
    pub fn from_micros(micros: f64) -> Result<Duration> {
        validate_non_negative_f64(micros)?;
        let i = micros.trunc();
        let f = micros.fract();
        Ok(Duration::from(
            StdDuration::from_micros(i as u64)
                + StdDuration::from_nanos((f * NANOS_PER_MICRO).round() as u64),
        ))
    }

    #[napi]
    pub fn from_nanos(nanos: f64) -> Result<Duration> {
        validate_non_negative_f64(nanos)?;
        Ok(Duration::from(
            StdDuration::from_nanos(nanos.round() as u64),
        ))
    }

    #[napi]
    pub fn add(base: Duration, other: Duration) -> Result<Duration> {
        let base = try_std_duration(&base)?;
        let other = try_std_duration(&other)?;
        base.checked_add(other)
            .map(Duration::from)
            .ok_or_else(|| napi_reason!("overflow when adding durations"))
    }

    #[napi]
    pub fn sub(base: Duration, other: Duration) -> Result<Duration> {
        let base = try_std_duration(&base)?;
        let other = try_std_duration(&other)?;
        base.checked_sub(other)
            .map(Duration::from)
            .ok_or_else(|| napi_reason!("overflow when subtracting durations"))
    }

    #[napi]
    pub fn mul(base: Duration, other: f64) -> Result<Duration> {
        let base = try_std_duration(&base)?;
        // try_from_secs_f64 rejects negative, NaN and overflow, so no std panic path remains.
        StdDuration::try_from_secs_f64(base.as_secs_f64() * other)
            .map(Duration::from)
            .map_err(|e| napi_reason!("invalid duration multiplication: {e}"))
    }

    #[napi]
    pub fn div(base: Duration, other: f64) -> Result<Duration> {
        let base = try_std_duration(&base)?;
        // A zero, negative, non-finite divisor or an overflowing quotient is rejected here.
        StdDuration::try_from_secs_f64(base.as_secs_f64() / other)
            .map(Duration::from)
            .map_err(|e| napi_reason!("invalid duration division: {e}"))
    }

    impl From<StdDuration> for Duration {
        fn from(value: StdDuration) -> Self {
            Duration {
                secs: value.as_secs() as f64,
                nanos: value.subsec_nanos(),
            }
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
        Instant {
            secs: duration.as_secs() as f64,
            nanos: duration.subsec_nanos(),
        }
    }

    #[napi]
    pub fn after_secs(secs: f64) -> Result<Instant> {
        duration::from_secs(secs).and_then(|duration| add(now(), duration))
    }

    #[napi]
    pub fn after_millis(millis: f64) -> Result<Instant> {
        duration::from_millis(millis).and_then(|duration| add(now(), duration))
    }

    #[napi]
    pub fn after_micros(micros: f64) -> Result<Instant> {
        duration::from_micros(micros).and_then(|duration| add(now(), duration))
    }

    #[napi]
    pub fn after_nanos(nanos: f64) -> Result<Instant> {
        duration::from_nanos(nanos).and_then(|duration| add(now(), duration))
    }

    #[napi]
    pub fn add(base: Instant, other: Duration) -> Result<Instant> {
        let base = try_std_instant(&base)?;
        let other = try_std_duration(&other)?;
        base.checked_add(other)
            .map(Instant::from)
            .ok_or_else(|| napi_reason!("overflow when adding duration to instant"))
    }

    #[napi]
    pub fn sub(base: Instant, other: Duration) -> Result<Instant> {
        let base = try_std_instant(&base)?;
        let other = try_std_duration(&other)?;
        base.checked_sub(other)
            .map(Instant::from)
            .ok_or_else(|| napi_reason!("overflow when subtracting duration from instant"))
    }

    #[napi]
    pub fn duration_since(base: Instant, other: Instant) -> Result<Duration> {
        let base = try_std_instant(&base)?;
        let other = try_std_instant(&other)?;
        base.checked_duration_since(other)
            .map(Duration::from)
            .ok_or_else(|| {
                napi_reason!("the subtracted instant is not earlier than the base instant")
            })
    }

    impl From<StdInstant> for Instant {
        fn from(value: StdInstant) -> Self {
            let duration = value.duration_since(anchor().instant);
            Instant {
                secs: duration.as_secs() as f64,
                nanos: duration.subsec_nanos(),
            }
        }
    }

    impl From<Instant> for StdInstant {
        fn from(value: Instant) -> Self {
            // Infallible by contract: every JS boundary validates its value through
            // `try_std_instant` first, so an out-of-range `secs` never reaches this path.
            // The macro-generated `Into<origin>` conversions depend on this impl staying
            // infallible.
            let duration = StdDuration::from_secs(value.secs as u64)
                + StdDuration::from_nanos(value.nanos as u64);
            anchor().instant + duration
        }
    }

    impl From<&Instant> for StdInstant {
        fn from(value: &Instant) -> Self {
            StdInstant::from(value.clone())
        }
    }
}
