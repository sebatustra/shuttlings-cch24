use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};

use axum::{routing::{get, post}, Router};
pub mod routes;
use routes::{
    day_five::process_manifest, day_minus_one::{
        bonus_minus_one, 
        hello_bird
    }, day_nine::{leaky_bucket, Bucket, refill_bucket}, day_two::{
        decrypt_destination, 
        decrypt_destination_v6, 
        decrypt_key, 
        decrypt_key_v6
    }
};

pub struct AppState {
    pub bucket: Mutex<Bucket>
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {

    let app_state = Arc::new(AppState {
        bucket: Mutex::new(Bucket::init())
    });
    
    let router = Router::new()
    .route("/", get(hello_bird))
    .route("/-1/seek", get(bonus_minus_one))
    .route("/2/dest", get(decrypt_destination))
    .route("/2/key", get(decrypt_key))
    .route("/2/v6/dest", get(decrypt_destination_v6))
    .route("/2/v6/key", get(decrypt_key_v6))
    .route("/5/manifest", post(process_manifest))
    .route("/9/milk", post(leaky_bucket))
    .route("/9/refill", post(refill_bucket))
    .with_state(app_state.clone());

    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            app_state.bucket.lock().await.refill();
        }
    });

    Ok(router.into())
}
