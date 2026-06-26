pub mod error;
pub mod handlers;
pub mod routes;
pub mod settings;

use anyhow::Result;
use axum::Router;
use settings::CoreSettings;
{% if persistence ~= 'None' %}use {{ prefix_name }}_{{ suffix_name }}_persistence::PersistencePool;
{% endif %}{% if cache ~= 'None' %}use {{ prefix_name }}_{{ suffix_name }}_cache::CachePool;
{% endif %}{% if messaging ~= 'None' %}use {{ prefix_name }}_{{ suffix_name }}_messaging::MessagingClient;
{% endif %}
#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)]
    settings: CoreSettings,
{% if persistence ~= 'None' %}    pub db: PersistencePool,
{% endif %}{% if cache ~= 'None' %}    pub cache: CachePool,
{% endif %}{% if messaging ~= 'None' %}    pub messaging: MessagingClient,
{% endif %}}

pub struct {{ PrefixName }}{{ SuffixName }}Core {
    state: AppState,
}

impl {{ PrefixName }}{{ SuffixName }}Core {
    pub fn builder({% if persistence ~= 'None' %}db: PersistencePool{% endif %}) -> Builder {
        Builder::new({% if persistence ~= 'None' %}db{% endif %})
    }

    pub fn router(self) -> Router {
        routes::router(self.state)
    }

    pub fn management_router() -> Router {
        routes::management_router()
    }
}

pub struct Builder {
    settings: CoreSettings,
{% if persistence ~= 'None' %}    db: PersistencePool,
{% endif %}{% if cache ~= 'None' %}    cache: Option<CachePool>,
{% endif %}{% if messaging ~= 'None' %}    messaging: Option<MessagingClient>,
{% endif %}}

impl Builder {
    #[allow(clippy::new_without_default)]
    pub fn new({% if persistence ~= 'None' %}db: PersistencePool{% endif %}) -> Self {
        Self {
            settings: CoreSettings::default(),
{% if persistence ~= 'None' %}            db,
{% endif %}{% if cache ~= 'None' %}            cache: None,
{% endif %}{% if messaging ~= 'None' %}            messaging: None,
{% endif %}        }
    }

    pub fn with_settings(mut self, settings: &CoreSettings) -> Self {
        self.settings = settings.clone();
        self
    }
{% if cache ~= 'None' %}
    pub fn with_cache(mut self, cache: CachePool) -> Self {
        self.cache = Some(cache);
        self
    }
{% endif %}
{% if messaging ~= 'None' %}
    pub fn with_messaging(mut self, messaging: MessagingClient) -> Self {
        self.messaging = Some(messaging);
        self
    }
{% endif %}

    pub async fn build(self) -> Result<{{ PrefixName }}{{ SuffixName }}Core> {
        Ok({{ PrefixName }}{{ SuffixName }}Core {
            state: AppState {
                settings: self.settings,
{% if persistence ~= 'None' %}                db: self.db,
{% endif %}{% if cache ~= 'None' %}                cache: self.cache.expect("cache must be set via with_cache()"),
{% endif %}{% if messaging ~= 'None' %}                messaging: self.messaging.expect("messaging must be set via with_messaging()"),
{% endif %}            },
        })
    }
}
