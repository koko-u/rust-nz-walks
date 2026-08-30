use crate::features::health_check::rows;

pub async fn get_database_status<'c, E>(executor: E) -> Result<rows::HealthCheckRow, sqlx::Error>
where
    E: sqlx::Executor<'c, Database=sqlx::Postgres>,
{
    sqlx::query_as!(
        rows::HealthCheckRow,
        r#"
        SELECT
            "datname" AS "database!",
            COUNT(*) FILTER (WHERE state = 'active') AS "active_connections!",
            COUNT(*) FILTER (WHERE state = 'idle') AS "idle_connections!",
            COUNT(*) AS "total_connections!",
            (SELECT setting::int FROM "pg_settings" WHERE "name" = 'max_connections') AS "max_connections!"
        FROM "pg_stat_activity"
        WHERE "datname" IS NOT NULL
        GROUP BY "datname"
        ORDER BY "total_connections!" DESC
        "#
    )
        .fetch_one(executor)
        .await
}
