use serde::{Deserialize, Serialize};

/// Запрос на получение обновлений (polling)
#[derive(Debug, Serialize, Default)]
pub struct GetUpdatesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Запрос на отправку текстового сообщения
#[derive(Debug, Serialize, Default)]
pub struct SendTextRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]   // ← ДОБАВЛЕНО
    pub user_id: Option<String>,                       // ← ДОБАВЛЕНО (по требованию API)
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_important: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_link_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggest_buttons: Option<crate::types::SuggestButtons>,
}