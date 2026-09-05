use crate::features::regions::models;
use crate::features::regions::rows;

pub async fn delete_region<'c, E>(
    executor: E,
    id: models::RegionId,
) -> Result<Option<rows::RegionRow>, sqlx::Error>
where
    E: sqlx::Executor<'c, Database=sqlx::Postgres>,
{
    let id = id.into_inner();

    sqlx::query_file_as!(rows::RegionRow, "sql/regions/delete_by_id.sql", id)
        .fetch_optional(executor)
        .await
}
