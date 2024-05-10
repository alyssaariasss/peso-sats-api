# Peso to Sats Converter

This project provides a simple API to convert a given amount in PHP to its equivalent in Satoshis, based on the current Bitcoin price fetched from the CoinGecko API.

## Setup

1. Clone the repository:
```
git clone https://github.com/alyssaariasss/peso-sats-api.git
cd peso-sats-api
```

2. Install dependencies:
```
cargo build
```

3. Create a .env file in the project root directory and add your CoinGecko API key. Follow [this guide](https://support.coingecko.com/hc/en-us/articles/21880397454233-User-Guide-How-to-sign-up-for-CoinGecko-Demo-API-and-generate-an-API-key) to generate a demo API key.
```
API_KEY="your_api_key"
```

## Usage

1. Start the server:
```
cargo run
```

2. Send a **POST** request to `http://localhost:3000/php_to_sats` with a query parameter **amount** containing the PHP amount to be converted.
```
amount=10.0
```

3. The API will respond with the equivalent amount in sats, together with the current bitcoin to peso conversion rate, and your PHP input amount using this format:
```
{
  "btc_to_peso": 0.0,
  "input_php": 0.0,
  "output_sats": 0.0
}
```
