use crate::features::walks::commands;

pub async fn get_count_by_filters<'c, E>(
    executor: E,
    command: &commands::QueryCommand,
) -> Result<i64, sqlx::Error>
where
    E: sqlx::Executor<'c, Database=sqlx::Postgres>,
{
    let name_contains = command.name_contains.as_ref();
    let difficulty = command.difficulty;
    let region_id = command.region_id.map(|region_id| region_id.into_inner());

    sqlx::query_file_scalar!(
        "sql/walks/count_by_filters.sql",
        name_contains,
        difficulty as _,
        region_id
    )
        .fetch_one(executor)
        .await
}
