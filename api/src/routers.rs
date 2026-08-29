use axum::extract;
use axum::response;

use crate::features::students;
use crate::state;
use crate::{errors, shared};

pub fn app_router() -> axum::Router<state::AppState> {
    axum::Router::new()
        .route("/health-check", axum::routing::get(index))
        .nest("/api/students", students::students_router())
}

#[utoipa::path(
    get,
    path = "/health-check",
    tag = "Health Check",
    responses(
        (status = 200, description = "Hello World", body = Row),
        (status = 500, description = "Internal Server Error", body = shared::ProblemDetails)
    )
)]
pub async fn index(
    extract::State(s): extract::State<state::AppState>,
) -> Result<impl response::IntoResponse, errors::ApiError> {
    let mut conn = s.pool.acquire().await?;
    let row = sqlx::query_as!(
        Row,
        r#"
        SELECT
            "datname" AS "database!",
            COUNT(*) FILTER (WHERE state = 'active') AS "active_connections!",
            COUNT(*) FILTER (WHERE state = 'idle') AS "idle_connections!",
            COUNT(*) AS "total_connections!",
            (SELECT setting::int FROM "pg_settings" WHERE "name" = 'max_connections') AS "max_connections!"
        FROM "pg_stat_activity"
        WHERE "datname" IS NOT NULL
        GROUP BY "datname"
        ORDER BY "total_connections!" DESC
        "#
    )
        .fetch_one(conn.as_mut())
        .await?;

    Ok(axum::Json(row))
}

#[derive(Debug, serde::Serialize, sqlx::FromRow, utoipa::ToSchema)]
struct Row {
    database: String,
    active_connections: i64,
    idle_connections: i64,
    total_connections: i64,
    max_connections: i32,
}
