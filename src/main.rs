use axum::{routing::get, Router};
pub mod routes;
use routes::{
    day_minus_one::{bonus_minus_one, hello_bird},
    day_two::{decrypt_destination, decrypt_key, decrypt_destination_v6, decrypt_key_v6}
};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_bird))
        .route("/-1/seek", get(bonus_minus_one))
        .route("/2/dest", get(decrypt_destination))
        .route("/2/key", get(decrypt_key))
        .route("/2/v6/dest", get(decrypt_destination_v6))
        .route("/2/v6/key", get(decrypt_key_v6));

    Ok(router.into())
}
