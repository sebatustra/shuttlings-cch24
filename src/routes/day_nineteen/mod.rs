use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub mod quote_controller;

use crate::AppState;

#[derive(FromRow, Debug, Serialize)]
pub struct Quote {
    pub id: Uuid,
    pub author: String,
    pub quote: String,
    pub created_at: DateTime<Utc>,
    pub version: i32
}

#[derive(Deserialize, Debug)]
pub struct QuoteForCreation {
    pub author: String,
    pub quote: String
}

#[derive(Deserialize, Debug)]
pub struct QuoteForUpdate {
    pub author: String,
    pub quote: String
}

#[axum::debug_handler]
pub async fn draft(
    State(state): State<Arc<AppState>>,
    Json(body): Json<QuoteForCreation>
) -> Result<(StatusCode, Json<Quote>), StatusCode> {

    match state.quote_controller
        .create_quote(body)
        .await 
    {
        Ok(quote) => Ok((StatusCode::CREATED, Json(quote))),
        Err(e) => {
            println!("Error inserting quote: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[axum::debug_handler]
pub async fn cite(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>
) -> Result<Json<Quote>, StatusCode> {
    match state.quote_controller
        .get_quote(&id)
        .await
    {
        Ok(option) => {
            match option {
                Some(quote) => Ok(Json(quote)),
                None => Err(StatusCode::NOT_FOUND)
            }
        },
        Err(e) => {
            println!("Error fetching quote: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[axum::debug_handler]
pub async fn remove(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>
) -> Result<Json<Quote>, StatusCode> {
    match state.quote_controller
        .delete_quote(&id)
        .await
    {
        Ok(option) => {
            match option {
                Some(quote) => Ok(Json(quote)),
                None => Err(StatusCode::NOT_FOUND)
            }
        },
        Err(e) => {
            println!("Error deleting quote: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[axum::debug_handler]
pub async fn undo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(body): Json<QuoteForUpdate>
) -> Result<Json<Quote>, StatusCode> {
    match state.quote_controller
        .update_quote(&id, &body)
        .await
    {
        Ok(option) => {
            match option {
                Some(quote) => Ok(Json(quote)),
                None => Err(StatusCode::NOT_FOUND)
            }
        },
        Err(e) => {
            println!("Error updating quote: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[axum::debug_handler]
pub async fn reset(
    State(state): State<Arc<AppState>>,
) -> StatusCode {
    match state.quote_controller
        .clean_db()
        .await 
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("Error deleting table: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
