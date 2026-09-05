use crate::features::regions::rows;

pub async fn get_region_by_code<'c, E>(executor: E, code: &str) -> Result<Option<rows::RegionRow>, sqlx::Error>
where
    E: sqlx::Executor<'c, Database=sqlx::Postgres>,
{
    sqlx::query_file_as!(rows::RegionRow, "sql/regions/select_by_code.sql", code)
        .fetch_optional(executor)
        .await
}
