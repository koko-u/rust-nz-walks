use crate::features::regions::models;
use crate::features::regions::rows;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct Region {
    pub id: models::RegionId,
    pub code: String,
    pub name: String,
    pub image_url: Option<String>,
}

impl From<rows::RegionRow> for Region {
    fn from(row: rows::RegionRow) -> Self {
        Self {
            id: row.id.into(),
            code: row.code,
            name: row.name,
            image_url: row.image_url,
        }
    }
}
