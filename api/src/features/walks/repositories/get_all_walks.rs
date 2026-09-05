use crate::features::walks::rows;

pub async fn get_all_walks<'c, E>(
    executor: E,
) -> Result<Vec<rows::WalkRow>, sqlx::Error>
where
    E: sqlx::Executor<'c, Database=sqlx::Postgres>,
{
    sqlx::query_file_as!(rows::WalkRow, "sql/walks/select_all.sql")
        .fetch_all(executor)
        .await
}
