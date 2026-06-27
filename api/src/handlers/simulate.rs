use axum::response::IntoResponse;

pub async fn simulate_handler() -> impl IntoResponse {
    "Simulation endpoint"
}
