ALTER TABLE "user_connection_roles"
    ADD COLUMN "folder_scope" TEXT NOT NULL DEFAULT 'SELECTED'
        CHECK ("folder_scope" IN ('SELECTED', 'ALL'));

ALTER TABLE "user_connection_roles"
    ADD COLUMN "queue_scope" TEXT NOT NULL DEFAULT 'SELECTED'
        CHECK ("queue_scope" IN ('SELECTED', 'ALL'));
