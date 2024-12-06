
use axum::http::StatusCode;
use cargo_manifest::Manifest;
use serde::Deserialize;

pub async fn process_toml_manifest(
    body: String
) -> Result<(StatusCode, String), StatusCode> {
    
    let cargo_manifest = Manifest::<Metadata>::from_slice_with_metadata(body.as_bytes());
    
    let manifest = match cargo_manifest {
        Ok(manifest) => manifest,
        Err(_) => return Ok((StatusCode::BAD_REQUEST, "Invalid manifest".to_string()))
    };
    
    let package = match manifest.package {
        Some(package) => package,
        None => return Err(StatusCode::NO_CONTENT)
    };

    let keywords = match package.keywords {
        Some(keywords) => match keywords.as_local() {
            Some(keywords) => keywords,
            None => return Ok((StatusCode::BAD_REQUEST, "Magic keyword not provided".to_string()))
        },
        None => return Ok((StatusCode::BAD_REQUEST, "Magic keyword not provided".to_string()))
    };

    if !keywords.contains(&"Christmas 2024".to_string()) {
        return Ok((StatusCode::BAD_REQUEST, "Magic keyword not provided".to_string()))
    }
    
    let metadata = match package.metadata {
        Some(metadata) => metadata,
        None => return Err(StatusCode::NO_CONTENT)
    };
    
    let return_string = metadata.get_orders_string()?;

    Ok(
        (StatusCode::OK, return_string)
    )
}

#[derive(Deserialize, Debug)]
struct Metadata {
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

