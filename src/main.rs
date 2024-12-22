use atuin_server_sqlite_unofficial::Sqlite;
use tokio::net::lookup_host;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    let formatting_layer = tracing_tree::HierarchicalLayer::default()
        .with_writer(tracing_subscriber::fmt::TestWriter::new())
        .with_indent_lines(true)
        .with_ansi(true)
        .with_targets(true)
        .with_indent_amount(2);

    tracing_subscriber::registry()
        .with(formatting_layer)
        .with(EnvFilter::from_default_env())
        .init();

    let settings = atuin_server::Settings::new().unwrap();
    let addr = lookup_host((&*settings.host, settings.port))
        .await
        .expect("listen addr should be valid")
        .next()
        .expect("listen addr should be valid");

    atuin_server::launch::<Sqlite>(settings, addr)
        .await
        .unwrap();
}
