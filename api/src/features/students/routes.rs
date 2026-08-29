use axum::routing;

use super::handlers;

pub fn students_router<S>() -> axum::Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    axum::Router::new().route("/", routing::get(handlers::get_all_students))
}
