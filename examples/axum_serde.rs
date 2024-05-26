use std::sync::{Arc, Mutex};

use anyhow::Result;
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tracing::{info, instrument, level_filters::LevelFilter};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer,
};

#[derive(Debug, Clone, PartialEq, Serialize)]
struct User {
    name: String,
    age: u8,
    skills: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct UpdateUser {
    age: u8,
    skills: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let console = fmt::Layer::new()
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .with_filter(LevelFilter::DEBUG);
    tracing_subscriber::registry().with(console).init();

    let user = User {
        name: "Alice".to_string(),
        age: 30,
        skills: vec!["Rust".to_string(), "Web Development".to_string()],
    };
    let user = Arc::new(Mutex::new(user));

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on: {}", addr);

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/", post(update_handler))
        .with_state(user);
    axum::serve(listener, app).await?;
    Ok(())
}

#[instrument]
async fn index_handler(State(user): State<Arc<Mutex<User>>>) -> Json<User> {
    user.lock().unwrap().clone().into()
}

#[instrument]
async fn update_handler(
    State(user): State<Arc<Mutex<User>>>,
    Json(user_update): Json<UpdateUser>,
) -> Json<UpdateUser> {
    let mut user = user.lock().unwrap();
    user.age = user_update.age;
    user.skills = user_update.skills.clone();
    user_update.clone().into()
}
