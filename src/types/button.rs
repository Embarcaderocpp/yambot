use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Button {
    pub text: String,
    #[serde(default)]
    pub payload: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}