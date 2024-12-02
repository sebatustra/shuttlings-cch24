use axum::{http::{StatusCode, header}, response::IntoResponse, routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, bird!"
}

async fn bonus_minus_one() -> impl IntoResponse {
    (
        StatusCode::FOUND,
        [(header::LOCATION, "https://www.youtube.com/watch?v=9Gc4QTqslN4")]
    )
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/seek", get(bonus_minus_one));

    Ok(router.into())
}
