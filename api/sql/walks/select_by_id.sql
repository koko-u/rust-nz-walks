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
FROM (SELECT * FROM "walks" WHERE "id" = $1::uuid) AS "W"
         INNER JOIN
     "regions" AS "R"
     ON
         "W"."region_id" = "R"."id"