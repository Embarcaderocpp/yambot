use serde::{Deserialize, Serialize};

use super::BotRequestError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotRequest {
    #[serde(default)]
    pub action: Option<String>,
    #[serde(default)]
    pub element_id: Option<String>,
    #[serde(default)]
    pub errors: Option<Vec<BotRequestError>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotRequestError {
    pub r#type: String, // "directive_not_supported" | "invalid_directive_data" | "client_error"
    #[serde(default)]
    pub directive_name: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
}