use axum::http::StatusCode;
use serde::Deserialize;

use crate::routes::day_five::verify_keywords;

use super::Metadata;

#[derive(Deserialize, Debug)]
struct Manifest {
    package: Package
}

#[derive(Deserialize, Debug)]
struct Package {
    metadata: Metadata,
    #[serde(rename = "rust-version")]
    #[allow(dead_code)]
    rust_version: Option<String>,
    keywords: Option<Vec<String>>
}


pub async fn process_json_manifest(
    body: String
) -> Result<(StatusCode, String), StatusCode> {

    let manifest: Manifest = match serde_json::from_str(&body) {
        Ok(json) => json,
        Err(e) => {
            println!("error: {:?}", e);
            return Ok((StatusCode::BAD_REQUEST, "Invalid manifest".to_string()))
        }
    };
    println!("manifest json: {:?}", manifest);

    let metadata: Metadata = manifest.package.metadata;

    let return_string = metadata.get_orders_string()?;

    match manifest.package.keywords {
        Some(keywords) => {
            match verify_keywords(keywords) {
                Ok(_) => (),
                Err(e) => return Ok(e)
            };
        }
        None => return Ok((StatusCode::BAD_REQUEST, "Magic keyword not provided".to_string()))
    }

    Ok(
        (StatusCode::OK, return_string)
    )
}