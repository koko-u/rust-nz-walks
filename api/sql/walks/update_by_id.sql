WITH "params" ("id",
               "name",
               "description",
               "length",
               "image_url",
               "region_id",
               "difficulty") AS (SELECT $1::uuid,
                                        $2::VARCHAR,
                                        $3::TEXT,
                                        $4::double precision,
                                        $5::VARCHAR,
                                        "R"."id",
                                        $7::difficulty
                                 FROM "regions" AS "R"
                                 WHERE "R"."code" = $6::VARCHAR)
UPDATE "walks" AS "W"
SET "name"        = "P"."name",
    "description" = "P"."description",
    "length_km"   = "P"."length",
    "image_url"   = "P"."image_url",
    "region_id"   = "P"."region_id",
    "difficulty"  = "P"."difficulty"
FROM "params" AS "P"
WHERE "W"."id" = "P"."id"
RETURNING "W"."id";
