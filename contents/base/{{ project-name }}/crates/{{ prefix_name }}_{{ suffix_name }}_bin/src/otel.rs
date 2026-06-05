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
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, trace as sdktrace};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

pub fn init_tracing(structured: bool) {
    let env_filter = EnvFilter::from_default_env();

    // Build the OTLP layer only when the endpoint env var is set.
    let otel_layer = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .ok()
        .filter(|e| !e.is_empty())
        .and_then(|endpoint| {
            let exporter = opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(endpoint);

            let tracer_provider = opentelemetry_otlp::new_pipeline()
                .tracing()
                .with_exporter(exporter)
                .with_trace_config(sdktrace::Config::default())
                .install_batch(runtime::Tokio)
                .map_err(|e| tracing::warn!("OTLP tracer init failed: {e}"))
                .ok()?;

            let tracer = tracer_provider.tracer("{{ project-name }}");
            Some(OpenTelemetryLayer::new(tracer))
        });

    if structured {
        let fmt_layer = tracing_subscriber::fmt::layer()
            .json()
            .with_filter(env_filter);

        match otel_layer {
            Some(otel) => tracing_subscriber::registry()
                .with(fmt_layer)
                .with(otel)
                .init(),
            None => tracing_subscriber::registry()
                .with(fmt_layer)
                .init(),
        }
    } else {
        let fmt_layer = tracing_subscriber::fmt::layer().with_filter(env_filter);

        match otel_layer {
            Some(otel) => tracing_subscriber::registry()
                .with(fmt_layer)
                .with(otel)
                .init(),
            None => tracing_subscriber::registry()
                .with(fmt_layer)
                .init(),
        }
    }
}
