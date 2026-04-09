mod common;

use common::{active_company_label, create_client_from_env};

fn main() {
    let client = create_client_from_env();
    let company = active_company_label(&client);

    let mut currencies = client.get_currencies().unwrap_or_else(|err| {
        eprintln!("Failed to fetch currencies: {err}");
        std::process::exit(1);
    });
    currencies.sort();

    println!("Currencies in company: {company}");
    println!("Found {} currency/currencies", currencies.len());
    for currency in currencies {
        println!("{currency}");
    }
}
