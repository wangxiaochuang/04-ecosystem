use std::time::Duration;

use axum::{routing::get, Router};
use tokio::{
    net::TcpListener,
    time::{sleep, Instant},
};
use tracing::{info, instrument, level_filters::LevelFilter, warn};
use tracing_subscriber::{
    fmt::{format::FmtSpan, Layer},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer as _,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // tracing_subscriber::fmt::init(); // old way
    let console = Layer::new()
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .with_filter(LevelFilter::DEBUG);
    let file_appender = tracing_appender::rolling::daily("/tmp/logs", "ecosystem.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let file = Layer::new()
        // .with_span_events(FmtSpan::CLOSE)
        .with_writer(non_blocking)
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(console)
        .with(file)
        .init();

    let addr = "0.0.0.0:8081";
    let app = Router::new().route("/", get(index_handler));

    let listener = TcpListener::bind(addr).await?;
    info!("Starting server on {}", addr);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[instrument]
async fn index_handler() -> &'static str {
    sleep(Duration::from_millis(10)).await;
    let ret = long_task().await;
    info!(http.status = 200, "index handler completed");
    ret
}

#[instrument]
async fn long_task() -> &'static str {
    let start = Instant::now();
    sleep(Duration::from_millis(112)).await;
    let elapsed = start.elapsed().as_millis();
    warn!(app.task_dration = elapsed, "task takes too long");
    "Hello, World!"
}
