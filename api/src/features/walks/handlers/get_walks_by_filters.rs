use axum::extract;

use crate::errors;
use crate::features::walks::models;
use crate::features::walks::repositories;
use crate::features::walks::{dto, requests};
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "/api/walks",
    tag = "Walks",
    params(requests::GetWalksQuery),
    responses(
        (status = 200, description = "Returns a list of walks", body = Vec<dto::WalkDto>),
        (status = 500, description = "Internal Server Error", body = shared::ProblemDetails)
    )
)]
pub async fn get_walks_by_filters(
    extract::State(s): extract::State<state::AppState>,
    extract::Query(query): extract::Query<requests::GetWalksQuery>,
) -> Result<axum::Json<Vec<dto::WalkDto>>, errors::ApiError> {
    // validate query parameter
    let command = query.validate_into(&s.pool).await?;

    let mut conn = s.pool.acquire().await?;

    let rows = repositories::get_walks_by_filters(conn.as_mut(), &command).await?;

    let walks = rows
        .into_iter()
        .map(models::Walk::from)
        .map(dto::WalkDto::from)
        .collect();

    Ok(axum::Json(walks))
}
