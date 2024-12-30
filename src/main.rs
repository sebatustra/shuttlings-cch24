use std::sync::Arc;
use rand::rngs::StdRng;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};
use rand::SeedableRng;
use axum::{routing::{get, post, delete, put}, Router};
use sqlx::PgPool;
use tower_http::services::ServeDir;

pub mod routes;
use routes::{
    day_five::process_manifest, day_minus_one::{
        bonus_minus_one, 
        hello_bird
    }, 
    day_nine::{leaky_bucket, Bucket, refill_bucket}, 
    day_two::{
        decrypt_destination, 
        decrypt_destination_v6, 
        decrypt_key, 
        decrypt_key_v6
    },
    day_twelve::{
        create_board, 
        reset_board,
        place_item,
        generate_random_board,
        Board
    },
    day_sixteen::{
        wrap,
        unwrap
    },
    day_nineteen::{
        quote_controller::QuoteController,
        draft,
        cite,
        remove,
        undo,
        reset
    },
    day_twenty_three::{
        light_star,
        change_color,
        change_ornament
    }
};

pub struct AppState {
    pub bucket: Mutex<Bucket>,
    pub board: Mutex<Board>,
    pub rng: Mutex<StdRng>,
    pub secret: Mutex<String>,
    pub quote_controller: QuoteController,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_axum::ShuttleAxum {

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let app_state = Arc::new(AppState {
        bucket: Mutex::new(Bucket::init()),
        board: Mutex::new(Board::new()),
        rng: Mutex::new(rand::rngs::StdRng::seed_from_u64(2024)),
        secret: Mutex::new(String::from("ULTRA_SECRET!!")),
        quote_controller: QuoteController::build(pool)
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
        .route("/12/board", get(create_board))
        .route("/12/place/:team/:column", post(place_item))
        .route("/12/reset", post(reset_board))
        .route("/12/random-board", get(generate_random_board))
        .route("/16/wrap", post(wrap))
        .route("/16/unwrap", get(unwrap))
        .route("/19/reset", post(reset))
        .route("/19/cite/:id", get(cite))
        .route("/19/remove/:id", delete(remove))
        .route("/19/undo/:id", put(undo))
        .route("/19/draft", post(draft))
        .route("/23/star", get(light_star))
        .route("/23/present/:color", get(change_color))
        .route("/23/ornament/:state/:n", get(change_ornament))
        .nest_service("/assets", ServeDir::new("assets"))
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
