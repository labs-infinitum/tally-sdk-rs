mod common;

use common::{active_company_label, create_client_from_env};

fn main() {
    let client = create_client_from_env();
    let company = active_company_label(&client);

    let mut currencies = client.get_currencies().unwrap_or_else(|err| {
        eprintln!("Failed to fetch currencies: {err}");
        std::process::exit(1);
    });
    currencies.sort_by(|a, b| a.name.cmp(&b.name));

    println!("Currencies in company: {company}");
    println!("Found {} currency/currencies", currencies.len());
    for currency in currencies {
        println!(
            "{} | original {} | mailing {} | expanded {} | decimals {}",
            currency.name,
            currency.original_name.as_deref().unwrap_or("-"),
            currency.mailing_name.as_deref().unwrap_or("-"),
            currency.expanded_symbol.as_deref().unwrap_or("-"),
            currency
                .decimal_places
                .map(|value| value.to_string())
                .unwrap_or_else(|| "-".into())
        );
    }
}
