use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::{debug_handler, Json};
use serde::Deserialize;

use crate::database::{DatabaseGeneratorRun, GeneratorSubmitType, GeneratorValues};
use crate::AppState;

#[derive(Deserialize)]
pub struct GeneratorRun {
    pub runner_id: String,
    pub submit_type: GeneratorSubmitType,
    pub values: GeneratorValues,
}

#[debug_handler]
pub async fn push_run_handler(
    State(shared_state): State<Arc<AppState>>,
    Json(generator_run): Json<GeneratorRun>,
) -> StatusCode {
    tracing::info!("Received run from {}", generator_run.runner_id);
    tracing::debug!("Values: {:?}", generator_run.values);

    shared_state
        .generator_runs
        .insert_one(
            DatabaseGeneratorRun {
                runner_id: generator_run.runner_id,
                submit_type: generator_run.submit_type,
                values: generator_run.values,
                run_at: chrono::Utc::now(),
            },
            None,
        )
        .await
        .expect("Failed to insert run into database");

    StatusCode::OK
}
