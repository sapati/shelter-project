use axum::http::StatusCode;

pub async fn hello() -> Result<String, StatusCode> {
    Ok("\nHello world!\n\n".to_string())
}
