use crate::features::regions::commands;
use crate::features::regions::rows;

pub async fn update_region<'c, E>(
    executor: E,
    data: commands::UpdateCommand,
) -> Result<Option<rows::RegionRow>, sqlx::Error>
where
    E: sqlx::Executor<'c, Database=sqlx::Postgres>,
{
    let id = data.id.into_inner();
    sqlx::query_file_as!(
        rows::RegionRow,
        "sql/regions/update_by_id.sql",
        id,
        &data.code,
        &data.name,
        data.image_url.as_ref(),
    )
        .fetch_optional(executor)
        .await
}
