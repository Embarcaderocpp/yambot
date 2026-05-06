use serde::{Deserialize, Serialize};

use super::InlineSuggestButton;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestButtons {
    #[serde(default)]
    pub one_time: Option<bool>,
    #[serde(default)]
    pub persistent: Option<bool>,
    pub buttons: Vec<Vec<InlineSuggestButton>>, // двумерный массив для строк кнопок
}