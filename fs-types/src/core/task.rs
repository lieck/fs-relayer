use ceramic_box::event::{Event, EventsUploader};
use ceramic_box::{http, Ceramic};
use fang::async_trait;
use fang::asynk::async_queue::AsyncQueueable;
use fang::serde::{Deserialize, Serialize};
use fang::typetag;
use fang::AsyncRunnable;
use fang::FangError;

use crate::core::stream::FileSlot;

#[derive(Serialize, Deserialize)]
#[serde(crate = "fang::serde")]
pub struct SyncStream {
    pub ceramic: Ceramic,
    pub stream: FileSlot,
    pub events: Vec<Event>,
}

#[async_trait]
#[typetag::serde]
impl AsyncRunnable for SyncStream {
    async fn run(&self, _queue: &mut dyn AsyncQueueable) -> Result<(), FangError> {
        let client = http::Client::new();
        let stream_id = match self.stream.stream_id() {
            Ok(stream_id) => stream_id,
            Err(err) => {
                log::error!("failed to get stream id: {}", err);
                return Ok(());
            }
        };
        let res = client
            .upload_events(&self.ceramic, &stream_id, self.events.to_vec())
            .await;
        if let Err(err) = res {
            log::error!("failed to upload events: {}", err);
        }
        Ok(())
    }
}
