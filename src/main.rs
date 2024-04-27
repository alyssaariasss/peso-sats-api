use axum::{routing::post, Json, Router};
use reqwest::{Client, StatusCode};
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
    input_php: f64,
    output_sats: f64,
}

const BTC_TO_SATS: f64 = 100_000_000.00;
const CLIENT_URL: &'static str =
    "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=php";

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
async fn connect_to_client() -> Result<PriceResponse, StatusCode> {
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set");

    let response = match Client::new()
        .get(CLIENT_URL)
        .header("x-cg-demo-api-key", api_key)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    if response.status().is_success() {
        match response.json().await {
            Ok(data) => return Ok(data),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else {
        return Err(response.status());
    }
}

// convert given php amount to sats
async fn convert_peso_to_sats(
    Json(body): Json<PriceData>,
) -> Result<Json<ConvertedPeso>, StatusCode> {
    let client = match connect_to_client().await {
        Ok(data) => data,
        Err(status) => return Err(status),
    };

    let btc_to_php = client.bitcoin.php;
    let converted_sats = body.php / btc_to_php * BTC_TO_SATS;

    Ok(Json(ConvertedPeso {
        btc_to_php,
        input_php: body.php,
        output_sats: converted_sats,
    }))
}
