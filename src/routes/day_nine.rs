use std::sync::Arc;
use axum::{extract::State, http::{header::CONTENT_TYPE, HeaderMap, StatusCode}, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;
use crate::AppState;

#[derive(Clone, Debug)]
pub struct Bucket {
    capacity: u8
}

impl Bucket {
    pub fn init() -> Self {
        Self {
            capacity: 5
        }
    }

    pub fn withdraw(&mut self) -> Option<()> {
        println!("calling withdraw");
        if self.capacity > 0 {
            println!("withdrawing 1 liter of milk");
            self.capacity -= 1;
            Some(())
        } else {
            println!("bucket is empty, unable to withdraw milk");
            None
        }
    }

    pub fn refill(&mut self) {
        println!("calling refill");
        if self.capacity < 5 {
            println!("increasing capacity by 1");
            self.capacity += 1;
        }
    }

    pub fn force_refill(&mut self) {
        self.capacity = 5;
    }
}

#[derive(Deserialize)]
pub struct ConvertUnit {
    pub gallons: Option<f32>,
    pub liters: Option<f32>,
    pub litres: Option<f32>,
    pub pints: Option<f32>,
}

impl ConvertUnit {
    pub fn convert_liters_to_gallons(liters: f32) -> f32 {
        liters / 3.78541
    }
    
    pub fn convert_gallons_to_liters(gallons: f32) -> f32 {
        gallons * 3.78541
    }

    pub fn convert_litres_to_pints(liters: f32) -> f32 {
        liters * 1.75975
    }

    pub fn convert_pints_to_litres(pints: f32) -> f32 {
        pints / 1.75975
    }
}

pub async fn leaky_bucket(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    payload_opt: Option<Json<ConvertUnit>>
) -> impl IntoResponse {

    let mut bucket = state.bucket.lock().await;

    if bucket.withdraw().is_none() {
        return (StatusCode::TOO_MANY_REQUESTS, "No milk available\n".to_string()).into_response()
    }

    if headers.get(CONTENT_TYPE)
        .and_then(|ct| ct.to_str().ok())
        .filter(|ct| *ct == "application/json")
        .is_some()
    {
        match payload_opt {
            Some(Json(payload)) => {
                match try_to_convert(payload) {
                    Ok(result) => {
                        return Json(json!({
                            result.result_unit: result.result_number
                        })).into_response()
                    },
                    Err(_) => StatusCode::BAD_REQUEST.into_response()
                }
            },
            None => return StatusCode::BAD_REQUEST.into_response()
        }

    } else {
        return (StatusCode::OK, "Milk withdrawn\n".to_string()).into_response()
    }
}

pub struct ConversionResult {
    pub result_unit: String,
    pub result_number: f32
}

fn try_to_convert(payload: ConvertUnit) -> Result<ConversionResult, ()>{
    if payload.gallons.is_some() && payload.liters.is_some() {
        return Err(())
    }

    if payload.litres.is_some() && payload.liters.is_some() {
        return Err(())
    }

    if payload.gallons.is_some() && payload.pints.is_some() {
        return Err(())
    }

    if payload.litres.is_some() && payload.pints.is_some() {
        return Err(())
    }

    if let Some(gallons) = payload.gallons {
        return Ok(ConversionResult {
            result_unit: "liters".to_string(),
            result_number: ConvertUnit::convert_gallons_to_liters(gallons)
        })
    }

    if let Some(liters) = payload.liters {
        return Ok(ConversionResult {
            result_unit: "gallons".to_string(),
            result_number: ConvertUnit::convert_liters_to_gallons(liters)
        })
    }

    if let Some(litres) = payload.litres {
        return Ok(ConversionResult {
            result_unit: "pints".to_string(),
            result_number: ConvertUnit::convert_litres_to_pints(litres)
        })
    }

    if let Some(pints) = payload.pints {
        return Ok(ConversionResult {
            result_unit: "litres".to_string(),
            result_number: ConvertUnit::convert_pints_to_litres(pints)
        })
    }


    Err(())
}

pub async fn refill_bucket(
    State(state): State<Arc<AppState>>
) -> impl IntoResponse {
    state.bucket.lock().await.force_refill();

    (StatusCode::OK).into_response()
}