use std::env;

use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use tracing::Level;
use uuid::Uuid;

use corelib::{config, env::load_env, subscriber};

pub async fn configure_database(config: &config::DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .unwrap();
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.name).as_str())
        .await
        .expect("Failed to create database.");

    let pool = PgPool::connect_with(config.with_db()).await.unwrap();

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate the database");

    pool
}

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_level = Level::INFO;
    let subscriber_name = "test";

    if env::var("TEST_LOG").is_ok() {
        let subscriber =
            subscriber::get_subscriber(subscriber_name.into(), default_level, std::io::stdout);
        subscriber::init_subscriber(subscriber);
    } else {
        let subscriber =
            subscriber::get_subscriber(subscriber_name.into(), default_level, std::io::sink);
        subscriber::init_subscriber(subscriber);
    }
});

pub async fn spawn_graphql() -> schema::AppSchemaBuilder {
    load_env();

    Lazy::force(&TRACING);

    let mut settings = config::get_configuration().expect("Failed to read configuration.");
    settings.database.name = Uuid::now_v7().to_string();

    let pool = configure_database(&settings.database).await;
    schema::build().data(pool)
}
