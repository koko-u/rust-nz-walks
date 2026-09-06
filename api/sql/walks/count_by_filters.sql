-- $1::varchar - name_contains
-- $2::difficulty - difficulty
-- $3::uuid - region_id
--
SELECT COUNT(*) AS "count!"
FROM "walks" AS "W"
WHERE ($1::varchar IS NULL OR "W"."name" ILIKE '%' || $1::varchar || '%')
  AND ($2::difficulty IS NULL OR "W"."difficulty" = $2::difficulty)
  AND ($3::uuid IS NULL OR "W"."region_id" = $3::uuid)
;
