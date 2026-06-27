use axum::response::IntoResponse;

pub async fn commit_handler() -> impl IntoResponse {
    "Commit endpoint"
}
