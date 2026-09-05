use crate::features::walks::commands;
use crate::features::walks::models;

pub async fn update_walk<'c, E>(
    execute: E,
    data: commands::UpdateCommand,
) -> Result<Option<models::WalkId>, sqlx::Error>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    let id = data.id.into_inner();
    let query_result = sqlx::query_file_scalar!(
        "sql/walks/update_by_id.sql",
        id,
        &data.name,
        data.description.as_ref(),
        data.length,
        data.image_url.as_ref(),
        &data.region_code,
        data.difficulty as _
    )
    .fetch_optional(execute)
    .await?;

    Ok(query_result.map(models::WalkId::from))
}
