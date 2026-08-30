use crate::features::health_check;
use crate::features::regions;
use crate::state;

pub fn app_router() -> axum::Router<state::AppState> {
    axum::Router::new()
        .nest("/health-check", health_check::routes::health_check_router())
        .nest("/api/regions", regions::routes::regions_router())
}

