use spinners::{Spinner, Spinners};

const COINGECKO_API: &str = "https://api.coingecko.com/api/v3/";

pub async fn convert(
    amount: &f64,
    input_currency: &String,
    output_currency: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut spinner = Spinner::new(Spinners::Arrow, "Requesting prices from coingecko".into());
    let response: serde_json::Value = reqwest::get(format!(
        "{}simple/price?ids={}&vs_currencies={}",
        COINGECKO_API, input_currency, output_currency
    ))
    .await?
    .json()
    .await?;
    spinner.stop_with_symbol("✅");

    let price = response
        .get(input_currency)
        .unwrap()
        .get(output_currency)
        .unwrap()
        .as_f64()
        .unwrap();

    println!(
        "⭐️ {} {} = {:.2} {}",
        amount,
        input_currency,
        amount * price,
        output_currency
    );
    Ok(())
}
