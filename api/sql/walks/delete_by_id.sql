WITH "row" AS (SELECT "W"."id",
                      "R"."id"        AS "region_id",
                      "R"."code"      AS "region_code",
                      "R"."name"      AS "region_name",
                      "R"."image_url" AS "region_image_url"
               FROM "regions" AS "R"
                        LEFT OUTER JOIN
                        (SELECT * FROM "walks" WHERE "id" = $1::uuid) AS "W"
                        ON
                            "R"."id" = "W"."region_id")
DELETE
FROM "walks" AS "W"
    USING "row" AS "R"
WHERE "W"."id" = "R"."id"
RETURNING "W"."id",
    "W"."name",
    "W"."description",
    "W"."length_km",
    "W"."image_url",
    "W"."region_id",
    "R"."region_code",
    "R"."region_name",
    "R"."region_image_url",
    "W"."difficulty" AS "difficulty: _";

/*
        "R"."code"      AS "region_code",
       "R"."name"      AS "region_name",
       "R"."image_url" AS "region_image_url",

 */