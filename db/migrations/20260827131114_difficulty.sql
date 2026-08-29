-- migrate:up
DO
$$
    BEGIN
        IF NOT EXISTS (SELECT 1
                       FROM "pg_type" t
                                INNER JOIN
                            "pg_namespace" n
                            ON
                                n.oid = t.typnamespace
                       WHERE t.typname = 'difficulty'
                         AND n.nspname = 'public') THEN
            CREATE TYPE "public"."difficulty" AS ENUM ('easy', 'medium', 'hard', 'expert');
        END IF;
    END
$$;

-- migrate:down
DROP TYPE IF EXISTS "public"."difficulty";
