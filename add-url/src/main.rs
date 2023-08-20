use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde_json::json; // Import serde_json for handling JSON data

// Define the asynchronous function handler for the Lambda function
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract the URL path from the incoming request
    let path = event.uri().path();
    // Split the path into parts using '/' as the separator
    let parts: Vec<&str> = path.split('/').collect();

    // Extract and parse the value of 'a' from the URL, default to 0 if parsing fails
    let a: i32 = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    // Extract and parse the value of 'b' from the URL, default to 0 if parsing fails
    let b: i32 = parts.get(3).and_then(|s| s.parse().ok()).unwrap_or(0);

    // Calculate the sum of 'a' and 'b'
    let sum = a + b;

    // Create a JSON payload containing the sum
    let payload = json!({ "total": sum });

    // Build the response with the JSON payload, setting the HTTP status to 200 and content type to application/json
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .map_err(Box::new)?;
    // Return the response
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize tracing for logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    // Run the Lambda function using the defined handler
    run(service_fn(function_handler)).await
}
