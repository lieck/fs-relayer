use ceramic_box::event::Event;
use ceramic_box::Cid;
use ceramic_box::{StreamId, StreamState};
use int_enum::IntEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSlot {
    pub r#type: u64,
    pub dapp_id: uuid::Uuid,
    // pub network: String,
    pub genesis: Cid,
    pub tip: Cid,
    pub account: Option<String>,
    pub model: Option<StreamId>,
    #[serde(default = "content_default")]
    pub content: serde_json::Value,
}

fn content_default() -> serde_json::Value {
    serde_json::Value::Null
}

impl FileSlot {
    pub fn new(
        dapp_id: &uuid::Uuid,
        r#type: u64,
        genesis: &Event,
        model: Option<StreamId>,
    ) -> anyhow::Result<Self> {
        Ok(FileSlot {
            r#type,
            dapp_id: *dapp_id,
            tip: genesis.cid,
            genesis: genesis.cid,
            model,
            account: None,
            content: serde_json::Value::Null,
        })
    }

    pub fn stream_id(&self) -> anyhow::Result<StreamId> {
        Ok(StreamId {
            r#type: IntEnum::from_int(self.r#type)?,
            cid: self.genesis,
        })
    }

    pub async fn state(&self, commits: Vec<Event>) -> anyhow::Result<StreamState> {
        StreamState::make(self.r#type, commits).await
    }
}

#[async_trait]
pub trait FileSlotStore: Sync + Send {
    async fn save_stream(&self, stream: &FileSlot) -> anyhow::Result<()>;
    async fn load_stream(&self, stream_id: &StreamId) -> anyhow::Result<Option<FileSlot>>;
    async fn list_all_streams(&self) -> anyhow::Result<Vec<FileSlot>>;
}
