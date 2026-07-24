//! The {{ PrefixName }} store — where the standard CRUD surface persists its entity.
{% if has_persistence %}//! Backed by the `_persistence` crate's connection pool; the entity lives in the
//! `{{ prefix_name }}s` table (created by this service's own migration).
{% else %}//! No persistence selected: an in-memory store keeps the API surface fully functional.
{% endif %}
{% if has_persistence %}
use {{ prefix_name }}_{{ suffix_name }}_persistence::PersistencePool;
use serde::{Deserialize, Serialize};
{% else %}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
{% endif %}
use uuid::Uuid;
/// The {{ PrefixName }} entity as the API surfaces it (JSON is camelCased at the boundary).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct {{ PrefixName }} {
    pub id: String,
    pub display_name: String,
}

{% if has_persistence %}
#[derive(Clone)]
pub struct Store {
    db: PersistencePool,
}

impl Store {
    pub fn new(db: PersistencePool) -> Self {
        Self { db }
    }

    pub async fn create(&self, display_name: &str) -> anyhow::Result<{{ PrefixName }}> {
        let id = Uuid::new_v4().to_string();
{% if persistence == 'MySQL' %}
        sqlx::query("INSERT INTO {{ prefix_name }}s (id, display_name) VALUES (?, ?)")
{% else %}
        sqlx::query("INSERT INTO {{ prefix_name }}s (id, display_name) VALUES ($1, $2)")
{% endif %}
            .bind(&id)
            .bind(display_name)
            .execute(self.db.pool())
            .await?;
        Ok({{ PrefixName }} {
            id,
            display_name: display_name.to_string(),
        })
    }

    pub async fn get(&self, id: &str) -> anyhow::Result<Option<{{ PrefixName }}>> {
{% if persistence == 'MySQL' %}
        let row: Option<(String, String)> = sqlx::query_as("SELECT id, display_name FROM {{ prefix_name }}s WHERE id = ?")
{% else %}
        let row: Option<(String, String)> = sqlx::query_as("SELECT id, display_name FROM {{ prefix_name }}s WHERE id = $1")
{% endif %}
            .bind(id)
            .fetch_optional(self.db.pool())
            .await?;
        Ok(row.map(|(id, display_name)| {{ PrefixName }} { id, display_name }))
    }

    pub async fn list(&self) -> anyhow::Result<Vec<{{ PrefixName }}>> {
        let rows: Vec<(String, String)> = sqlx::query_as("SELECT id, display_name FROM {{ prefix_name }}s ORDER BY id")
            .fetch_all(self.db.pool())
            .await?;
        Ok(rows
            .into_iter()
            .map(|(id, display_name)| {{ PrefixName }} { id, display_name })
            .collect())
    }

    pub async fn update(&self, id: &str, display_name: &str) -> anyhow::Result<Option<{{ PrefixName }}>> {
{% if persistence == 'MySQL' %}
        let result = sqlx::query("UPDATE {{ prefix_name }}s SET display_name = ? WHERE id = ?")
{% else %}
        let result = sqlx::query("UPDATE {{ prefix_name }}s SET display_name = $1 WHERE id = $2")
{% endif %}
            .bind(display_name)
            .bind(id)
            .execute(self.db.pool())
            .await?;
        if result.rows_affected() == 0 {
            return Ok(None);
        }
        Ok(Some({{ PrefixName }} {
            id: id.to_string(),
            display_name: display_name.to_string(),
        }))
    }

    pub async fn delete(&self, id: &str) -> anyhow::Result<bool> {
{% if persistence == 'MySQL' %}
        let result = sqlx::query("DELETE FROM {{ prefix_name }}s WHERE id = ?")
{% else %}
        let result = sqlx::query("DELETE FROM {{ prefix_name }}s WHERE id = $1")
{% endif %}
            .bind(id)
            .execute(self.db.pool())
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
{% else %}
#[derive(Clone, Default)]
pub struct Store {
    items: Arc<RwLock<HashMap<String, {{ PrefixName }}>>>,
}

impl Store {
    pub async fn create(&self, display_name: &str) -> anyhow::Result<{{ PrefixName }}> {
        let entity = {{ PrefixName }} {
            id: Uuid::new_v4().to_string(),
            display_name: display_name.to_string(),
        };
        self.items
            .write()
            .expect("store lock poisoned")
            .insert(entity.id.clone(), entity.clone());
        Ok(entity)
    }

    pub async fn get(&self, id: &str) -> anyhow::Result<Option<{{ PrefixName }}>> {
        Ok(self.items.read().expect("store lock poisoned").get(id).cloned())
    }

    pub async fn list(&self) -> anyhow::Result<Vec<{{ PrefixName }}>> {
        let mut items: Vec<_> = self
            .items
            .read()
            .expect("store lock poisoned")
            .values()
            .cloned()
            .collect();
        items.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(items)
    }

    pub async fn update(&self, id: &str, display_name: &str) -> anyhow::Result<Option<{{ PrefixName }}>> {
        let mut items = self.items.write().expect("store lock poisoned");
        match items.get_mut(id) {
            Some(entity) => {
                entity.display_name = display_name.to_string();
                Ok(Some(entity.clone()))
            }
            None => Ok(None),
        }
    }

    pub async fn delete(&self, id: &str) -> anyhow::Result<bool> {
        Ok(self.items.write().expect("store lock poisoned").remove(id).is_some())
    }
}
{% endif %}
