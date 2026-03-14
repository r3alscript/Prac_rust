use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use reqwest::Client;
use serde::Deserialize;

use crate::{
    auth::jwt::create_jwt,
    config::AppState,
    infrastructure::user_repository::UserRepository,
};

#[derive(Deserialize)]
pub struct AuthRequest {
    pub code: String,
}

#[derive(Deserialize)]
struct GoogleTokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct GoogleUserInfo {
    email: String,
    name: String,
    picture: Option<String>,
}

pub async fn google_login() -> Redirect {
    let client_id = std::env::var("GOOGLE_CLIENT_ID").unwrap();
    let redirect_uri = std::env::var("GOOGLE_REDIRECT_URI").unwrap();

    let url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope=openid%20email%20profile",
        client_id, redirect_uri
    );

    Redirect::temporary(&url)
}

pub async fn google_callback(
    State(state): State<AppState>,
    Query(params): Query<AuthRequest>,
) -> Response {
    match handle_google_callback(state, params).await {
        Ok(redirect) => redirect.into_response(),
        Err(message) => (StatusCode::INTERNAL_SERVER_ERROR, message).into_response(),
    }
}

async fn handle_google_callback(
    state: AppState,
    params: AuthRequest,
) -> Result<Redirect, String> {
    let client_id = std::env::var("GOOGLE_CLIENT_ID").map_err(|e| e.to_string())?;
    let client_secret = std::env::var("GOOGLE_CLIENT_SECRET").map_err(|e| e.to_string())?;
    let redirect_uri = std::env::var("GOOGLE_REDIRECT_URI").map_err(|e| e.to_string())?;
    let frontend_url = std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5174".to_string());

    let client = Client::new();

    let token_res = client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("code", params.code.as_str()),
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("redirect_uri", redirect_uri.as_str()),
            ("grant_type", "authorization_code"),
        ])
        .send()
        .await
        .map_err(|e| format!("Failed to fetch Google token: {e}"))?;

    if !token_res.status().is_success() {
        let text = token_res.text().await.unwrap_or_default();
        return Err(format!("Google token endpoint error: {text}"));
    }

    let token_json = token_res
        .json::<GoogleTokenResponse>()
        .await
        .map_err(|e| format!("Failed to parse Google token response: {e}"))?;

    let userinfo_res = client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(&token_json.access_token)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch Google user info: {e}"))?;

    if !userinfo_res.status().is_success() {
        let text = userinfo_res.text().await.unwrap_or_default();
        return Err(format!("Google userinfo endpoint error: {text}"));
    }

    let google_user = userinfo_res
        .json::<GoogleUserInfo>()
        .await
        .map_err(|e| format!("Failed to parse Google user info: {e}"))?;

    let repo = UserRepository::new(state.db.clone());

    let app_user = repo
        .find_or_create_google_user(
            &google_user.email,
            &google_user.name,
            google_user.picture.as_deref(),
        )
        .await
        .map_err(|e| format!("Database error: {e}"))?;

    let jwt = create_jwt(&app_user).map_err(|e| format!("JWT creation error: {e}"))?;

    let redirect_url = format!("{}/authorization?token={}", frontend_url, jwt);

    Ok(Redirect::temporary(&redirect_url))
}