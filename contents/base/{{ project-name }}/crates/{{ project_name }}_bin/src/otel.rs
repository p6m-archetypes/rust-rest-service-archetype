/// OpenTelemetry initialization.
///
/// Configures an OTLP trace exporter using the standard OTEL env vars:
///   OTEL_SERVICE_NAME            — set to project-name by PlatformApplication spec.config
///   OTEL_EXPORTER_OTLP_ENDPOINT  — injected by the platform at deploy time
///   OTEL_TRACES_SAMPLER          — injected by the platform at deploy time
///
/// Fail-open: if OTEL_EXPORTER_OTLP_ENDPOINT is absent or the exporter fails
/// to initialize, the service starts without tracing rather than panicking.
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::runtime;
use opentelemetry_sdk::trace::TracerProvider;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

pub fn init_tracing(structured: bool) {
    // Default to `info` when RUST_LOG is unset — services must emit their startup and request
    // logs out of the box (structured logging is only observable if something is logged).
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // Format layer: JSON in non-local environments, human-readable otherwise.
    // Boxed so both variants share a single concrete type — required because the
    // OTLP layer below is generic over the subscriber it is attached to.
    let fmt_layer = if structured {
        tracing_subscriber::fmt::layer().json().boxed()
    } else {
        tracing_subscriber::fmt::layer().boxed()
    };

    // Build the OTLP layer only when the endpoint env var is set (fail-open).
    let otel_layer = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .ok()
        .filter(|e| !e.is_empty())
        .and_then(|endpoint| {
            let exporter = SpanExporter::builder()
                .with_tonic()
                .with_endpoint(endpoint)
                .build()
                .map_err(|e| tracing::warn!("OTLP exporter init failed: {e}"))
                .ok()?;

            let tracer_provider = TracerProvider::builder()
                .with_batch_exporter(exporter, runtime::Tokio)
                .build();

            let tracer = tracer_provider.tracer("{{ project-name }}");
            Some(OpenTelemetryLayer::new(tracer))
        });

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .with(otel_layer)
        .init();
}
