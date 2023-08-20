/*Simple add function that returns the sum
Returns a JSON object with the sum of the two numbers
Takes an event object with two numbers:  x, y
*/


use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    x: i32,
    y: i32,
}


#[derive(Serialize)]
struct Response {
    total: i32,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract numbers from the event
    let x = event.payload.x;
    let y = event.payload.y;

    // Add the numbers
    let total = x + y;

    // Prepare the response
    let resp = Response {
        total
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

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
