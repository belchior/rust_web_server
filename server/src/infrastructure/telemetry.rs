pub fn start_tracing() {
  use tracing_subscriber::{EnvFilter, Registry, fmt, layer::SubscriberExt};

  let subscriber = Registry::default()
    .with(fmt::layer().json().flatten_event(true))
    .with(EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info")));

  tracing::subscriber::set_global_default(subscriber).unwrap();
}
