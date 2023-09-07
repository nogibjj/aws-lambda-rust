use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    greeting: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

//A marco polo function that return polo if you pass in Marco
fn marco_polo_greeting(greeting: &str) -> String {
    if greeting == "Marco" {
        "Polo".to_string()
    } else {
        "No".to_string()
    }
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful info from the request
    let greeting = event.payload.greeting;
    
    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: marco_polo_greeting(&greeting),
    };

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
