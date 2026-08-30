use axum::routing;

use crate::features::regions::handlers;
use crate::state;

pub fn regions_router() -> axum::Router<state::AppState> {
    axum::Router::new()
        .route(
            "/",
            routing::MethodRouter::new()
                .get(handlers::get_all_regions)
                .post(handlers::create_region),
        )
        .route(
            "/{id}",
            routing::MethodRouter::new()
                .get(handlers::get_single_region)
                .put(handlers::update_region)
                .delete(handlers::delete_region),
        )
}
