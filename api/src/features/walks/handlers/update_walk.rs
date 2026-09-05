use crate::features::walks::{dto, models, repositories, requests};
use crate::{errors, shared, state};
use axum::extract;

#[utoipa::path(
    put,
    path = "/api/walks/{id}",
    params(
        ("id" = models::WalkId, Path, description = "Walk ID")
    ),
    request_body = requests::UpdateRequest,
    tag = "Walks",
    responses(
        (status = 200, description = "Updated the walk", body = dto::WalkDto),
        (status = 400, description = "Validation Error", body = shared::ProblemDetails),
        (status = 500, description = "Internal Server Error", body = shared::ProblemDetails)
    )
)]
pub async fn update_walk(
    extract::Path(id): extract::Path<models::WalkId>,
    extract::State(s): extract::State<state::AppState>,
    extract::Json(request): extract::Json<requests::UpdateRequest>,
) -> Result<axum::Json<dto::WalkDto>, errors::ApiError> {
    // validate request data
    let command = request.validate_into(id)?;

    // update the region
    let updated = {
        let mut tx = s.pool.begin().await?;
        let mut row = None;
        if let Some(walk_id) = repositories::update_walk(tx.as_mut(), command).await? {
            row = repositories::get_walk_by_id(tx.as_mut(), walk_id).await?;
        }

        tx.commit().await?;

        row.map(models::Walk::from).map(dto::WalkDto::from)
    };

    match updated {
        Some(walk) => Ok(axum::Json(walk)),
        None => Err(errors::ApiError::NotFound {
            message: format!("Walk of id = {id} is not found"),
        }),
    }
}
