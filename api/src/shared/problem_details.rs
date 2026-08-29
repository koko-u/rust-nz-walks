use std::borrow::Borrow;
use std::collections;

use axum::http;

#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
pub struct ProblemDetails {
    pub title: String,
    #[serde(serialize_with = "se_status")]
    #[schema(value_type = String)]
    pub status: http::StatusCode,
    pub detail: Option<String>,
    pub errors: FieldErrors,
}

impl<R> From<R> for ProblemDetails
where
    R: Borrow<garde::Report>,
{
    fn from(report: R) -> Self {
        let report = report.borrow();
        let field_errors = report.iter().fold(
            collections::HashMap::new(),
            |mut hash_map, (field, error)| {
                hash_map
                    .entry(field.to_string())
                    .and_modify(|e: &mut Vec<_>| e.push(error.to_string()))
                    .or_insert(vec![error.to_string()]);
                hash_map
            },
        ); //.map(|(field, error)| (field.to_string(), error.to_string())));

        let problem_details = ProblemDetails {
            title: "Validation error".to_string(),
            status: http::StatusCode::BAD_REQUEST,
            detail: Some(report.to_string()),
            errors: field_errors.into(),
        };

        problem_details
    }
}

#[derive(Debug, Clone, serde::Serialize, derive_more::From, utoipa::ToSchema)]
pub struct FieldErrors(collections::HashMap<String, Vec<String>>);

fn se_status<S>(status: &http::StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(status.as_str())
}
