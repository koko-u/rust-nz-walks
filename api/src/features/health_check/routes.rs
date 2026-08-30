use crate::features::health_check::handlers;
use crate::state;
use axum::routing;

pub fn health_check_router() -> axum::Router<state::AppState>
{
    axum::Router::new()
        .route("/", routing::get(handlers::health_check))
}