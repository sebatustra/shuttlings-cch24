use std::{str::FromStr, sync::Arc};
use axum::{extract::State, http::{header, HeaderMap, StatusCode}, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation, Algorithm};
use cookie::Cookie;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    pub json: String,
    pub exp: usize
}

pub async fn wrap(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Result<(StatusCode, HeaderMap), StatusCode> {
    let mut headers = HeaderMap::new();
    println!("payload in wrap: {:?}", payload);

    let exp = get_exp_for_one_hour();

    let claim = Claims {
        json: payload.to_string(),
        exp
    };

    let secret = state.secret.lock().await;

    let token = encode(
        &Header::default(), 
        &claim, 
        &EncodingKey::from_secret(secret.as_ref())
    ).map_err(|e| {
        println!("Error creating jwt: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let cookie = Cookie::new("gift", token);

    headers.insert(
        header::SET_COOKIE, 
        cookie.to_string().parse().unwrap()
    );

    Ok((StatusCode::OK, headers))
}

pub async fn unwrap(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<Value>, StatusCode> {

    let secret = state.secret.lock().await;

    let cookie_str = headers.get(header::COOKIE)
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_str()
        .map_err(|e| {
            println!("Error converting cookie to string: {}", e);
            StatusCode::BAD_REQUEST
        })?;

    let cookie = Cookie::parse(cookie_str)
        .map_err(|e| {
            println!("Error converting cookie to string: {}", e);
            StatusCode::BAD_REQUEST
        })?;

    let (cookie_name, cookie_value) = cookie.name_value();

    if cookie_name == "gift" {
        let decoded = decode::<Claims>(
            cookie_value,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256)
        ).map_err(|e| {
            println!("Failed to decode token: {}", e);
            StatusCode::BAD_REQUEST
        })?;
    
        let value = Value::from_str(&decoded.claims.json)
            .map_err(|e| {
                println!("Failed to convert string to json value: {}", e);
                StatusCode::BAD_REQUEST
            })?;
    
        Ok(Json(value))
    } else {
        return Err(StatusCode::BAD_REQUEST)
    }

}

fn get_exp_for_one_hour() -> usize {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let one_hour_later = now.as_secs() + 3600; // Add 1 hour in seconds
    one_hour_later as usize
}


pub async fn decode_santa() -> impl IntoResponse {

}