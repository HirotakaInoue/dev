use axum::{
    extract::{rejection::JsonRejection, Json},
    http::StatusCode,
    response::IntoResponse,
};
use jsonwebtoken::{encode, errors, EncodingKey, Header};
use serde::{Deserialize, Serialize};

pub struct User {
    id: String,
    password: String,
}

impl User {
    fn new(id: &str, password: &str) -> Self {
        User {
            id: id.to_string(),
            password: password.to_string(),
        }
    }
}

#[derive(Deserialize)]
pub struct AuthRequest {
    id: String,
    password: String,
}

pub async fn login(payload: Result<Json<AuthRequest>, JsonRejection>) -> impl IntoResponse {
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

    // ダミーデータベースのユーザー
    let dummy_user = User::new("admin", "admin");

    // ユーザー名とパスワードの確認
    if payload.id == dummy_user.id && payload.password == dummy_user.password {
        // JWTトークン生成
        match generate_token(&dummy_user.id).await {
            Ok(token) => return (StatusCode::OK, Json(serde_json::json!({ "token": token }))),
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({ "error": "Failed to generate token" })),
                )
            }
        }
    }

    // 認証失敗
    (
        StatusCode::UNAUTHORIZED,
        Json(serde_json::json!({ "error": "Invalid credentials" })),
    )
}

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

async fn generate_token(username: &str) -> errors::Result<String> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(1))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration,
    };

    let secret_key = "dummy_secret";
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
}
