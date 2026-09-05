use crate::features::walks::commands;
use crate::features::walks::models;

pub async fn create_walk<'c, E>(
    executor: E,
    data: commands::CreateCommand,
) -> Result<models::WalkId, sqlx::Error>
where
    E: sqlx::Executor<'c, Database=sqlx::Postgres>,
{
    let id = sqlx::query_file_scalar!(
        "sql/walks/insert.sql",
        &data.name,
        data.description.as_ref(),
        data.length,
        data.image_url.as_ref(),
        &data.region_code,
        data.difficulty as _
    )
        .fetch_one(executor)
        .await?;

    Ok(id.into())
}
