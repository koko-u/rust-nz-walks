use crate::features::regions::models;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UpdateCommand {
    pub id: models::RegionId,
    pub code: String,
    pub name: String,
    pub image_url: Option<String>,
}