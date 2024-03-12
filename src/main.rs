use crate::api::{calculate_converted_amount, fetch_exchange_rates};
use std::io;

mod api;

fn is_valid_currency(currency: &str) -> bool {
    return currency.chars().all(|c| c.is_ascii_uppercase());
}

fn parse_amount(amount: &str) -> Option<f64> {
    match amount.parse::<f64>() {
        Ok(parsed_amount) if parsed_amount > 0.0 => Some(parsed_amount),
        _ => None,
    }
}

#[tokio::main]
async fn main() {
    let mut source_currency = String::new();
    let mut target_currency = String::new();
    let mut amount = String::new();
    let mut display_exchange_rates = String::new();
    // let mut redis_client = match mini_redis::client::connect("127.0.0.1:6379").await {
    //     Ok(client) => client,
    //     Err(err) => return Err(err.into()),
    // };

    println!("Currency Exchange Application.");
    println!("Please type the source currency code, like: \"USD\"\n");

    io::stdin()
        .read_line(&mut source_currency)
        .expect("Failed to read data");

    let source_currency = source_currency.trim();

    if !is_valid_currency(&source_currency) {
        return println!("Currency code has to consist only of uppercase alphabetic characters");
    }

    println!(
        "Would you like to see available exchange rates for your selected currency? Type (yes/no)\n"
    );

    io::stdin()
        .read_line(&mut display_exchange_rates)
        .expect("Failed to read data");

    let display_exchange_rates = display_exchange_rates.trim();

    if display_exchange_rates != "no" {
        println!("Displaying exchange rates for {}:\n", source_currency);
        match fetch_exchange_rates(source_currency).await {
            Ok(result) => {
                match result
                    .get("conversion_rates")
                    .and_then(|rates| rates.as_object())
                {
                    Some(conversion_rates) => {
                        for (currency, rate) in conversion_rates {
                            println!("{}: {}", currency, rate);
                        }
                    }
                    None => println!("No conversion rates found"),
                }
            }
            Err(e) => return println!("There was an error while processing api request: {}\n", e),
        }
    }

    println!("\nPlease type the source currency code, like: \"PLN\"\n");

    io::stdin()
        .read_line(&mut target_currency)
        .expect("Failed to read data");

    let target_currency = target_currency.trim();

    if !is_valid_currency(&target_currency) {
        return println!("Currency code has to consist only of uppercase alphabetic characters");
    }

    println!("\nPlease type the amount of currency to exchange, like: \"100\"\n");

    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read data");

    let amount = amount.trim();

    let parsed_amount = match parse_amount(amount) {
        Some(amount) => amount,
        None => {
            return println!("The amount has an invalid type. Try passing a positive number.");
        }
    };

    match calculate_converted_amount(source_currency, target_currency, &parsed_amount).await {
        Ok((converted_amount, exchange_rate)) => {
            println!("\nYour converted amount is:");
            println!(
                "{} {}, exchange rate ({}/{}): {}",
                (converted_amount * 100.0).round() / 100.0,
                target_currency,
                source_currency,
                target_currency,
                exchange_rate
            );
        }
        Err(e) => return println!("There was an error while processing api request: {}\n", e),
    }
}
