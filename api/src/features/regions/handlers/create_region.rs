use axum::extract;
use garde::Validate;

use crate::errors;
use crate::features::regions::commands;
use crate::features::regions::models;
use crate::features::regions::repositories;
use crate::features::regions::requests;
use crate::shared;
use crate::shared::responses;
use crate::state;

#[utoipa::path(
    post,
    path = "/api/regions",
    request_body = requests::CreateRegion,
    tag = "Regions",
    responses(
        (status = 201, description = "Created new region", body = models::Region),
        (status = 400, description = "Validation Error", body = shared::ProblemDetails),
        (status = 500, description = "Internal Server Error", body = shared::ProblemDetails)
    )
)]
pub async fn create_region(
    extract::State(s): extract::State<state::AppState>,
    extract::Json(request): extract::Json<requests::CreateRegion>,
) -> Result<responses::Created<models::Region>, errors::ApiError> {
    // validate request data
    request.validate()?;
    let command = commands::CreateCommand {
        code: request.code,
        name: request.name,
        image_url: request.image_url,
    };

    // insert new region
    let new_region = {
        let mut tx = s.pool.begin().await?;
        let row = repositories::create_region(tx.as_mut(), command).await?;

        tx.commit().await?;

        models::Region::from(row)
    };

    Ok(responses::created(
        format!("/api/regions/{id}", id = new_region.id),
        new_region,
    ))
}
