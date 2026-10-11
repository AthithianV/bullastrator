-- ============================================================
-- Bullastrator SQLite schema
-- ============================================================
--
-- Architecture:
--
-- users
--   │
--   ├── active_workspace_id ────────┐
--   │                               │
--   ▼                               ▼
-- workspace_members ────────────> workspaces
--                                      │
--                                      ├── connections
--                                      │      ├── queues
--                                      │      └── folders
--                                      │
--                                      └── tabs
--
-- Authentication:
--   users -> sessions
--
-- Workspace membership:
--   users <-> workspace_members <-> workspaces
--
-- Connection/resource permissions:
--   workspace_members
--       ↓
--   connections
--       ↓
--   folders / queues
--       ↓
--   user_access_connections
--
-- ============================================================

PRAGMA foreign_keys = ON;

BEGIN TRANSACTION;


-- ============================================================
-- USERS
-- ============================================================

CREATE TABLE "users" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "email" TEXT NOT NULL UNIQUE,
    "password_hash" TEXT,
    "image" TEXT,

    "active_workspace_id" TEXT,

    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY ("active_workspace_id")
        REFERENCES "workspaces" ("id")
        ON DELETE SET NULL
);


-- ============================================================
-- SESSIONS
-- ============================================================

CREATE TABLE "sessions" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "user_id" TEXT NOT NULL,
    "token_hash" TEXT NOT NULL UNIQUE,
    "expires_at" TIMESTAMP NOT NULL,
    "revoked_at" TIMESTAMP,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY ("user_id")
        REFERENCES "users" ("id")
        ON DELETE CASCADE
);

CREATE INDEX "idx_sessions_user_id"
    ON "sessions" ("user_id");

CREATE INDEX "idx_session_token_hash"
    ON "sessions" ("token_hash");


-- ============================================================
-- WORKSPACES
-- ============================================================

CREATE TABLE "workspaces" (
    "id" TEXT NOT NULL PRIMARY KEY,

    "user_id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "color" TEXT,
    "icon" INTEGER,

    "active_tab_id" TEXT,

    "last_accessed_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY ("user_id")
        REFERENCES "users" ("id")
        ON DELETE CASCADE
);

-- ============================================================
-- WORKSPACE MEMBERS
-- ============================================================
--
-- Defines which users have access to which workspaces.
--
-- role examples:
--   OWNER
--   ADMIN
--   MEMBER
--
-- Authorization should use this table.
-- users.active_workspace_id should NOT be used for authorization.
-- ============================================================

CREATE TABLE "workspace_members" (
    "workspace_id" TEXT NOT NULL,
    "user_id" TEXT NOT NULL,

    "role" TEXT NOT NULL
        CHECK (length(trim("role")) > 0),

    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY ("workspace_id", "user_id"),

    FOREIGN KEY ("workspace_id")
        REFERENCES "workspaces" ("id")
        ON DELETE CASCADE,

    FOREIGN KEY ("user_id")
        REFERENCES "users" ("id")
        ON DELETE CASCADE
);

CREATE INDEX "idx_workspace_members_user_id"
    ON "workspace_members" ("user_id");

CREATE INDEX "idx_workspace_members_workspace_id"
    ON "workspace_members" ("workspace_id");


-- ============================================================
-- CONNECTIONS
-- ============================================================

CREATE TABLE "connections" (
    "id" TEXT NOT NULL PRIMARY KEY,

    "workspace_id" TEXT NOT NULL,

    "name" TEXT NOT NULL,
    "host" TEXT NOT NULL,
    "port" INTEGER NOT NULL,

    "password" TEXT,
    "username" TEXT,
    "db" INTEGER DEFAULT 0,

    "last_synced_at" TIMESTAMP,

    "bullmq_prefix" TEXT DEFAULT 'bull',

    "is_tls_enabled" BOOLEAN DEFAULT 0,

    "color" TEXT,
    "label" TEXT,

    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY ("workspace_id")
        REFERENCES "workspaces" ("id")
        ON DELETE CASCADE,

    UNIQUE ("workspace_id", "name")
);

CREATE INDEX "idx_connections_workspace_id"
    ON "connections" ("workspace_id");


-- ============================================================
-- TABS
-- ============================================================

CREATE TABLE "tabs" (
    "id" TEXT NOT NULL PRIMARY KEY,

    "workspace_id" TEXT NOT NULL,

    "connection_id" TEXT,

    "user_id" TEXT,

    "title" TEXT NOT NULL,

    -- JSON stored as TEXT.
    "params" TEXT NOT NULL,

    "is_active" BOOLEAN NOT NULL DEFAULT 0,
    "is_dirty" BOOLEAN NOT NULL DEFAULT 0,
    "is_pinned" BOOLEAN NOT NULL DEFAULT 0,
    "is_preview" BOOLEAN NOT NULL DEFAULT 0,

    "rank" INTEGER NOT NULL DEFAULT 0,

    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY ("workspace_id")
        REFERENCES "workspaces" ("id")
        ON DELETE CASCADE,

    FOREIGN KEY ("connection_id")
        REFERENCES "connections" ("id")
        ON DELETE CASCADE,

    FOREIGN KEY ("user_id")
        REFERENCES "users" ("id")
        ON DELETE CASCADE
);

CREATE INDEX "idx_tabs_workspace_id"
    ON "tabs" ("workspace_id");

CREATE INDEX "idx_tabs_connection_id"
    ON "tabs" ("connection_id");

CREATE INDEX "idx_tabs_user_id"
    ON "tabs" ("user_id");


-- ============================================================
-- FOLDERS
-- ============================================================

CREATE TABLE "folders" (
    "id" TEXT NOT NULL PRIMARY KEY,

    "connection_id" TEXT NOT NULL,

    "user_id" TEXT,

    "title" TEXT NOT NULL,

    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY ("connection_id")
        REFERENCES "connections" ("id")
        ON DELETE CASCADE,

    FOREIGN KEY ("user_id")
        REFERENCES "users" ("id")
        ON DELETE CASCADE,

    UNIQUE ("connection_id", "title")
);

CREATE INDEX "idx_folders_connection_id"
    ON "folders" ("connection_id");

CREATE INDEX "idx_folders_user_id"
    ON "folders" ("user_id");


-- ============================================================
-- QUEUES
-- ============================================================

CREATE TABLE "queues" (
    "id" TEXT NOT NULL PRIMARY KEY,

    "connection_id" TEXT NOT NULL,

    "queue_name" TEXT NOT NULL,

    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY ("connection_id")
        REFERENCES "connections" ("id")
        ON DELETE CASCADE,

    UNIQUE ("connection_id", "queue_name")
);

CREATE INDEX "idx_queues_connection_id"
    ON "queues" ("connection_id");


-- ============================================================
-- COMPOSITE RESOURCE KEYS
-- ============================================================
--
-- These allow user_access_connections to enforce that:
--
--   connection_id + folder_id
--
-- actually identifies a folder belonging to that connection.
--
-- Same for queues.
-- ============================================================

CREATE UNIQUE INDEX "idx_folders_connection_id_id"
    ON "folders" ("connection_id", "id");

CREATE UNIQUE INDEX "idx_queues_connection_id_id"
    ON "queues" ("connection_id", "id");


-- ============================================================
-- FOLDER QUEUES
-- ============================================================

CREATE TABLE "folder_queues" (
    "id" TEXT NOT NULL PRIMARY KEY,

    "folder_id" TEXT NOT NULL,
    "queue_id" TEXT NOT NULL,

    "sort_order" INTEGER NOT NULL DEFAULT 0,

    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY ("folder_id")
        REFERENCES "folders" ("id")
        ON DELETE CASCADE,

    FOREIGN KEY ("queue_id")
        REFERENCES "queues" ("id")
        ON DELETE CASCADE,

    UNIQUE ("folder_id", "queue_id")
);

CREATE INDEX "idx_folder_queues_folder_id"
    ON "folder_queues" ("folder_id");

CREATE INDEX "idx_folder_queues_queue_id"
    ON "folder_queues" ("queue_id");


-- ============================================================
-- SETTINGS
-- ============================================================

CREATE TABLE "settings" (
    "key" TEXT NOT NULL PRIMARY KEY,

    "user_id" TEXT,

    "value" TEXT,

    FOREIGN KEY ("user_id")
        REFERENCES "users" ("id")
        ON DELETE CASCADE
);

CREATE INDEX "idx_settings_user_id"
    ON "settings" ("user_id");


-- ============================================================
-- USER RESOURCE ACCESS
-- ============================================================
--
-- Despite the historical table name, this controls access to
-- resources inside a connection.
--
-- Exactly one of:
--
--   folder_id
--   queue_id
--
-- must be specified.
--
-- role:
--   view
--   edit
-- ============================================================

CREATE TABLE "user_access_connections" (
    "id" TEXT NOT NULL PRIMARY KEY,

    "user_id" TEXT NOT NULL,

    "connection_id" TEXT NOT NULL,

    "folder_id" TEXT,
    "queue_id" TEXT,

    "role" TEXT NOT NULL
        CHECK ("role" IN ('view', 'edit')),

    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CHECK (
        (
            "folder_id" IS NOT NULL
            AND "queue_id" IS NULL
        )
        OR
        (
            "folder_id" IS NULL
            AND "queue_id" IS NOT NULL
        )
    ),

    FOREIGN KEY ("user_id")
        REFERENCES "users" ("id")
        ON DELETE CASCADE,

    FOREIGN KEY ("connection_id")
        REFERENCES "connections" ("id")
        ON DELETE CASCADE,

    FOREIGN KEY ("connection_id", "folder_id")
        REFERENCES "folders" ("connection_id", "id")
        ON DELETE CASCADE,

    FOREIGN KEY ("connection_id", "queue_id")
        REFERENCES "queues" ("connection_id", "id")
        ON DELETE CASCADE
);

CREATE INDEX "idx_user_access_connections_user_id"
    ON "user_access_connections" ("user_id");

CREATE INDEX "idx_user_access_connections_connection_id"
    ON "user_access_connections" ("connection_id");

CREATE INDEX "idx_user_access_connections_folder_id"
    ON "user_access_connections" ("folder_id");

CREATE INDEX "idx_user_access_connections_queue_id"
    ON "user_access_connections" ("queue_id");

CREATE UNIQUE INDEX "idx_user_access_folder"
    ON "user_access_connections"
        ("user_id", "connection_id", "folder_id")
    WHERE "folder_id" IS NOT NULL;

CREATE UNIQUE INDEX "idx_user_access_queue"
    ON "user_access_connections"
        ("user_id", "connection_id", "queue_id")
    WHERE "queue_id" IS NOT NULL;


-- ============================================================
-- QUEUE ACTIONS / AUDIT LOG
-- ============================================================

CREATE TABLE "queue_actions" (
    "id" TEXT NOT NULL PRIMARY KEY,

    "workspace_id" TEXT NOT NULL,

    "connection_id" TEXT NOT NULL,

    "user_id" TEXT NOT NULL,

    "action" TEXT NOT NULL,

    -- Action-specific JSON payload.
    "metadata" TEXT,

    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY ("workspace_id")
        REFERENCES "workspaces" ("id")
        ON DELETE CASCADE,

    FOREIGN KEY ("connection_id")
        REFERENCES "connections" ("id")
        ON DELETE CASCADE,

    FOREIGN KEY ("user_id")
        REFERENCES "users" ("id")
        ON DELETE CASCADE
);

CREATE INDEX "idx_queue_actions_workspace_id"
    ON "queue_actions" ("workspace_id");

CREATE INDEX "idx_queue_actions_connection_id"
    ON "queue_actions" ("connection_id");

CREATE INDEX "idx_queue_actions_user_id"
    ON "queue_actions" ("user_id");

CREATE INDEX "idx_queue_actions_created_at"
    ON "queue_actions" ("created_at");


COMMIT;
