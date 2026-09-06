-- $1::bigint - limit
-- $2::bigint - offset
-- $3::varchar - name_contains
-- $4::difficulty - difficulty
-- $5::uuid - region_id
--
SELECT "W"."id",
       "W"."name",
       "W"."description",
       "W"."length_km",
       "W".image_url,
       "W"."region_id",
       "R"."code"      AS "region_code",
       "R"."name"      AS "region_name",
       "R"."image_url" AS "region_image_url",
       "W".difficulty  AS "difficulty: _"
FROM "walks" AS "W"
         INNER JOIN
     "regions" AS "R"
     ON
         "W"."region_id" = "R"."id"
WHERE ($3::varchar IS NULL OR "W"."name" ILIKE '%' || $3::varchar || '%')
  AND ($4::difficulty IS NULL OR "W"."difficulty" = $4::difficulty)
  AND ($5::uuid IS NULL OR "W"."region_id" = $5::uuid)

