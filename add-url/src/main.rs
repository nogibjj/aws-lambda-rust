use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde_json::json;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract the numbers from the URL path
    let path = event.uri().path();
    let parts: Vec<&str> = path.split('/').collect();

    let a: i32 = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    let b: i32 = parts.get(3).and_then(|s| s.parse().ok()).unwrap_or(0);

    let sum = a + b;

    // Create JSON payload with the sum
    let payload = json!({ "total": sum });

    // Return a response with the JSON payload
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
