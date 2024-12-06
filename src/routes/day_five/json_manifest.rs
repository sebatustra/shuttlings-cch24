use axum::http::StatusCode;


pub async fn process_json_manifest(
    _body: String
) -> Result<(StatusCode, String), StatusCode> {
    Ok(
        (StatusCode::OK,
        "OK".to_string())
    )
}