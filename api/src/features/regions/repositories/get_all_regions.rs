use crate::features::regions::rows;

pub async fn get_all_regions<'c, E>(executor: E) -> Result<Vec<rows::RegionRow>, sqlx::Error>
where
    E: sqlx::Executor<'c, Database=sqlx::Postgres>,
{
    sqlx::query_file_as!(rows::RegionRow, "sql/regions/select_all.sql")
        .fetch_all(executor)
        .await
}
