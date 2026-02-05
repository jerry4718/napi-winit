#[napi(js_name = "Extra")]
pub mod namespace {
    use napi::bindgen_prelude::*;
    use crate::extra::time::Duration;

    #[napi]
    pub async fn tokio_sleep(duration: Duration) {
        tokio::time::sleep(duration.into()).await;
    }
}