use crate::features::regions::commands;
use crate::features::regions::rows;

pub async fn create_region<'c, E>(
    executor: E,
    data: commands::CreateCommand,
) -> Result<rows::RegionRow, sqlx::Error>
where
    E: sqlx::Executor<'c, Database=sqlx::Postgres>,
{
    sqlx::query_file_as!(
        rows::RegionRow,
        "sql/regions/insert.sql",
        &data.code,
        &data.name,
        data.image_url.as_ref()
    )
        .fetch_one(executor)
        .await
}
