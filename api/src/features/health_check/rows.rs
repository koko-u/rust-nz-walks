#[derive(Debug, serde::Serialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct HealthCheckRow {
    pub database: String,
    pub active_connections: i64,
    pub idle_connections: i64,
    pub total_connections: i64,
    pub max_connections: i32,
}
