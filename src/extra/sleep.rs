#[napi(js_name = "Extra")]
pub mod namespace {
    use napi::bindgen_prelude::*;
    use crate::extra::time::{try_std_duration, Duration};

    #[napi]
    pub async fn tokio_sleep(duration: Duration) -> Result<()> {
        tokio::time::sleep(try_std_duration(&duration)?).await;
        Ok(())
    }
}