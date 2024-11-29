use crate::models::{ApiKey, LLMModel};
use axum::{
    extract::State,
    extract::{rejection::JsonRejection, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct SetApiKeyRequest {
    user_id: String,
    llm_type: String,
    key: String,
}

pub async fn set_api_key(
    State(extension): State<PgPool>,
    payload: Result<Json<SetApiKeyRequest>, JsonRejection>,
) -> impl IntoResponse {
    match &payload {
        Ok(_payload) => (),
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": "Invalid payload" })),
            )
        }
    }
    let payload = payload.unwrap().0;

    let llm_model = match LLMModel::from_str(&payload.llm_type) {
        Ok(model_type) => model_type,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!(e.to_string())),
            )
        }
    };

    let api_key = ApiKey::new(&payload.user_id, &payload.key, llm_model);

    sqlx::query!(
        "INSERT INTO api_keys (user_id, key, llm_model) VALUES ($1, $2, $3)",
        api_key.user_id,
        api_key.key,
        api_key.model as LLMModel
    )
    .execute(&extension)
    .await
    .unwrap();

    (
        StatusCode::OK,
        Json(serde_json::json!({ "message": "API key set" })),
    )
}
