use axum::http::StatusCode;
use serde::Deserialize;

use super::Metadata;

#[derive(Deserialize, Debug)]

struct Manifest {
    package: Package
}

#[derive(Deserialize, Debug)]
struct Package {
    metadata: Metadata,
    #[serde(rename = "rust-version")]
    rust_version: Option<String>,
}


pub async fn process_yaml_manifest(
    body: String
) -> Result<(StatusCode, String), StatusCode> {

    let manifest: Manifest = match serde_yaml::from_str(&body) {
        Ok(yaml) => yaml,
        Err(e) => {
            println!("error: {:?}", e);
            return Ok((StatusCode::BAD_REQUEST, "Invalid manifest".to_string()))
        }
    };
    
    let rust_version_option = manifest.package.rust_version;
    if rust_version_option == Some("true".to_string()) || rust_version_option == Some("false".to_string()) {
        return Ok((StatusCode::BAD_REQUEST, "Invalid manifest".to_string()))
    }

    let metadata: Metadata = manifest.package.metadata;


    let return_string = metadata.get_orders_string()?;

    Ok(
        (StatusCode::OK, return_string)
    )
}