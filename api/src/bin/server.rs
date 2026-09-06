use api::routers;
use api::state;
use api::{config, shared};
use tower_http::trace;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
    let config = config::Config::new()?;

    let state = state::AppState::new(&config.database_url(), config.max_connections()).await?;
    let auth_layer = shared::create_auth_layer::<shared::AuthRole>(&config);

    let app = routers::app_router(auth_layer)
        .layer(trace::TraceLayer::new_for_http())
        .with_state(state);

    #[cfg(feature = "api-doc")]
    let app = {
        use api::openapi;
        use utoipa::OpenApi;
        use utoipa_scalar::Scalar;
        use utoipa_scalar::Servable;

        app.merge(Scalar::with_url("/scalar", openapi::ApiDoc::openapi()))
    };

    let listener = tokio::net::TcpListener::bind(config.addrs()).await?;
    tracing::info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
