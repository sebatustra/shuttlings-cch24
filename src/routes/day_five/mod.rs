use axum::http::{header::CONTENT_TYPE, HeaderMap, StatusCode};
use json_manifest::process_json_manifest;
use toml_manifest::process_toml_manifest;
use yaml_manifest::process_yaml_manifest;

pub mod toml_manifest;
pub mod yaml_manifest;
pub mod json_manifest;

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