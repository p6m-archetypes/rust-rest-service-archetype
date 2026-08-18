use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{error::AppError, store::{{ EntityName }}, AppState};

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
}

pub async fn readiness() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

pub async fn liveness() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

/// Prometheus metrics endpoint: renders everything the installed recorder has collected.
pub async fn metrics(handle: metrics_exporter_prometheus::PrometheusHandle) -> impl IntoResponse {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain; version=0.0.4; charset=utf-8")],
        handle.render(),
    )
}

/// Create/update request body — JSON is camelCased at the boundary.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct {{ EntityName }}Request {
    pub display_name: String,
}

pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<{{ EntityName }}Request>,
) -> Result<(StatusCode, Json<{{ EntityName }}>), AppError> {
    let created = state.store.create(&req.display_name).await?;
    Ok((StatusCode::CREATED, Json(created)))
}

pub async fn list(State(state): State<AppState>) -> Result<Json<Vec<{{ EntityName }}>>, AppError> {
    Ok(Json(state.store.list().await?))
}

pub async fn get(State(state): State<AppState>, Path(id): Path<String>) -> Result<Json<{{ EntityName }}>, AppError> {
    match state.store.get(&id).await? {
        Some(entity) => Ok(Json(entity)),
        None => Err(AppError::NotFound),
    }
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<{{ EntityName }}Request>,
) -> Result<Json<{{ EntityName }}>, AppError> {
    match state.store.update(&id, &req.display_name).await? {
        Some(entity) => Ok(Json(entity)),
        None => Err(AppError::NotFound),
    }
}

pub async fn delete(State(state): State<AppState>, Path(id): Path<String>) -> Result<StatusCode, AppError> {
    if state.store.delete(&id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}
