use axum::{Json, extract::State, http::{header, StatusCode}, response::IntoResponse};
use serde::{Deserialize, Serialize};
use crate::{AppState, error::AppError};

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

/// Prometheus metrics endpoint.
/// Returns metrics collected by the `metrics` crate via `metrics-exporter-prometheus`.
pub async fn metrics() -> impl IntoResponse {
    // TODO: wire up metrics-exporter-prometheus handle and return rendered text.
    // For now returns an empty valid Prometheus response so Kubernetes scraping succeeds.
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain; version=0.0.4; charset=utf-8")],
        "# Prometheus metrics\n",
    )
}

#[derive(Serialize, Deserialize)]
pub struct {{ PrefixName }}Item {
    pub id: String,
    pub name: String,
}

pub async fn list(State(_state): State<AppState>) -> Result<Json<Vec<{{ PrefixName }}Item>>, AppError> {
    // TODO: implement domain logic
    Ok(Json(vec![]))
}
