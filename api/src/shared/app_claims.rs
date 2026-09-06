#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize)]
pub struct AppClaims {
    #[serde(rename = "sub")]
    pub subject: String,

    #[serde(default)]
    pub preferred_username: Option<String>,
}