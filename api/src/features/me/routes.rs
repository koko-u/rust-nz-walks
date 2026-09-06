use axum::routing;

use crate::features::me::handlers;
use crate::state;

pub fn me_router() -> axum::Router<state::AppState> {
    axum::Router::new().route("/", routing::get(handlers::get_me_info))
}
