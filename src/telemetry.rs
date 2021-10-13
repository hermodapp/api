//! Contains functions that configure and initialize tracing telemetry, as well as additional helpers.
use actix_web::rt::task::JoinHandle;
use opentelemetry_otlp::WithExportConfig;
use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

/// Compose multiple layers into a `tracing`'s subscriber.
///
/// # Implementation Notes
///
/// We are using `impl Subscriber` as return type to avoid having to spell out the actual
/// type of the returned subscriber, which is indeed quite complex.
pub fn get_subscriber(
    name: String,
    env_filter: String,
    sink: impl MakeWriter + Send + Sync + 'static,
    endpoint_url: &str,
) -> impl Subscriber + Sync + Send {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(endpoint_url),
        )
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Failed to install tracing pipeline.");
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .with(telemetry)
}

pub fn get_subscriber_test(
    name: String,
    env_filter: String,
    sink: impl MakeWriter + Send + Sync + 'static,
) -> impl Subscriber + Sync + Send {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}

/// Spawn a blocking function within the current tracing Span
pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    actix_web::rt::task::spawn_blocking(move || current_span.in_scope(f))
}
