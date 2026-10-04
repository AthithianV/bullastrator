-- Bullastrator SQLite schema
--
-- This is a complete, from-scratch schema. It includes the tables previously
-- created by src-tauri/migration and the user ownership/access additions.

PRAGMA foreign_keys = ON;

BEGIN TRANSACTION;

CREATE TABLE "user" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "email" TEXT NOT NULL UNIQUE,
    "image" TEXT,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "workspace" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "user_id" TEXT,
    "name" TEXT NOT NULL,
    "color" TEXT,
    "active_tab_id" TEXT,
    "icon" INTEGER,
    "last_accessed_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "plan" TEXT NOT NULL DEFAULT 'FREE',
    "role" TEXT NOT NULL,
    "max_connections" INTEGER NOT NULL DEFAULT 1,
    "is_guest_mode" BOOLEAN NOT NULL DEFAULT 0,
    "is_primary" BOOLEAN NOT NULL DEFAULT 0,

    FOREIGN KEY ("user_id")
        REFERENCES "user" ("id") ON DELETE CASCADE
);

CREATE TABLE "connection" (
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
        REFERENCES "workspace" ("id") ON DELETE CASCADE,
    UNIQUE ("workspace_id", "name")
);

CREATE TABLE "tab" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "workspace_id" TEXT NOT NULL,
    "connection_id" TEXT,
    "user_id" TEXT,
    "title" TEXT NOT NULL,
    "params" TEXT NOT NULL,
    "is_active" BOOLEAN NOT NULL DEFAULT 0,
    "is_dirty" BOOLEAN NOT NULL DEFAULT 0,
    "is_pinned" BOOLEAN NOT NULL DEFAULT 0,
    "is_preview" BOOLEAN NOT NULL DEFAULT 0,
    "rank" INTEGER NOT NULL DEFAULT 0,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY ("workspace_id")
        REFERENCES "workspace" ("id") ON DELETE CASCADE,
    FOREIGN KEY ("connection_id")
        REFERENCES "connection" ("id") ON DELETE CASCADE,
    FOREIGN KEY ("user_id")
        REFERENCES "user" ("id") ON DELETE CASCADE
);

CREATE TABLE "folder" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "connection_id" TEXT NOT NULL,
    "user_id" TEXT,
    "title" TEXT NOT NULL,
    "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY ("connection_id")
        REFERENCES "connection" ("id") ON DELETE CASCADE,
    FOREIGN KEY ("user_id")
        REFERENCES "user" ("id") ON DELETE CASCADE,
    UNIQUE ("connection_id", "title")
);

CREATE TABLE "queue" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "connection_id" TEXT NOT NULL,
    "queue_name" TEXT NOT NULL,
    "display_name" TEXT,
    "is_starred" BOOLEAN DEFAULT 0,
    "auto_refresh_rate" INTEGER DEFAULT 5000,
    "notification_settings" TEXT DEFAULT 'critical',
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY ("connection_id")
        REFERENCES "connection" ("id") ON DELETE CASCADE,
    UNIQUE ("connection_id", "queue_name")
);

CREATE TABLE "folder_queue" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "folder_id" TEXT NOT NULL,
    "queue_id" TEXT NOT NULL,
    "sort_order" INTEGER NOT NULL DEFAULT 0,
    "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY ("folder_id")
        REFERENCES "folder" ("id") ON DELETE CASCADE,
    FOREIGN KEY ("queue_id")
        REFERENCES "queue" ("id") ON DELETE CASCADE,
    UNIQUE ("folder_id", "queue_id")
);

CREATE TABLE "settings" (
    "key" TEXT NOT NULL PRIMARY KEY,
    "user_id" TEXT,
    "value" TEXT,

    FOREIGN KEY ("user_id")
        REFERENCES "user" ("id") ON DELETE CASCADE
);

-- A user's role within a connection, for example OWNER, ADMIN, or MEMBER.
CREATE TABLE "user_role" (
    "user_id" TEXT NOT NULL,
    "connection_id" TEXT NOT NULL,
    "role" TEXT NOT NULL CHECK (length(trim("role")) > 0),
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY ("user_id", "connection_id"),

    FOREIGN KEY ("user_id")
        REFERENCES "user" ("id") ON DELETE CASCADE,
    FOREIGN KEY ("connection_id")
        REFERENCES "connection" ("id") ON DELETE CASCADE
);

-- A resource-specific permission. Exactly one of folder_id or queue_id must
-- be set, and role is either view or edit.
CREATE TABLE "user_access_connection" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "user_id" TEXT NOT NULL,
    "connection_id" TEXT NOT NULL,
    "folder_id" TEXT,
    "queue_id" TEXT,
    "role" TEXT NOT NULL CHECK ("role" IN ('view', 'edit')),
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CHECK (
        ("folder_id" IS NOT NULL AND "queue_id" IS NULL)
        OR
        ("folder_id" IS NULL AND "queue_id" IS NOT NULL)
    ),

    FOREIGN KEY ("user_id")
        REFERENCES "user" ("id") ON DELETE CASCADE,
    FOREIGN KEY ("connection_id")
        REFERENCES "connection" ("id") ON DELETE CASCADE,
    FOREIGN KEY ("connection_id", "folder_id")
        REFERENCES "folder" ("connection_id", "id") ON DELETE CASCADE,
    FOREIGN KEY ("connection_id", "queue_id")
        REFERENCES "queue" ("connection_id", "id") ON DELETE CASCADE
);

-- Audit trail for actions performed on queues.
-- Metadata stores the action-specific payload as JSON text.
CREATE TABLE "queue_actions" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "workspace_id" TEXT NOT NULL,
    "connection_id" TEXT NOT NULL,
    "user_id" TEXT NOT NULL,
    "action" TEXT NOT NULL,
    "metadata" TEXT,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY ("workspace_id")
        REFERENCES "workspace" ("id") ON DELETE CASCADE,
    FOREIGN KEY ("connection_id")
        REFERENCES "connection" ("id") ON DELETE CASCADE,
    FOREIGN KEY ("user_id")
        REFERENCES "user" ("id") ON DELETE CASCADE
);

-- Composite resource keys make sure an access row cannot use a folder or
-- queue belonging to a different connection.
CREATE UNIQUE INDEX "idx-folder-connection-id"
    ON "folder" ("connection_id", "id");
CREATE UNIQUE INDEX "idx-queue-connection-id"
    ON "queue" ("connection_id", "id");

CREATE INDEX "idx-tab-user-id" ON "tab" ("user_id");
CREATE INDEX "idx-folder-user-id" ON "folder" ("user_id");
CREATE INDEX "idx-settings-user-id" ON "settings" ("user_id");
CREATE INDEX "idx-user-role-connection-id"
    ON "user_role" ("connection_id");
CREATE INDEX "idx-queue-actions-workspace-id"
    ON "queue_actions" ("workspace_id");
CREATE INDEX "idx-queue-actions-connection-id"
    ON "queue_actions" ("connection_id");
CREATE INDEX "idx-queue-actions-user-id"
    ON "queue_actions" ("user_id");

CREATE UNIQUE INDEX "idx-user-access-folder"
    ON "user_access_connection" ("user_id", "connection_id", "folder_id")
    WHERE "folder_id" IS NOT NULL;
CREATE UNIQUE INDEX "idx-user-access-queue"
    ON "user_access_connection" ("user_id", "connection_id", "queue_id")
    WHERE "queue_id" IS NOT NULL;

COMMIT;
