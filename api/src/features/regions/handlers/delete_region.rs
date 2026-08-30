use axum::extract;
use axum::http;

use crate::errors;
use crate::features::regions::models;
use crate::features::regions::repositories;
use crate::shared;
use crate::state;

#[utoipa::path(
    delete,
    path = "/api/regions/{id}",
    params(
        ("id" = models::RegionId, Path, description = "Region ID")
    ),
    tag = "Regions",
    responses(
        (status = 204, description = "Deleted the region"),
        (status = 404, description = "Not Found", body = String),
        (status = 500, description = "Internal Server Error", body = shared::ProblemDetails)
    )
)]
pub async fn delete_region(
    extract::Path(id): extract::Path<models::RegionId>,
    extract::State(s): extract::State<state::AppState>,
) -> Result<http::StatusCode, errors::ApiError> {
    // update the region
    let deleted = {
        let mut tx = s.pool.begin().await?;
        let row = repositories::delete_region(tx.as_mut(), id).await?;

        tx.commit().await?;

        row.map(models::Region::from)
    };

    match deleted {
        Some(_) => Ok(http::StatusCode::NO_CONTENT),
        None => Err(errors::ApiError::NotFound {
            message: format!("Region of id = {id} is not found"),
        }),
    }
}
