use axum::extract;

use crate::errors;
use crate::features::regions::models;
use crate::features::regions::repositories;
use crate::shared;
use crate::state;

#[utoipa::path(
    get,
    path = "/api/regions/{id}",
    params(
        ("id" = models::RegionId, Path, description = "Region Id"),
    ),
    tag = "Regions",
    responses(
        (status = 200, description = "Returns a list of regions", body = Vec<models::Region>),
        (status = 404, description = "Region not found", body = String),
        (status = 500, description = "Internal Server Error", body = shared::ProblemDetails)
    )
)]
pub async fn get_single_region(
    extract::Path(id): extract::Path<models::RegionId>,
    extract::State(s): extract::State<state::AppState>,
) -> Result<axum::Json<models::Region>, errors::ApiError> {
    let mut conn = s.pool.acquire().await?;
    let row = repositories::get_single_region(conn.as_mut(), id).await?;

    match row {
        Some(row) => {
            let region = models::Region::from(row);
            Ok(axum::Json(region))
        }
        None => Err(errors::ApiError::NotFound {
            message: format!("Region of id = {id} not found"),
        }),
    }
}
