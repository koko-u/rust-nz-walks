#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, garde::Validate, utoipa::ToSchema)]
pub struct CreateRegion {
    #[garde(alphanumeric, length(min = 1, max = 100))]
    pub code: String,
    #[garde(length(min = 1, max = 255))]
    pub name: String,
    #[garde(url)]
    pub image_url: Option<String>,
}