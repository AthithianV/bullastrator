-- Global user role.
ALTER TABLE "users"
    ADD COLUMN "role" TEXT NOT NULL DEFAULT 'USER'
        CHECK ("role" IN ('USER', 'GLOBAL_ADMIN'));

-- Explicit connection-level access grants.
CREATE TABLE IF NOT EXISTS "user_connection_roles" (
    "user_id" TEXT NOT NULL,
    "connection_id" TEXT NOT NULL,
    "role" TEXT NOT NULL
        CHECK ("role" IN ('VIEWER', 'EDITOR')),

    PRIMARY KEY ("user_id", "connection_id"),

    FOREIGN KEY ("user_id")
        REFERENCES "users" ("id")
        ON DELETE CASCADE,

    FOREIGN KEY ("connection_id")
        REFERENCES "connections" ("id")
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS "idx_user_connection_roles_connection_id"
    ON "user_connection_roles" ("connection_id");

-- Explicit folder-level access grants.
CREATE TABLE IF NOT EXISTS "user_folder_roles" (
    "user_id" TEXT NOT NULL,
    "folder_id" TEXT NOT NULL,
    "role" TEXT NOT NULL
        CHECK ("role" IN ('VIEWER', 'EDITOR')),

    PRIMARY KEY ("user_id", "folder_id"),

    FOREIGN KEY ("user_id")
        REFERENCES "users" ("id")
        ON DELETE CASCADE,

    FOREIGN KEY ("folder_id")
        REFERENCES "folders" ("id")
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS "idx_user_folder_roles_folder_id"
    ON "user_folder_roles" ("folder_id");

-- Explicit queue-level access grants.
CREATE TABLE IF NOT EXISTS "user_queue_roles" (
    "user_id" TEXT NOT NULL,
    "queue_id" TEXT NOT NULL,
    "role" TEXT NOT NULL
        CHECK ("role" IN ('VIEWER', 'EDITOR')),

    PRIMARY KEY ("user_id", "queue_id"),

    FOREIGN KEY ("user_id")
        REFERENCES "users" ("id")
        ON DELETE CASCADE,

    FOREIGN KEY ("queue_id")
        REFERENCES "queues" ("id")
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS "idx_user_queue_roles_queue_id"
    ON "user_queue_roles" ("queue_id");
