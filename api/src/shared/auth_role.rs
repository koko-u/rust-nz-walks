use axum_keycloak_auth::role;

#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    derive_more::Display,
    derive_more::IsVariant,
    serde::Serialize,
    utoipa::ToSchema,
    strum::EnumIter,
)]
#[serde(rename_all = "snake_case")]
pub enum AuthRole {
    #[display("admin")]
    Admin,
    #[display("user")]
    User,
    #[display("guest")]
    Guest,
    #[display("unknown: {}", _0)]
    #[serde(untagged)]
    Unknown(String),
}

impl From<String> for AuthRole {
    fn from(value: String) -> Self {
        match value.to_ascii_lowercase().as_str() {
            "admin" => Self::Admin,
            "user" => Self::User,
            "guest" => Self::Guest,
            _ => Self::Unknown(value),
        }
    }
}

impl role::Role for AuthRole {}