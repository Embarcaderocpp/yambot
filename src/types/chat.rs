use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    pub r#type: String, // "private" | "group" | "channel"
    #[serde(default)]
    pub id: Option<String>,
}