use std::{fs, path::Path as FsPath};

use axum::{
    extract::{Multipart, Path as AxumPath, State},
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    Json,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    application::{
        create_lot_use_case::CreateLotUseCase,
        get_lot_use_case::GetLotUseCase,
        get_lots_use_case::GetLotsUseCase,
    },
    auth::jwt::decode_jwt,
    config::AppState,
    infrastructure::lot_repository::LotRepository,
    presentation::{
        request_models::CreateBidRequest,
        response_models::{LotCardResponse, LotDetailsResponse},
    },
};

pub async fn get_lots_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<LotCardResponse>>, (StatusCode, String)> {
    let lots = GetLotsUseCase::execute(&state.db)
        .await
        .map_err(internal_error)?;

    let mut response = Vec::with_capacity(lots.len());

    for lot in lots {
        let current_price: f64 = LotRepository::current_price(&state.db, lot.id)
            .await
            .map_err(internal_error)?;

        let image_url: Option<String> = LotRepository::first_image_url(&state.db, lot.id)
            .await
            .map_err(internal_error)?;

        response.push(LotCardResponse {
            id: lot.id,
            title: lot.title,
            start_price: lot.start_price,
            current_price,
            image_url,
        });
    }

    Ok(Json(response))
}

pub async fn get_lot_by_id_handler(
    AxumPath(lot_id): AxumPath<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<LotDetailsResponse>, (StatusCode, String)> {
    let lot = GetLotUseCase::execute(&state.db, lot_id)
        .await
        .map_err(internal_error)?
        .ok_or((StatusCode::NOT_FOUND, "Lot not found".to_string()))?;

    let bids_count: i64 = LotRepository::count_bids(&state.db, lot_id)
        .await
        .map_err(internal_error)?;

    let max_bid: f64 = LotRepository::max_bid(&state.db, lot_id)
        .await
        .map_err(internal_error)?
        .unwrap_or(lot.start_price);

    let current_price: f64 = LotRepository::current_price(&state.db, lot_id)
        .await
        .map_err(internal_error)?;

    let image_url: Option<String> = LotRepository::first_image_url(&state.db, lot_id)
        .await
        .map_err(internal_error)?;

    Ok(Json(LotDetailsResponse {
        id: lot.id,
        title: lot.title,
        description: lot.description,
        start_price: lot.start_price,
        current_price,
        seller_id: lot.seller_id,
        image_url,
        auction_end: lot.end_at_utc,
        created_at: lot.created_at_utc,
        bids_count,
        max_bid,
    }))
}

pub async fn create_lot_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<LotCardResponse>), (StatusCode, String)> {
    let user_id = extract_user_id_from_headers(&headers)?;

    let mut title: Option<String> = None;
    let mut description: Option<String> = None;
    let mut start_price: Option<f64> = None;
    let mut auction_end: Option<DateTime<Utc>> = None;
    let mut image_url: Option<String> = None;

    loop {
        let next = multipart
            .next_field()
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Multipart error: {e}")))?;

        let Some(field) = next else {
            break;
        };

        let name = field.name().unwrap_or("").to_string();

        match name.as_str() {
            "title" => {
                let value = field
                    .text()
                    .await
                    .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid title: {e}")))?;
                title = Some(value);
            }
            "description" => {
                let value = field
                    .text()
                    .await
                    .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid description: {e}")))?;
                description = Some(value);
            }
            "start_price" => {
                let value = field
                    .text()
                    .await
                    .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid start_price: {e}")))?;

                let parsed: f64 = value.parse().map_err(|e| {
                    (
                        StatusCode::BAD_REQUEST,
                        format!("Invalid start_price number: {e}"),
                    )
                })?;

                start_price = Some(parsed);
            }
            "auction_end" => {
                let value = field
                    .text()
                    .await
                    .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid auction_end: {e}")))?;

                let parsed = DateTime::parse_from_rfc3339(&value).map_err(|e| {
                    (
                        StatusCode::BAD_REQUEST,
                        format!("Invalid auction_end format: {e}"),
                    )
                })?;

                auction_end = Some(parsed.with_timezone(&Utc));
            }
            "image" => {
                let original_name = field
                    .file_name()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "image.bin".to_string());

                let extension = FsPath::new(&original_name)
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("jpg");

                let bytes = field
                    .bytes()
                    .await
                    .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid image bytes: {e}")))?;

                fs::create_dir_all("uploads/lots").map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to create uploads directory: {e}"),
                    )
                })?;

                let saved_name = format!("{}.{}", Uuid::new_v4(), extension);
                let relative_url = format!("/uploads/lots/{}", saved_name);
                let full_path = format!("uploads/lots/{}", saved_name);

                fs::write(&full_path, &bytes).map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to save image: {e}"),
                    )
                })?;

                image_url = Some(relative_url);
            }
            _ => {}
        }
    }

    let title = title.ok_or((StatusCode::BAD_REQUEST, "title is required".to_string()))?;
    let description =
        description.ok_or((StatusCode::BAD_REQUEST, "description is required".to_string()))?;
    let start_price =
        start_price.ok_or((StatusCode::BAD_REQUEST, "start_price is required".to_string()))?;
    let auction_end =
        auction_end.ok_or((StatusCode::BAD_REQUEST, "auction_end is required".to_string()))?;

    if start_price <= 0.0 {
        return Err((StatusCode::BAD_REQUEST, "Start price must be > 0".to_string()));
    }

    let lot = CreateLotUseCase::execute(
        &state.db,
        user_id,
        title,
        description,
        start_price,
        auction_end,
        image_url,
    )
        .await
        .map_err(internal_error)?;

    let image_url: Option<String> = LotRepository::first_image_url(&state.db, lot.id)
        .await
        .map_err(internal_error)?;

    Ok((
        StatusCode::CREATED,
        Json(LotCardResponse {
            id: lot.id,
            title: lot.title,
            start_price: lot.start_price,
            current_price: lot.start_price,
            image_url,
        }),
    ))
}

pub async fn place_bid_handler(
    AxumPath(lot_id): AxumPath<Uuid>,
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateBidRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    if payload.amount <= 0.0 {
        return Err((StatusCode::BAD_REQUEST, "Bid amount must be > 0".to_string()));
    }

    let user_id = extract_user_id_from_headers(&headers)?;

    LotRepository::place_bid(&state.db, lot_id, user_id, payload.amount)
        .await
        .map_err(internal_error)?;

    Ok(StatusCode::CREATED)
}

fn extract_user_id_from_headers(headers: &HeaderMap) -> Result<Uuid, (StatusCode, String)> {
    let auth_header = headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or((
            StatusCode::UNAUTHORIZED,
            "Missing Authorization header".to_string(),
        ))?;

    let token = auth_header.strip_prefix("Bearer ").ok_or((
        StatusCode::UNAUTHORIZED,
        "Invalid Authorization format".to_string(),
    ))?;

    let claims = decode_jwt(token).map_err(|e| {
        (
            StatusCode::UNAUTHORIZED,
            format!("Invalid token: {e}"),
        )
    })?;

    claims.user_id().map_err(|e| {
        (
            StatusCode::UNAUTHORIZED,
            format!("Invalid user id in token: {e}"),
        )
    })
}

fn internal_error(err: sqlx::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Database error: {}", err),
    )
}