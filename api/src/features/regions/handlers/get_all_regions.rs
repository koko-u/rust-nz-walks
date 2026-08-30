use crate::features::regions::{models, repositories};
use crate::{errors, shared, state};
use axum::extract;

#[utoipa::path(
    get,
    path = "/api/regions",
    tag = "Regions",
    responses(
        (status = 200, description = "Returns a list of regions", body = Vec<models::Region>),
        (status = 500, description = "Internal Server Error", body = shared::ProblemDetails)
    )
)]
pub async fn get_all_regions(
    extract::State(s): extract::State<state::AppState>,
) -> Result<axum::Json<Vec<models::Region>>, errors::ApiError> {
    let mut conn = s.pool.acquire().await?;
    let rows = repositories::get_all_regions(conn.as_mut()).await?;

    let regions = rows.into_iter().map(models::Region::from).collect();

    Ok(axum::Json(regions))
}
