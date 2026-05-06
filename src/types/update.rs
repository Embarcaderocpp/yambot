use serde::{Deserialize, Serialize};

use super::{BotRequest, Chat, File, Image, Sender};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Update {
    #[serde(rename = "from")]          // ← ИСПРАВЛЕНО: в API поле называется "from"
    pub sender: Sender,
    pub chat: Chat,
    #[serde(default)]
    pub text: Option<String>,
    pub timestamp: i64,
    pub message_id: i64,               // ← ИСПРАВЛЕНО: было String → i64
    pub update_id: i64,                // ← ИСПРАВЛЕНО: было String → i64
    #[serde(default)]
    pub file: Option<File>,
    #[serde(default)]
    pub image: Option<Image>,
    #[serde(default)]
    pub bot_request: Option<BotRequest>,
}