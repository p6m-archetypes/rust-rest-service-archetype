{% if cache ~= 'None' %}
use {{ project_name }}_cache::settings::CacheSettings;
{% endif %}
use {{ project_name }}_core::settings::CoreSettings;
{% if messaging ~= 'None' %}
use {{ project_name }}_messaging::settings::MessagingSettings;
{% endif %}
{% if persistence ~= 'None' %}
use {{ project_name }}_persistence::settings::PersistenceSettings;
{% endif %}
{% if has_azure_blob %}
use {{ project_name }}_storage_azure::settings::StorageAzureSettings;
{% endif %}
{% if has_s3 %}
use {{ project_name }}_storage_s3::settings::StorageS3Settings;
{% endif %}
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub core: CoreSettings,
{% if persistence ~= 'None' %}
    pub persistence: PersistenceSettings,
{% endif %}
{% if cache ~= 'None' %}
    pub cache: CacheSettings,
{% endif %}
{% if messaging ~= 'None' %}
    pub messaging: MessagingSettings,
{% endif %}
{% if has_s3 %}
    pub storage_s3: StorageS3Settings,
{% endif %}
{% if has_azure_blob %}
    pub storage_azure: StorageAzureSettings,
{% endif %}
}

impl Settings {
    pub fn load(config_file: Option<&str>) -> anyhow::Result<Self> {
        let mut figment = Figment::from(Serialized::defaults(Self::default())).merge(Toml::file("config/default.toml"));

        if let Some(path) = config_file {
            figment = figment.merge(Toml::file(path));
        }

        figment = figment.merge(Env::prefixed("APP_").split("__"));

        // Platform environment contract: the deployment manifests inject the server ports as
        // bare variables (SERVER_PORT / MANAGEMENT_PORT). Layer them onto the settings tree so
        // the platform's names are honored; APP_-prefixed vars keep working for local overrides.
        // DB_* / CACHE_* / MESSAGING_* are consumed by the resource crates directly.
        figment = figment.merge(
            Env::raw()
                .filter(|key| key == "SERVER_PORT" || key == "MANAGEMENT_PORT")
                .map(|key| {
                    if key == "SERVER_PORT" {
                        "server.port".into()
                    } else {
                        "server.management_port".into()
                    }
                })
                .split("."),
        );
{% if persistence ~= 'None' %}
        // The database contract arrives as discrete DB_* vars; assemble the connection URL
        // the persistence layer expects.
        if let (Ok(host), Ok(port), Ok(user), Ok(pass), Ok(db)) = (
            std::env::var("DB_HOST"),
            std::env::var("DB_PORT"),
            std::env::var("DB_USERNAME"),
            std::env::var("DB_PASSWORD"),
            std::env::var("DB_DBNAME"),
        ) {
            figment = figment.merge(Serialized::default(
                "persistence.url",
                format!("{% if persistence == 'MySQL' %}mysql{% else %}postgres{% endif %}://{user}:{pass}@{host}:{port}/{db}"),
            ));
        }
{% endif %}
        Ok(figment.extract()?)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
    pub management_port: u16,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: {{ service_port }},
            management_port: {{ management_port }},
        }
    }
}
