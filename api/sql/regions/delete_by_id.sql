DELETE
FROM "regions"
WHERE "id" = $1
RETURNING "id", "code", "name", "image_url";