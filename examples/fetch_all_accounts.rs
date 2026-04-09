mod common;

use common::{active_company_label, create_client_from_env};

fn main() {
    let client = create_client_from_env();
    let company = active_company_label(&client);

    let mut accounts = client.get_ledgers().unwrap_or_else(|err| {
        eprintln!("Failed to fetch accounts: {err}");
        std::process::exit(1);
    });
    accounts.sort_by(|a, b| a.0.cmp(&b.0));

    println!("Accounts in company: {company}");
    println!("Found {} account(s)", accounts.len());
    for (name, parent) in accounts {
        println!("{} | parent {}", name, parent.unwrap_or_else(|| "-".into()));
    }
}
