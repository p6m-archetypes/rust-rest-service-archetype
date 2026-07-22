use axum::{
    Router,
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::get,
};
use metrics_exporter_prometheus::PrometheusBuilder;

use crate::{AppState, handlers};

/// Main service router — domain routes only, versioned and named from the service identity.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route(
            "/api/v1/{{ prefix-name }}s",
            get(handlers::list).post(handlers::create),
        )
        .route(
            "/api/v1/{{ prefix-name }}s/{id}",
            get(handlers::get).put(handlers::update).delete(handlers::delete),
        )
        .layer(middleware::from_fn(track_requests))
        .with_state(state)
}

/// Management router — health and metrics endpoints on management_port.
/// Kept separate from the service router so Kubernetes network policy can
/// restrict probe traffic independently from service traffic.
///
/// Installs the process-global Prometheus recorder (call once, at startup).
pub fn management_router() -> Router {
    let metrics_handle = PrometheusBuilder::new()
        .install_recorder()
        .expect("failed to install Prometheus metrics recorder");

    // Seed a build-info family so /metrics is meaningful from the first scrape.
    metrics::gauge!(
        "{{ prefix_name }}_{{ suffix_name }}_build_info",
        "version" => env!("CARGO_PKG_VERSION")
    )
    .set(1.0);

    Router::new()
        .route("/health/readiness", get(handlers::readiness))
        .route("/health/liveness", get(handlers::liveness))
        .route(
            "/metrics",
            get(move || {
                let handle = metrics_handle.clone();
                async move { handlers::metrics(handle).await }
            }),
        )
}

/// Record every service request as a labeled counter.
async fn track_requests(req: Request, next: Next) -> Response {
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let response = next.run(req).await;
    metrics::counter!(
        "http_requests_total",
        "method" => method,
        "path" => path,
        "status" => response.status().as_u16().to_string()
    )
    .increment(1);
    response
}
