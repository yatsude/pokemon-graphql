use tracing::subscriber::set_global_default;
use tracing::{Level, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

pub fn get_subscriber<Sink>(
    name: String,
    default_level: Level,
    sink: Sink,
) -> impl Subscriber + Send + Sync + 'static
where
    Sink: for<'a> MakeWriter<'a> + Sync + Send + 'static,
{
    let filter = Targets::new()
        .with_target("tower_http::trace", default_level)
        .with_target("async_graphql::graphql", Level::WARN)
        .with_target("sqlx::query", Level::WARN)
        .with_target("zero2prod", default_level)
        .with_default(default_level);
    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    Registry::default()
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .with(filter)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync + 'static) {
    set_global_default(subscriber).expect("Failed to set subscriber");
}
