use std::{str::FromStr, sync::Arc, time::Duration};

use async_graphql_axum::GraphQL;
use axum::routing;
use axum::{body::Body, extract::Request, Router};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::Level;
use uuid::Uuid;

use corelib::utils::{health_check, root};
use corelib::{config, env, subscriber, utils};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env::load_env();

    let settings = config::get_configuration().expect("Failed to read configuration.");

    let log_level = Level::from_str(settings.app.log.as_str()).unwrap();
    let subscriber =
        subscriber::get_subscriber("pokemon-graphql".into(), log_level, std::io::stdout);
    subscriber::init_subscriber(subscriber);

    let address = format!("{}:{}", settings.app.host, settings.app.port);
    let listener = TcpListener::bind(address).await?;

    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(2))
        .connect_lazy_with(settings.database.with_db());

    run_server(listener, pool, settings).await
}

async fn run_server(
    tcp_listener: TcpListener,
    database_pool: PgPool,
    config: config::Settings,
) -> Result<(), std::io::Error> {
    let schema = schema::build().data(database_pool).finish();
    let config_state = Arc::new(config);

    let cors_layer = CorsLayer::permissive();
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|_request: &Request<Body>| tracing::info_span!("processing request", request_id=%Uuid::now_v7()));

    let app = Router::new()
        .route("/", routing::get(root))
        .route("/health-check", routing::get(health_check))
        .route(
            "/graphql",
            routing::get(utils::sandbox).post_service(GraphQL::new(schema)),
        )
        .layer(trace_layer)
        .layer(cors_layer)
        .with_state(config_state);

    axum::serve(tcp_listener, app).await.unwrap();
    Ok(())
}
