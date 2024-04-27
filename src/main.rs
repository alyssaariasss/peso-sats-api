use axum::{routing::post, Json, Router};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

/// Model to hold response from CoinGecko client.
#[derive(Debug, Serialize, Deserialize)]
struct PriceResponse {
    /// Current Bitcoin price in PHP.
    bitcoin: PriceData,
}

/// Model to hold user-provided PHP amount to be converted.
#[derive(Debug, Serialize, Deserialize)]
struct PriceData {
    php: f64,
}

/// Model to hold details of converted PHP amount.
#[derive(Debug, Serialize, Deserialize)]
struct ConvertedPeso {
    /// Current Bitcoin to PHP conversion rate.
    btc_to_php: f64,
    /// User-provided PHP amount to be converted.
    input_php: f64,
    /// Converted amount in SATS.
    output_sats: f64,
}

/// Constant representing the conversion rate from Bitcoin to Satoshis.
///
/// 1 BTC = 100,000,000 SATS
const BTC_TO_SATS: f64 = 100_000_000.00;
/// API endpoint to fetch Bitcoin price in PHP.
const CLIENT_URL: &'static str =
    "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=php";

#[tokio::main]
async fn main() {
    // Initialize .env file
    dotenv::dotenv().expect("Failed to read .env file");

    // Create axum router
    let app = Router::new().route("/", post(convert_peso_to_sats));

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Fetches the latest Bitcoin price in PHP from CoinGecko client.
///
/// # Returns
///
/// Result containing either the `PriceResponse` struct with the fetched data or a `StatusCode` indicating an error.
async fn connect_to_client() -> Result<PriceResponse, StatusCode> {
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set");

    // Build request with API key header
    let response = match Client::new()
        .get(CLIENT_URL)
        .header("x-cg-demo-api-key", api_key)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Check for successful response status
    if response.status().is_success() {
        match response.json().await {
            Ok(data) => return Ok(data),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else {
        return Err(response.status());
    }
}

/// Converts a given PHP amount to Satoshis.
///
/// # Arguments
///
/// * `body` - Json containing the PHP amount to be converted.
///
/// # Returns
///
/// Result containing either a Json object with the converted value as a `ConvertedPeso` struct
/// or a `StatusCode` indicating an error in the client.
async fn convert_peso_to_sats(
    Json(body): Json<PriceData>,
) -> Result<Json<ConvertedPeso>, StatusCode> {
    // Fetch client response
    let client = match connect_to_client().await {
        Ok(data) => data,
        Err(status) => return Err(status),
    };

    let btc_to_php = client.bitcoin.php;

    // Calculate the equivalent amount in sats for the given PHP input
    let converted_sats = body.php / btc_to_php * BTC_TO_SATS;

    Ok(Json(ConvertedPeso {
        btc_to_php,
        input_php: body.php,
        output_sats: converted_sats,
    }))
}
