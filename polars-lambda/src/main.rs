/*
Integrates Polars with AWS Lambda.
*/

use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use polars_lambda::calculate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    filter: f64,
}

#[derive(Serialize)]
struct Response {
    payload: String,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract filter float from the request
    let filter_value = event.payload.filter;

    // Run the DataFrame calculation
    let df = calculate(filter_value).unwrap();

    // Prepare the response
    let resp = Response {
        payload: format!("{}", df),
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

// The main function is the entry point of the program.
// It returns a Result type, which is either Ok or Err.
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
