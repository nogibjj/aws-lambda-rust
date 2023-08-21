use axum::{extract::Path, response::Json, routing::get, Router};
use lambda_http::{run, Error};
use polars_lambda_axum::calculate;
use serde_json::{json, Value};

async fn root() -> &'static str {
    "Hello, Polars"
}

//simple url: /iris/filter/5
async fn get_filter(Path(value): Path<f64>) -> Json<Value> {
    let df = calculate(value).unwrap();
    let json = json!({
        "payload": format!("{}", df),
    });
    Json(json)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let app = Router::new()
        .route("/", get(root))
        .route("/iris/filter/:value", get(get_filter));
    run(app).await
}
