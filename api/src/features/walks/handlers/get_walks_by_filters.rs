use axum::extract;

use crate::errors;
use crate::features::walks::dto;
use crate::features::walks::models;
use crate::features::walks::repositories;
use crate::features::walks::requests;
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "/api/walks",
    description = "Get Walks data with query parameter's filter",
    tag = "Walks",
    params(requests::GetWalksQuery),
    responses(
        (status = 200, description = "Returns a list of walks", body = shared::PagingResponse<dto::WalkDto>),
        (status = 500, description = "Internal Server Error", body = shared::ProblemDetails)
    )
)]
pub async fn get_walks_by_filters(
    extract::State(s): extract::State<state::AppState>,
    extract::Query(query): extract::Query<requests::GetWalksQuery>,
) -> Result<axum::Json<shared::PagingResponse<dto::WalkDto>>, errors::ApiError> {
    // validate query parameter
    let command = query.validate_into(&s.pool).await?;

    let mut conn = s.pool.acquire().await?;

    let rows = repositories::get_walks_by_filters(conn.as_mut(), &command).await?;
    let total = repositories::get_count_by_filters(conn.as_mut(), &command).await?;
    let pages = (total as f64 / command.per_page as f64).ceil() as u32;

    let walks = rows
        .into_iter()
        .map(models::Walk::from)
        .map(dto::WalkDto::from)
        .collect::<Vec<_>>();

    let response = shared::PagingResponse {
        current_page: command.page,
        page_size: command.per_page,
        total: total as u32,
        pages,
        items: walks,
    };

    Ok(axum::Json(response))
}
