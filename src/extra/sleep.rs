#[napi(js_name = "Extra")]
pub mod namespace {
    use crate::extra::time::{Duration, try_std_duration};
    use napi::bindgen_prelude::*;

    #[napi]
    pub async fn tokio_sleep(duration: Duration) -> Result<()> {
        tokio::time::sleep(try_std_duration(&duration)?).await;
        Ok(())
    }
}
