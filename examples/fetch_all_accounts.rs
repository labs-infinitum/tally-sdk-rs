mod common;

use common::{active_company_label, create_client_from_env};

fn main() {
    let client = create_client_from_env();
    let company = active_company_label(&client);

    let mut accounts = client.get_ledgers().unwrap_or_else(|err| {
        eprintln!("Failed to fetch accounts: {err}");
        std::process::exit(1);
    });
    accounts.sort_by(|a, b| a.name.cmp(&b.name));

    println!("Accounts in company: {company}");
    println!("Found {} account(s)", accounts.len());
    for account in accounts {
        println!(
            "{} | parent {}",
            account.name,
            account.parent.unwrap_or_else(|| "-".into())
        );
    }
}
