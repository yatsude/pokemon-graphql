use std::env;

use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use tracing::Level;
use uuid::Uuid;

use corelib::subscriber;
use shared::settings;

pub async fn configure_database(setting: &settings::DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&setting.without_db())
        .await
        .unwrap();
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, setting.name).as_str())
        .await
        .expect("Failed to create database.");

    let pool = PgPool::connect_with(setting.with_db()).await.unwrap();

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
    shared::env::load_env();

    Lazy::force(&TRACING);

    let mut settings = settings::get_configuration().expect("Failed to read configuration.");
    settings.database.name = Uuid::now_v7().to_string();

    let pool = configure_database(&settings.database).await;
    schema::build().data(pool)
}
