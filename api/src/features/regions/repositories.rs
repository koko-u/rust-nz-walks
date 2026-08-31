use crate::features::regions::commands;
use crate::features::regions::models;
use crate::features::regions::rows;

pub async fn get_all_regions<'c, E>(executor: E) -> Result<Vec<rows::RegionRow>, sqlx::Error>
where
    E: sqlx::Executor<'c, Database=sqlx::Postgres>,
{
    sqlx::query_file_as!(rows::RegionRow, "sql/regions/select_all.sql")
        .fetch_all(executor)
        .await
}

pub async fn get_single_region<'c, E>(
    executor: E,
    id: models::RegionId,
) -> Result<Option<rows::RegionRow>, sqlx::Error>
where
    E: sqlx::Executor<'c, Database=sqlx::Postgres>,
{
    let id = id.into_inner();
    sqlx::query_file_as!(rows::RegionRow, "sql/regions/select_by_id.sql", id)
        .fetch_optional(executor)
        .await
}

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
