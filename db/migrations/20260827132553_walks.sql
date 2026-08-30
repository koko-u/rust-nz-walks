-- migrate:up
CREATE TABLE IF NOT EXISTS "walks"
(
    "id"          UUID             NOT NULL DEFAULT uuidv7(),
    "name"        VARCHAR(255)     NOT NULL,
    "description" TEXT             NULL,
    "length_km"   DOUBLE PRECISION NULL,
    "image_url"   VARCHAR(2048)    NULL,
    "region_id"   UUID             NOT NULL,
    "difficulty"  difficulty       NULL,
    "created_at"  TIMESTAMPTZ      NOT NULL DEFAULT now(),
    "updated_at"  TIMESTAMPTZ      NOT NULL DEFAULT now(),
    CONSTRAINT "walks_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "fk_walks_region_id" FOREIGN KEY ("region_id") REFERENCES "regions" ("id") ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS "idx_walks_name" ON "walks" ("name");
CREATE INDEX IF NOT EXISTS "idx_walks_name_like" ON "walks" USING gin ("name" gin_trgm_ops);
CREATE INDEX IF NOT EXISTS "idx_walks_region_id" ON "walks" ("region_id");
CREATE INDEX IF NOT EXISTS "idx_walks_difficulty" ON "walks" ("difficulty");;

CREATE OR REPLACE TRIGGER "tgr_walks_update_at"
    BEFORE UPDATE
    ON "walks"
    FOR EACH ROW
EXECUTE PROCEDURE moddatetime("updated_at");


-- migrate:down
DROP TABLE IF EXISTS "walks";
