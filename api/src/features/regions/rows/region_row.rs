#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RegionRow {
    pub id: uuid::Uuid,
    pub code: String,
    pub name: String,
    pub image_url: Option<String>,
}