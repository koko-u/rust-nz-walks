use axum::extract;

use crate::errors;
use crate::features::walks::dto;
use crate::features::walks::models;
use crate::features::walks::repositories;
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "/api/walks",
    tag = "Walks",
    responses(
        (status = 200, description = "Returns a list of walks", body = Vec<dto::WalkDto>),
        (status = 500, description = "Internal Server Error", body = shared::ProblemDetails)
    )
)]
pub async fn get_all_walks(
    extract::State(s): extract::State<state::AppState>,
) -> Result<axum::Json<Vec<dto::WalkDto>>, errors::ApiError> {
    let mut conn = s.pool.acquire().await?;
    let rows = repositories::get_all_walks(conn.as_mut()).await?;

    let walks = rows
        .into_iter()
        .map(models::Walk::from)
        .map(dto::WalkDto::from)
        .collect();

    Ok(axum::Json(walks))
}
