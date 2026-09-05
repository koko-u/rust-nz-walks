use crate::features::walks::{models, repositories};
use crate::{errors, shared, state};
use axum::{extract, http};

#[utoipa::path(
    delete,
    path = "/api/walks/{id}",
    params(
        ("id" = models::WalkId, Path, description = "Walk ID")
    ),
    tag = "Walks",
    responses(
        (status = 204, description = "Deleted the walk"),
        (status = 404, description = "Not Found", body = String),
        (status = 500, description = "Internal Server Error", body = shared::ProblemDetails)
    )
)]
pub async fn delete_by_id(
    extract::Path(id): extract::Path<models::WalkId>,
    extract::State(s): extract::State<state::AppState>,
) -> Result<http::StatusCode, errors::ApiError> {
    // update the region
    let deleted = {
        let mut tx = s.pool.begin().await?;
        let row = repositories::delete_walk_by_id(tx.as_mut(), id).await?;

        tx.commit().await?;

        row.map(models::Walk::from)
    };

    match deleted {
        Some(_) => Ok(http::StatusCode::NO_CONTENT),
        None => Err(errors::ApiError::NotFound {
            message: format!("Walk of id = {id} is not found"),
        }),
    }
}
