use crate::features::walks::{models, rows};

pub async fn get_walk_by_id<'c, E>(
    executor: E,
    id: models::WalkId,
) -> Result<Option<rows::WalkRow>, sqlx::Error>
where
    E: sqlx::Executor<'c, Database=sqlx::Postgres>,
{
    let id = id.into_inner();
    sqlx::query_file_as!(rows::WalkRow, "sql/walks/select_by_id.sql", id)
        .fetch_optional(executor)
        .await
}
