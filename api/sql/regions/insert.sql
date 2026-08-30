WITH "params" ("code",
               "name",
               "image_url") AS (VALUES ($1::VARCHAR,
                                        $2::VARCHAR,
                                        $3::VARCHAR))
INSERT
INTO "regions" ("code", "name", "image_url")
SELECT "code", "name", "image_url"
FROM "params"
RETURNING "id", "code", "name", "image_url"