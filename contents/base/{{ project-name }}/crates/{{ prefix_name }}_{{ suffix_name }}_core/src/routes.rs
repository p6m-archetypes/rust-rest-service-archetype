use axum::{Router, routing::get};
use crate::{AppState, handlers};

/// Main service router — domain routes only.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/{{ prefix-name }}s", get(handlers::list))
        .with_state(state)
}

/// Management router — health and metrics endpoints on management_port.
/// Kept separate from the service router so Kubernetes network policy can
/// restrict probe traffic independently from service traffic.
pub fn management_router() -> Router {
    Router::new()
        .route("/health/readiness", get(handlers::readiness))
        .route("/health/liveness", get(handlers::liveness))
        .route("/metrics", get(handlers::metrics))
}
