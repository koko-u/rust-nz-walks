use crate::features::walks::models;
use crate::features::walks::rows;

pub async fn delete_walk_by_id<'c, E>(
    executor: E,
    id: models::WalkId,
) -> Result<Option<rows::WalkRow>, sqlx::Error>
where
    E: sqlx::Executor<'c, Database=sqlx::Postgres>,
{
    sqlx::query_file_as!(rows::WalkRow, "sql/walks/delete_by_id.sql", id.into_inner())
        .fetch_optional(executor)
        .await
}

// pub async fn delete_region<'c, E>(
//     executor: E,
//     id: models::RegionId,
// ) -> Result<Option<rows::RegionRow>, sqlx::Error>
// where
//     E: sqlx::Executor<'c, Database=sqlx::Postgres>,
// {
//     let id = id.into_inner();
//
//     sqlx::query_file_as!(rows::RegionRow, "sql/regions/delete_by_id.sql", id)
//         .fetch_optional(executor)
//         .await
// }
