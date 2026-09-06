use axum_keycloak_auth::decode;

use crate::errors;
use crate::features::me::dto;
use crate::shared;

#[utoipa::path(
    get,
    path = "/api/me",
    tag = "Users",
    responses(
        (status = 200, description = "Good Authentication", body = dto::MeResponse),
        (status = 500, description = "Internal Server Error", body = shared::ProblemDetails)
    )
)]
pub async fn get_me_info(
    axum::Extension(token): axum::Extension<decode::KeycloakToken<shared::AuthRole>>,
) -> Result<axum::Json<dto::MeResponse>, errors::ApiError> {
    let roles = token.roles.iter().map(|r| r.role()).cloned().collect();
    Ok(axum::Json(dto::MeResponse {
        subject: token.subject,
        username: token.extra.profile.preferred_username,
        roles,
    }))
}
