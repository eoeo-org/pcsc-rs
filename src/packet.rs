use serde::{self, Deserialize, Serialize};

use crate::status::SystemStatus;

#[derive(Serialize, Deserialize)]
#[serde(tag = "event_type", content = "data")]
pub enum PacketData {
    #[serde(rename = "SYNC")]
    Sync(Option<SystemStatus>),
    #[serde(rename = "STATUS")]
    Status(String),
}
