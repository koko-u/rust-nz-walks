use crate::features::walks::commands;
use crate::features::walks::models;

const SQL_WALKS_SELECT_BY_FILTERS: &str = include_str!(concat!(
env!("CARGO_MANIFEST_DIR"),
"/sql/walks/_select_by_filters.sql"
));

#[derive(derive_more::Deref, derive_more::DerefMut)]
pub struct GetWalksByFiltersQueryBuilder {
    builder: sqlx::QueryBuilder<sqlx::Postgres>,
}

impl GetWalksByFiltersQueryBuilder {
    pub fn new() -> Self {
        Self {
            builder: sqlx::QueryBuilder::new(SQL_WALKS_SELECT_BY_FILTERS),
        }
    }

    pub fn apply_where_clause(
        &mut self,
        name_contains: Option<&String>,
        difficulty: Option<models::Difficulty>,
        region_id: Option<uuid::Uuid>,
    ) {
        if name_contains.is_none() && difficulty.is_none() && region_id.is_none() {
            return;
        }

        self.builder.push(" WHERE ");

        let mut where_clause = self.builder.separated(" AND ");
        if let Some(name_contains) = name_contains {
            where_clause.push(r#""W"."name" ILIKE "#);
            where_clause.push_bind_unseparated(format!("%{name_contains}%"));
        }
        if let Some(difficulty) = difficulty {
            where_clause.push(r#""W"."difficulty" = "#);
            where_clause.push_bind_unseparated(difficulty);
        }
        if let Some(region_id) = region_id {
            where_clause.push(r#""W"."region_id" = "#);
            where_clause.push_bind_unseparated(region_id);
        }
    }

    pub fn apply_order_by(&mut self, sort_order: &[commands::Sort]) {
        if sort_order.is_empty() {
            return;
        }

        self.builder.push(" ORDER BY ");

        let mut separated = self.builder.separated(", ");

        for sort in sort_order {
            let column = match sort.field {
                commands::SortField::Name => r#""W"."name" "#,
                commands::SortField::Length => r#""W"."length_km" "#,
                commands::SortField::RegionCode => r#""R"."code" "#,
                commands::SortField::Difficulty => r#""W"."difficulty" "#,
            };

            let direction = match sort.direction {
                commands::SortDirection::Asc => "ASC",
                commands::SortDirection::Desc => "DESC",
            };

            separated.push(column);
            separated.push_unseparated(direction);
        }

        separated.push(r#""W"."id" ASC"#);
    }
}

