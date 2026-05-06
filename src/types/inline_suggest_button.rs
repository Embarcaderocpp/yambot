use serde::{Deserialize, Serialize};

use super::Directive;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineSuggestButton {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub directives: Option<Vec<Directive>>,
}