WITH "params" ("name",
               "description",
               "length_km",
               "image_url",
               "region_id",
               "region_code",
               "difficulty"
    ) AS (SELECT $1::varchar,
                 $2::text,
                 $3::double precision,
                 $4::varchar,
                 "id"   AS "region_id",
                 "code" AS "region_code",
                 $6::difficulty
          FROM "regions"
          WHERE "code" = $5::varchar)
INSERT
INTO "walks" ("name",
              "description",
              "length_km",
              "image_url",
              "region_id",
              "difficulty")
SELECT "P"."name",
       "P"."description",
       "P"."length_km",
       "P"."image_url",
       "P"."region_id",
       "P"."difficulty"
FROM "params" AS "P"
RETURNING "id";

