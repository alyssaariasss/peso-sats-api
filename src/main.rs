use axum::{routing::post, Json, Router};
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

#[derive(Debug, Serialize, Deserialize)]
struct ConvertedPeso {
    btc_to_php: f64,
    php: f64,
    sats: f64,
}

const BTC_TO_SATS: f64 = 100_000_000.00;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    // create axum router
    let app = Router::new().route("/", post(convert_peso_to_sats));

    // start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// connect to coingecko
async fn connect_to_client() -> PriceResponse {
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set");

    let response = Client::new()
        .get("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=php")
        .header("x-cg-demo-api-key", api_key)
        .send()
        .await
        .unwrap();

    response.json().await.unwrap()
}

// convert given php amount to sats
async fn convert_peso_to_sats(Json(body): Json<PriceData>) -> Json<ConvertedPeso> {
    let client = connect_to_client().await;
    let btc_to_php = client.bitcoin.php;
    let converted_sats = body.php / btc_to_php * BTC_TO_SATS;

    Json(ConvertedPeso {
        btc_to_php,
        php: body.php,
        sats: converted_sats,
    })
}
