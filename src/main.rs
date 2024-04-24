use axum::{routing::get, Json, Router};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct PriceResponse {
    bitcoin: PriceData,
}

#[derive(Debug, Serialize, Deserialize)]
struct PriceData {
    php: f64,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    // create axum router
    let app = Router::new().route("/", get(fetch_btc_to_sats));

    // start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn fetch_btc_to_sats() -> Json<PriceResponse> {
    // curcl command: curl "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=php&x_cg_demo_api_key={api_key}"

    let api_key = std::env::var("API_KEY").expect("API_KEY must be set");
    // let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=php";

    let response = Client::new()
        .get("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=php")
        .header("x-cg-demo-api-key", api_key)
        .send()
        .await
        .unwrap();

    let response_data: PriceResponse = response.json().await.unwrap();

    Json(response_data)
}
