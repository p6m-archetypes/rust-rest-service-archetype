use figment::{Figment, providers::{Format, Toml, Env, Serialized}};
use serde::{Deserialize, Serialize};
use {{ prefix_name }}_{{ suffix_name }}_core::settings::CoreSettings;
{% if persistence ~= 'None' %}use {{ prefix_name }}_{{ suffix_name }}_persistence::settings::PersistenceSettings;
{% endif %}{% if cache ~= 'None' %}use {{ prefix_name }}_{{ suffix_name }}_cache::settings::CacheSettings;
{% endif %}{% if messaging ~= 'None' %}use {{ prefix_name }}_{{ suffix_name }}_messaging::settings::MessagingSettings;
{% endif %}{% if has_s3 %}use {{ prefix_name }}_{{ suffix_name }}_storage_s3::settings::StorageS3Settings;
{% endif %}{% if has_azure_blob %}use {{ prefix_name }}_{{ suffix_name }}_storage_azure::settings::StorageAzureSettings;
{% endif %}
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub core: CoreSettings,
{% if persistence ~= 'None' %}    pub persistence: PersistenceSettings,
{% endif %}{% if cache ~= 'None' %}    pub cache: CacheSettings,
{% endif %}{% if messaging ~= 'None' %}    pub messaging: MessagingSettings,
{% endif %}{% if has_s3 %}    pub storage_s3: StorageS3Settings,
{% endif %}{% if has_azure_blob %}    pub storage_azure: StorageAzureSettings,
{% endif %}}

impl Settings {
    pub fn load(config_file: Option<&str>) -> anyhow::Result<Self> {
        let mut figment = Figment::from(Serialized::defaults(Self::default()))
            .merge(Toml::file("config/default.toml"));

        if let Some(path) = config_file {
            figment = figment.merge(Toml::file(path));
        }

        figment = figment.merge(Env::prefixed("APP_").split("__"));

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
