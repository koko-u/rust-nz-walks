use crate::features::walks::commands;
use crate::features::walks::rows;

pub async fn get_walks_by_filters<'c, E>(
    executor: E,
    command: &commands::QueryCommand,
) -> Result<Vec<rows::WalkRow>, sqlx::Error>
where
    E: sqlx::Executor<'c, Database=sqlx::Postgres>,
{
    tracing::info!(command = ?command, "filter");
    let page = command.page as i64;
    let per_page = command.per_page as i64;
    let name_contains = command.name_contains.as_ref();
    let difficulty = command.difficulty;
    let region_id = command.region_id.map(|region_id| region_id.into_inner());

    let limit = per_page;
    let offset = (page - 1) * per_page;

    let mut builder = builders::GetWalksByFiltersQueryBuilder::new();

    builder.apply_where_clause(name_contains, difficulty, region_id);
    builder.apply_order_by(&command.sort_order);

    builder.push(" LIMIT ");
    builder.push_bind(limit);

    builder.push(" OFFSET ");
    builder.push_bind(offset);

    builder
        .build_query_as::<rows::WalkRow>()
        .fetch_all(executor)
        .await
}

mod builders;
