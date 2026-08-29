-- migrate:up
CREATE TABLE IF NOT EXISTS "regions"
(
    "id"         UUID          NOT NULL DEFAULT uuidv7(),
    "code"       VARCHAR(100)  NOT NULL,
    "name"       VARCHAR(255)  NOT NULL,
    "image_url"  VARCHAR(2048) NULL,
    "created_at" TIMESTAMPTZ   NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMPTZ   NOT NULL DEFAULT now(),
    CONSTRAINT "regions_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "uq_regions_code" UNIQUE ("code")
);

CREATE INDEX IF NOT EXISTS "idx_regions_name" ON "regions" ("name");
CREATE INDEX IF NOT EXISTS "idx_regions_name_like" ON "regions" USING gin ("name" gin_trgm_ops);

CREATE OR REPLACE TRIGGER "tgr_regions_update_at"
    BEFORE UPDATE
    ON "regions"
    FOR EACH ROW
EXECUTE PROCEDURE moddatetime("updated_at");

-- migrate:down
DROP TABLE IF EXISTS "regions";
