use anyhow::Result;
use serde::Deserialize;
use sqlx::Type;

#[derive(Deserialize, Debug, Type)]
#[sqlx(type_name = "llm_models", rename_all = "lowercase")]
pub enum LLMModel {
    #[serde(rename = "gpt4")]
    Gpt4,
    #[serde(alias = "gemini")]
    Gemini,
}

impl Default for LLMModel {
    fn default() -> Self {
        LLMModel::Gpt4
    }
}

impl LLMModel {
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "gpt4" => Ok(LLMModel::Gpt4),
            "gemini" => Ok(LLMModel::Gemini),
            _ => anyhow::bail!("Invalid LLMType: {}", s),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            LLMModel::Gpt4 => "gpt4".to_string(),
            LLMModel::Gemini => "gemini".to_string(),
        }
    }
}
