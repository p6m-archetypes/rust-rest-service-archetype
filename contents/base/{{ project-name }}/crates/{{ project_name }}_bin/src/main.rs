use anyhow::Result;
use clap::Parser;

mod cli;
mod otel;
mod settings;

{% if cache ~= 'None' %}
use {{ project_name }}_cache::connect as cache_connect;
{% endif %}
use {{ project_name }}_core::{{ ProjectName }}Core;
{% if messaging ~= 'None' %}
use {{ project_name }}_messaging::MessagingClient;
{% endif %}
{% if persistence ~= 'None' %}
use {{ project_name }}_persistence::PersistencePool;
{% endif %}
{% if has_azure_blob %}
use {{ project_name }}_storage_azure::connect as azure_connect;
{% endif %}
{% if has_s3 %}
use {{ project_name }}_storage_s3::connect as s3_connect;
{% endif %}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let cli = cli::Cli::parse();
    let settings = settings::Settings::load(cli.config.as_deref())?;

    // Structured JSON logging + OpenTelemetry tracing.
    // LOGGING_STRUCTURED=true → JSON format (injected via PlatformApplication spec.config).
    // OTEL_EXPORTER_OTLP_ENDPOINT → enables OTLP export (fail-open: absent = no traces).
    let structured = std::env::var("LOGGING_STRUCTURED")
        .map(|v| v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    otel::init_tracing(structured);

    match cli.command {
        Some(cli::Commands::Config { action }) => match action {
            cli::ConfigAction::Defaults => {
                println!("{}", toml::to_string_pretty(&settings::Settings::default())?);
            }
            cli::ConfigAction::Show => {
                println!("{}", toml::to_string_pretty(&settings)?);
            }
        },
{% if persistence ~= 'None' %}
        Some(cli::Commands::Migrate { action }) => {
            let pool = PersistencePool::connect(&settings.persistence).await?;
            match action {
                cli::MigrateAction::Up => pool.migrate_up().await?,
                cli::MigrateAction::Down => pool.migrate_down(Some(1)).await?,
            }
        }
{% endif %}
        None => {
            tracing::info!("Starting {{ project-name }}...");

{% if persistence ~= 'None' %}
            let db = PersistencePool::connect(&settings.persistence).await?;
{% endif %}
{% if cache ~= 'None' %}
            let cache = cache_connect(&settings.cache).await?;
{% endif %}
{% if messaging ~= 'None' %}
            let messaging = MessagingClient::connect(&settings.messaging).await?;
{% endif %}
{% if has_s3 %}
            let _s3 = s3_connect(&settings.storage_s3).await?;
{% endif %}
{% if has_azure_blob %}
            let _azure = azure_connect(&settings.storage_azure)?;
{% endif %}
            let core = {{ ProjectName }}Core::builder({% if persistence ~= 'None' %}db{% endif %})
                .with_settings(&settings.core)
{% if cache ~= 'None' %}
                .with_cache(cache)
{% endif %}
{% if messaging ~= 'None' %}
                .with_messaging(messaging)
{% endif %}
                .build()
                .await?;

            let svc_router = core.router();
            let mgmt_router = {{ ProjectName }}Core::management_router();

            // Service server — domain traffic on service_port
            let svc_addr = format!("{}:{}", settings.server.host, settings.server.port);
            let svc_listener = tokio::net::TcpListener::bind(&svc_addr).await?;
            tracing::info!("REST service listening on {svc_addr}");

            // Management server — health probes and metrics on management_port
            let mgmt_addr = format!("{}:{}", settings.server.host, settings.server.management_port);
            let mgmt_listener = tokio::net::TcpListener::bind(&mgmt_addr).await?;
            tracing::info!("Management server listening on {mgmt_addr}");

            tokio::select! {
                result = axum::serve(svc_listener, svc_router).with_graceful_shutdown(shutdown_signal()) => {
                    result?;
                }
                result = axum::serve(mgmt_listener, mgmt_router) => {
                    result?;
                }
            }
        }
    }

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.expect("failed to listen for ctrl-c");
    tracing::info!("Shutting down...");
}
