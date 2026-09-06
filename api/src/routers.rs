use axum_keycloak_auth::layer::KeycloakAuthLayer;
use axum_keycloak_auth::role;

use crate::features::health_check;
use crate::features::me;
use crate::features::regions;
use crate::features::walks;
use crate::state;

pub fn app_router<R>(auth_layer: KeycloakAuthLayer<R>) -> axum::Router<state::AppState>
where
    R: role::Role + 'static,
{
    let protected_routes = axum::Router::new()
        .nest("/me", me::routes::me_router())
        .nest("/regions", regions::routes::regions_router())
        .nest("/walks", walks::routes::walks_router());
    let protected_routes = protected_routes.layer(auth_layer);

    axum::Router::new()
        .nest("/api", protected_routes)
        .nest("/health-check", health_check::routes::health_check_router())
}
