WITH "params" ("id",
               "code",
               "name",
               "image_url"
    ) AS (VALUES ($1::uuid,
                  $2::VARCHAR,
                  $3::VARCHAR,
                  $4::VARCHAR))
UPDATE "regions" AS "R"
SET "code"      = "P"."code",
    "name"      = "P"."name",
    "image_url" = "P"."image_url"
FROM "params" AS "P"
WHERE "R"."id" = "P"."id"
RETURNING "R"."id",
    "R"."code",
    "R"."name",
    "R"."image_url";