use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "event_type", content = "data")]
pub enum PacketData {
    #[serde(rename = "AUTH")]
    Auth(String),
    #[serde(rename = "SYNC")]
    Sync(String),
    #[serde(rename = "STATUS")]
    Status(String)
}
