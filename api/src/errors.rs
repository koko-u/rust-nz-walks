use axum::response;

use crate::shared;

#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
pub enum ApiError {
    #[display("Database error: {}", _0)]
    DB(#[error(source)] sqlx::Error),
    #[display("Validation error: {}", _0)]
    Validation(#[error(source)] garde::Report),
}

impl response::IntoResponse for ApiError {
    fn into_response(self) -> response::Response {
        match &self {
            Self::DB(err) => {
                tracing::error!("Database error: {err:?}");
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
            Self::Validation(err) => {
                tracing::error!("Validation error: {err:?}");
                let problem_details = shared::ProblemDetails::from(err);
                (problem_details.status, axum::Json(problem_details)).into_response()
            }
        }
    }
}
