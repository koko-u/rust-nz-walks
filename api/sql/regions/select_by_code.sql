SELECT "id", "code", "name", "image_url"
FROM "regions"
WHERE "code" = $1::varchar;