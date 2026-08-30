use axum::extract;
use garde::Validate;

use crate::errors;
use crate::features::regions::commands;
use crate::features::regions::models;
use crate::features::regions::repositories;
use crate::features::regions::requests;
use crate::shared;
use crate::state;

#[utoipa::path(
    put,
    path = "/api/regions/{id}",
    params(
        ("id" = models::RegionId, Path, description = "Region ID")
    ),
    request_body = requests::UpdateRegion,
    tag = "Regions",
    responses(
        (status = 200, description = "Updated the region", body = models::Region),
        (status = 400, description = "Validation Error", body = shared::ProblemDetails),
        (status = 500, description = "Internal Server Error", body = shared::ProblemDetails)
    )
)]
pub async fn update_region(
    extract::Path(id): extract::Path<models::RegionId>,
    extract::State(s): extract::State<state::AppState>,
    extract::Json(request): extract::Json<requests::UpdateRegion>,
) -> Result<axum::Json<models::Region>, errors::ApiError> {
    // validate request data
    request.validate()?;
    let command = commands::UpdateCommand {
        id: id.into(),
        code: request.code,
        name: request.name,
        image_url: request.image_url,
    };

    // update the region
    let updated = {
        let mut tx = s.pool.begin().await?;
        let row = repositories::update_region(tx.as_mut(), command).await?;

        tx.commit().await?;

        row.map(models::Region::from)
    };

    match updated {
        Some(region) => Ok(axum::Json(region)),
        None => Err(errors::ApiError::NotFound {
            message: format!("Region of id = {id} is not found"),
        }),
    }
}
