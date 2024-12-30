use axum::{extract::Path, http::StatusCode};
use html_escape::encode_double_quoted_attribute;

pub async fn light_star() -> &'static str {
    return "<div class=\"lit\" id=\"star\"></div>";
}

pub async fn change_color(
    Path(color): Path<String>
) -> Result<String, StatusCode> {
    let new_color = match color.as_str() {
        "red" => ("red", "blue"),
        "blue" => ("blue", "purple"),
        "purple" => ("purple", "red"),
        _ => return Err(StatusCode::IM_A_TEAPOT)
    };

    let new_div = format!(
        r#"
            <div class="present {}" hx-get="/23/present/{}" hx-swap="outerHTML">
                <div class="ribbon"></div>
                <div class="ribbon"></div>
                <div class="ribbon"></div>
                <div class="ribbon"></div>
            </div>
        "#,
        encode_double_quoted_attribute(new_color.0).to_string(),
        encode_double_quoted_attribute(new_color.1).to_string()
    );

    Ok(new_div)
}

pub async fn change_ornament(
    Path((state, n)): Path<(String, String)>
) -> Result<String, StatusCode> {
    let next_state = match state.as_str() {
        "on" => "off",
        "off" => "on",
        _ => return Err(StatusCode::IM_A_TEAPOT),
    };

    let class = if state == "on" { " on" } else { "" };

    let new_div = format!(
        r#"<div class="ornament{}" id="ornament{}" hx-trigger="load delay:2s once" hx-get="/23/ornament/{}/{}" hx-swap="outerHTML"></div>"#,
        encode_double_quoted_attribute(class).to_string(),
        encode_double_quoted_attribute(&n).to_string(),
        encode_double_quoted_attribute(next_state).to_string(),
        encode_double_quoted_attribute(&n).to_string()
    );

    Ok(new_div)
}

