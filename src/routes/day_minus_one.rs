use axum::response::IntoResponse;
use axum::http::{StatusCode, header};

pub async fn hello_bird() -> &'static str {
    "Hello, bird!"
}

pub async fn bonus_minus_one() -> impl IntoResponse {
    (
        StatusCode::FOUND,
        [(header::LOCATION, "https://www.youtube.com/watch?v=9Gc4QTqslN4")]
    )
}