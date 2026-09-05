use axum::extract;

use crate::errors;
use crate::features::walks::dto;
use crate::features::walks::models;
use crate::features::walks::repositories;
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "/api/walks/{id}",
    params(
        ("id" = models::WalkId, Path, description = "Walk Id"),
    ),
    tag = "Walks",
    responses(
        (status = 200, description = "Returns a single walk", body = dto::WalkDto),
        (status = 404, description = "the walk not found", body = String),
        (status = 500, description = "Internal Server Error", body = shared::ProblemDetails)
    )
)]
pub async fn get_walk_by_id(
    extract::Path(id): extract::Path<models::WalkId>,
    extract::State(s): extract::State<state::AppState>,
) -> Result<axum::Json<dto::WalkDto>, errors::ApiError> {
    let mut conn = s.pool.acquire().await?;
    let row = repositories::get_walk_by_id(conn.as_mut(), id).await?;

    match row {
        Some(row) => {
            let walk = models::Walk::from(row);
            let dto = dto::WalkDto::from(walk);
            Ok(axum::Json(dto))
        }
        None => Err(errors::ApiError::NotFound {
            message: format!("Walk of id = {id} not found"),
        }),
    }
}
