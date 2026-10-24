-- Add explicit ordering for folders and queues within a connection.
ALTER TABLE "folders"
    ADD COLUMN "sort_order" INTEGER NOT NULL DEFAULT 0;

ALTER TABLE "queues"
    ADD COLUMN "sort_order" INTEGER NOT NULL DEFAULT 0;

-- Preserve the current visible ordering while assigning stable positions.
WITH ordered_folders AS (
    SELECT
        id,
        ROW_NUMBER() OVER (
            PARTITION BY connection_id
            ORDER BY
                title COLLATE NOCASE,
                id
        ) - 1 AS position
    FROM folders
)
UPDATE folders
SET sort_order = (
    SELECT position
    FROM ordered_folders
    WHERE ordered_folders.id = folders.id
);

WITH ordered_queues AS (
    SELECT
        id,
        ROW_NUMBER() OVER (
            PARTITION BY connection_id
            ORDER BY queue_name COLLATE NOCASE, id
        ) - 1 AS position
    FROM queues
)
UPDATE queues
SET sort_order = (
    SELECT position
    FROM ordered_queues
    WHERE ordered_queues.id = queues.id
);

CREATE INDEX "idx_folders_connection_id_sort_order"
    ON "folders" ("connection_id", "sort_order");

CREATE INDEX "idx_queues_connection_id_sort_order"
    ON "queues" ("connection_id", "sort_order");
