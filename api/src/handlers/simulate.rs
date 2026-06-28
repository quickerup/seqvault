use axum::{Json, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use seqvault_core::sequence::Sequence;
use seqvault_core::emulator::Emulator;
use tracing::{info, error};

#[derive(Debug, Deserialize)]
pub struct SimulateRequest {
    pub sequence: Sequence,
}

#[derive(Debug, Serialize)]
pub struct SimulateResponse {
    pub hash: String,
    pub status: String,
}

pub async fn simulate_handler(
    Json(payload): Json<SimulateRequest>,
) -> impl IntoResponse {
    info!("Received simulation request for sequence with nonce {}", payload.sequence.nonce);

    // 1. Validate the sequence
    if let Err(e) = payload.sequence.validate() {
        error!("Sequence validation failed: {}", e);
        return (StatusCode::BAD_REQUEST, Json(SimulateResponse {
            hash: "".into(),
            status: format!("Validation Error: {}", e),
        })).into_response();
    }

    // 2. Calculate the deterministic hash
    let hash = match payload.sequence.calculate_hash() {
        Ok(h) => hex::encode(h),
        Err(e) => {
            error!("Hash calculation failed: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(SimulateResponse {
                hash: "".into(),
                status: format!("Hashing Error: {}", e),
            })).into_response();
        }
    };

    info!("Calculated sequence hash: {}", hash);

    // 3. Run simulation using the emulator
    let emulator = Emulator::new();
    if let Err(e) = emulator.run_simulation(&payload.sequence) {
        error!("Simulation failed for hash {}: {}", hash, e);
        return (StatusCode::OK, Json(SimulateResponse {
            hash,
            status: format!("Simulation Failure: {}", e),
        })).into_response();
    }

    info!("Simulation completed successfully for hash {}", hash);

    (StatusCode::OK, Json(SimulateResponse {
        hash,
        status: "success".into(),
    })).into_response()
}
