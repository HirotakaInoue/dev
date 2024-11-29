use crate::models::LLMModel;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApiKey {
    pub user_id: String,
    pub key: String,
    pub model: LLMModel,
}

impl ApiKey {
    pub fn new(user_id: &str, key: &str, model: LLMModel) -> Self {
        Self {
            user_id: user_id.to_string(),
            key: key.to_string(),
            model,
        }
    }
}
