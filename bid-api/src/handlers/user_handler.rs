use axum::{
    extract::State,
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};

use crate::{
    auth::jwt::decode_jwt,
    config::AppState,
    infrastructure::user_repository::UserRepository,
};

pub async fn me_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Response {
    match handle_me(state, headers).await {
        Ok(response) => Json(response).into_response(),
        Err((status, message)) => (status, message).into_response(),
    }
}

async fn handle_me(
    state: AppState,
    headers: HeaderMap,
) -> Result<crate::domain::user::User, (StatusCode, String)> {
    let auth_header = headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid Authorization format".to_string()))?;

    let claims = decode_jwt(token)
        .map_err(|e| (StatusCode::UNAUTHORIZED, format!("Invalid token: {e}")))?;

    let user_id = claims
        .user_id()
        .map_err(|e| (StatusCode::UNAUTHORIZED, format!("Invalid user id in token: {e}")))?;

    let repo = UserRepository::new(state.db.clone());

    let user = repo
        .find_by_id(user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {e}")))?;

    user.ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))
}