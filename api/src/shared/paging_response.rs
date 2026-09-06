#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
pub struct PagingResponse<T> {
    pub current_page: u32,
    pub page_size: u32,
    pub total: u32,
    pub pages: u32,
    pub items: Vec<T>,
}