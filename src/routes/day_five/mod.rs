use axum::http::{header::CONTENT_TYPE, HeaderMap, StatusCode};
use json_manifest::process_json_manifest;
use serde::Deserialize;
use toml_manifest::process_toml_manifest;
use yaml_manifest::process_yaml_manifest;

pub mod toml_manifest;
pub mod yaml_manifest;
pub mod json_manifest;

#[derive(Deserialize, Debug)]
pub struct Metadata {
    pub orders: Option<Vec<Order>>
}

impl Metadata {
    pub fn get_valid_orders(self) -> Result<Vec<Order>, StatusCode> {
        match self.orders {
            Some(orders) => {
                let valid_orders: Vec<Order> = orders.into_iter()
                    .filter(|order| order.is_valid())
                    .collect();

                if valid_orders.is_empty() {
                    return Err(StatusCode::NO_CONTENT)
                }

                Ok(valid_orders)
            },
            None => return Err(StatusCode::NO_CONTENT)
        }
    }

    pub fn get_orders_string(self) -> Result<String, StatusCode> {
        let mut return_strings: Vec<String> = Vec::new();
        let valid_orders = self.get_valid_orders()?;

        for order in valid_orders.iter() {
            let quantity = order.quantity.as_ref().unwrap().to_string();
            let string_order = format!("{}: {}", &order.item, quantity);
            return_strings.push(string_order);
        }
        let return_string = return_strings.join("\n");

        Ok(return_string)
    }
}

#[derive(Deserialize, Debug)]
pub struct Order {
    pub item: String,
    pub quantity: Option<serde_json::Value>
}

impl Order {
    pub fn is_valid(&self) -> bool {
        match &self.quantity {
            Some(quantity) => {
                match quantity {
                    serde_json::Value::Number(num) => {
                        num.as_u64().is_some() && !self.item.is_empty()
                    },
                    _ => false
                }
            }
            None => false
        }
    }
}

pub async fn process_manifest(
    headers: HeaderMap,
    body: String,
) -> Result<(StatusCode, String), StatusCode> {
    let content_type: &axum::http::HeaderValue = match headers.get(CONTENT_TYPE) {
        Some(content_type) => content_type,
        None => return Err(StatusCode::UNSUPPORTED_MEDIA_TYPE)
    };

    match content_type.to_str() {
        Ok(content_type_str) => {
            match content_type_str {
                "application/json" => Ok(process_json_manifest(body).await?),
                "application/yaml" => Ok(process_yaml_manifest(body).await?),
                "application/toml" => Ok(process_toml_manifest(body).await?),
                _ => return Err(StatusCode::UNSUPPORTED_MEDIA_TYPE)
            }
        },
        Err(_) => return Err(StatusCode::UNSUPPORTED_MEDIA_TYPE)
    }
}

pub fn verify_keywords(
    keywords: Vec<String>
) -> Result<(), (StatusCode, String)> {
    if !keywords.contains(&"Christmas 2024".to_string()) {
        return Err((StatusCode::BAD_REQUEST, "Magic keyword not provided".to_string()))
    }

    Ok(())
}

