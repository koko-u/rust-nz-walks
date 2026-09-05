use axum::extract;

use crate::errors;
use crate::features::regions::repositories as r_repositories;
use crate::features::walks::dto;
use crate::features::walks::models;
use crate::features::walks::repositories;
use crate::features::walks::requests;
use crate::shared;
use crate::shared::responses;
use crate::state;

#[utoipa::path(
    post,
    path = "/api/walks",
    request_body = requests::CreateRequest,
    tag = "Walks",
    responses(
        (status = 201, description = "Created new walk", body = dto::WalkDto),
        (status = 400, description = "Validation Error", body = shared::ProblemDetails),
        (status = 500, description = "Internal Server Error", body = shared::ProblemDetails)
    )
)]
pub async fn create_walk(
    extract::State(s): extract::State<state::AppState>,
    extract::Json(request): extract::Json<requests::CreateRequest>,
) -> Result<responses::Created<dto::WalkDto>, errors::ApiError> {

    // validate request data
    let command = request.validate_into()?;

    // insert new region
    let new_walk = {
        let mut tx = s.pool.begin().await?;

        // validate region code existence
        let regions = r_repositories::get_region_by_code(tx.as_mut(), &command.region_code).await?;
        if regions.is_none() {
            let mut report = garde::Report::new();
            report.append(
                garde::Path::new("region_code"),
                garde::error::Error::new(format!(
                    "Region code {codes} does not exist",
                    codes = command.region_code
                )),
            );
            return Err(errors::ApiError::Validation(report));
        }

        let id = repositories::create_walk(tx.as_mut(), command).await?;
        let row = repositories::get_walk_by_id(tx.as_mut(), id)
            .await?
            .expect("Cannot fetch walk data. but just register it.");

        tx.commit().await?;

        models::Walk::from(row)
    };

    let walk_dto = dto::WalkDto::from(new_walk);
    Ok(responses::created(
        format!("/api/walks/{id}", id = walk_dto.id),
        walk_dto,
    ))
}
