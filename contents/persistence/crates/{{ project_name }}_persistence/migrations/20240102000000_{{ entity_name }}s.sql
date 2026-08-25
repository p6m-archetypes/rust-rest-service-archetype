-- The {{ EntityName }} entity table — the standard CRUD API persists here.
-- Owned by the service archetype (the resource library owns only the baseline).
CREATE TABLE {{ entity_name }}s (
    id VARCHAR(36) PRIMARY KEY,
    display_name TEXT NOT NULL
);
