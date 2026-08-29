use api::config;
use api::routers;
use api::state;
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
    tracing::debug!("{config:?}");

    let state = state::AppState::new(&config.database_url(), config.max_connections()).await?;

    let app = routers::app_router()
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
