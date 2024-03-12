use reqwest;
use std::error::Error;
// use mini_redis::client::Client;

pub async fn fetch_exchange_rates(
    source_currency: &str,
    // redis_client: &mut Client
) -> Result<serde_json::Value, Box<dyn Error>> {
    // let result = redis_client.get(source_currency).await;

    let api_key = "b2f4ca5eb763c832e6557bec";
    let url = format!(
        "https://v6.exchangerate-api.com/v6/{}/latest/{}",
        api_key, source_currency
    );
    let response = reqwest::get(&url)
        .await?
        .json::<serde_json::Value>()
        .await?;

    if response["error"].is_object() {
        return Err(format!(
            "Error while handling the request: {}",
            response["error"]["type"]
        )
        .into());
    }

    // let response_bytes = serde_json::to_vec(&response)?;
    // redis_client.set(source_currency, Bytes::from(response_bytes)).await;

    Ok(response)
}

pub async fn calculate_converted_amount(
    source_currency: &str,
    target_currency: &str,
    amount: &f64,
    // redis_client:&mut Client
) -> Result<(f64, f64), Box<dyn Error>> {
    let rates = fetch_exchange_rates(source_currency).await?;
    let target_rate = rates["conversion_rates"][target_currency]
        .as_f64()
        .ok_or("Invalid currency code")?;
    let converted_amount = amount * target_rate;

    return Ok((converted_amount, target_rate));
}
