use axum::response;

#[utoipa::path(
    get,
    path = "/api/students",
    tag = "students",
    responses(
        (status = 200, description = "Returns a list of students", body = Vec<String>)
    )
)]
pub async fn get_all_students() -> impl response::IntoResponse {
    axum::Json(vec!["John", "Jane", "Mark", "Emily", "David"])
}
