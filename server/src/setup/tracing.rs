use opentelemetry::{runtime::TokioCurrentThread, sdk::propagation::TraceContextPropagator};
use std::env;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

pub fn start_tracing() {
  let app_name = "rust_web_server";

  opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());

  let tracer = opentelemetry_jaeger::new_pipeline()
    .with_service_name(app_name)
    .install_batch(TokioCurrentThread)
    .expect("Failed to install OpenTelemetry tracer.");

  let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));
  let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
  let formatting_layer = BunyanFormattingLayer::new(app_name.into(), std::io::stdout);
  let subscriber = Registry::default()
    .with(env_filter)
    .with(telemetry)
    .with(JsonStorageLayer)
    .with(formatting_layer);

  tracing::subscriber::set_global_default(subscriber).expect("Failed to install `tracing` subscriber.");
  tracing::info!("tracing UI running at {}", env::var("TRACING_UI").unwrap());
}
