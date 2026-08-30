use axum::extract;

use crate::errors;
use crate::features::health_check::repositories;
use crate::features::health_check::rows;
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "/health-check",
    tag = "Health Check",
    responses(
        (status = 200, description = "Hello World", body = rows::HealthCheckRow),
        (status = 500, description = "Internal Server Error", body = shared::ProblemDetails)
    )
)]
pub async fn health_check(
    extract::State(s): extract::State<state::AppState>,
) -> Result<axum::Json<rows::HealthCheckRow>, errors::ApiError> {
    let mut conn = s.pool.acquire().await?;
    let row = repositories::get_database_status(conn.as_mut()).await?;
    Ok(axum::Json(row))
}
