use axum::routing;

use crate::features::walks::handlers;
use crate::state;

pub fn walks_router() -> axum::Router<state::AppState> {
    axum::Router::new()
        .route(
            "/",
            routing::MethodRouter::new()
                .get(handlers::get_walks_by_filters)
                .post(handlers::create_walk),
        )
        .route(
            "/{id}",
            routing::MethodRouter::new()
                .get(handlers::get_walk_by_id)
                .put(handlers::update_walk)
                .delete(handlers::delete_by_id),
        )
}
