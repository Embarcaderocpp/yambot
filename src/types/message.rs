use serde::{Deserialize, Serialize};

use super::{BotRequest, Chat, File, Image, Sender};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub sender: Sender,
    pub chat: Chat,
    #[serde(default)]
    pub text: Option<String>,
    pub timestamp: i64,
    pub message_id: String,
    pub update_id: String,
    #[serde(default)]
    pub file: Option<File>,
    #[serde(default)]
    pub image: Option<Image>,
    #[serde(default)]
    pub bot_request: Option<BotRequest>,
}