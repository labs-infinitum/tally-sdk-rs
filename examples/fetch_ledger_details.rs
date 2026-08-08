mod common;

use common::{active_company_label, create_client_from_env};

fn main() {
    let client = create_client_from_env();
    let company = active_company_label(&client);

    let ledgers = client.get_ledger_details().unwrap_or_else(|err| {
        eprintln!("Failed to fetch ledger details: {err}");
        std::process::exit(1);
    });
    let items = client.get_stock_item_details().unwrap_or_else(|err| {
        eprintln!("Failed to fetch stock item details: {err}");
        std::process::exit(1);
    });

    println!("Company: {company}");
    println!("Ledger details: {}", ledgers.len());
    for ledger in ledgers.iter().take(10) {
        println!(
            "{} | parent {} | gstin {} | addr {} | opening {:?}",
            ledger.name,
            ledger.parent.as_deref().unwrap_or("-"),
            ledger.party_gstin.as_deref().unwrap_or("-"),
            if ledger.address.is_empty() {
                "-".into()
            } else {
                ledger.address.join("; ")
            },
            ledger.opening_balance
        );
    }
    if ledgers.len() > 10 {
        println!("... and {} more", ledgers.len() - 10);
    }

    println!("\nStock item details: {}", items.len());
    for item in items.iter().take(10) {
        println!(
            "{} | units {} | hsn {} | supply {} | rate {:?}",
            item.name,
            item.base_units.as_deref().unwrap_or("-"),
            item.hsn_code.as_deref().unwrap_or("-"),
            item.gst_type_of_supply.as_deref().unwrap_or("-"),
            item.gst_rate.or(item.opening_rate)
        );
    }
    if items.len() > 10 {
        println!("... and {} more", items.len() - 10);
    }
}
