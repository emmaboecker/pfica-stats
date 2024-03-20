use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;
use dotenvy::dotenv;
use mongodb::{Client, Collection};
use tower_http::cors::CorsLayer;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;

use crate::database::DatabaseGeneratorRun;
use crate::routes::push::push_run_handler;

pub mod database;

pub mod routes;

pub struct AppState {
    generator_runs: Collection<DatabaseGeneratorRun>,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into())
                .from_env_lossy(),
        )
        .init();

    let listen_url = std::env::var("LISTEN_URL").unwrap_or("0.0.0.0:3000".to_string());

    let listener = tokio::net::TcpListener::bind(&listen_url).await.unwrap();

    let mongo_url = std::env::var("MONGO_URL").unwrap_or("mongodb://localhost:27017".to_string());
    let mongo_db = std::env::var("MONGO_DB").unwrap_or("pfica-stats".to_string());

    let client = Client::with_uri_str(mongo_url).await.unwrap();
    let database = client.database(&mongo_db);
    let generator_runs = database.collection::<DatabaseGeneratorRun>("generator_runs");

    let shared_state = Arc::new(AppState { generator_runs });

    let origins = std::env::var("CORS_ORIGINS")
        .map(|origins| {
            origins
                .split(',')
                .map(|origin| origin.to_string().parse().expect("Invalid origin"))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    println!("{:?}", origins);

    let app = Router::new()
        .route("/", get(|| async { "soon :3" }))
        .route("/push_run", post(push_run_handler))
        .layer(CorsLayer::new().allow_origin(origins))
        .with_state(shared_state);

    tracing::info!("Listening on {}", listen_url);
    axum::serve(listener, app).await?;

    Ok(())
}
