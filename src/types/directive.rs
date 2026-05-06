use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Directive {
    #[serde(rename = "open_uri")]
    OpenUri { url: String },

    #[serde(rename = "send_message")]
    SendMessage {
        text: String,
        #[serde(default)]
        payload: Option<String>,
    },

    #[serde(rename = "server_action")]
    ServerAction {
        action: String,
        payload: String,
    },

    #[serde(rename = "set_elements_state")]
    SetElementsState {
        element_ids: Vec<String>,
        state: String, // "disabled" | "enabled"
        #[serde(default)]
        duration: Option<i64>,
    },
}