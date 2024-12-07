
use axum::http::StatusCode;
use cargo_manifest::Manifest;

use super::{verify_keywords, Metadata};

pub async fn process_toml_manifest(
    body: String
) -> Result<(StatusCode, String), StatusCode> {
    
    let cargo_manifest = Manifest::<Metadata>::from_slice_with_metadata(body.as_bytes());
    
    let manifest = match cargo_manifest {
        Ok(manifest) => manifest,
        Err(_) => return Ok((StatusCode::BAD_REQUEST, "Invalid manifest".to_string()))
    };

    let package = manifest.package.ok_or(StatusCode::NO_CONTENT)?;

    let keywords = match package.keywords {
        Some(keywords) => match keywords.as_local() {
            Some(keywords) => keywords,
            None => return Ok((StatusCode::BAD_REQUEST, "Magic keyword not provided".to_string()))
        },
        None => return Ok((StatusCode::BAD_REQUEST, "Magic keyword not provided".to_string()))
    };

    match verify_keywords(keywords) {
        Ok(_) => (),
        Err(e) => return Ok(e)
    };

    let metadata = package.metadata.ok_or(StatusCode::NO_CONTENT)?;
    
    let return_string = metadata.get_orders_string()?;

    Ok(
        (StatusCode::OK, return_string)
    )
}


