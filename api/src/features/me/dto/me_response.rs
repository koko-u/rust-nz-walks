use crate::shared;

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, utoipa::ToSchema)]
pub struct MeResponse {
    pub subject: String,
    pub username: String,
    pub roles: Vec<shared::AuthRole>,
}