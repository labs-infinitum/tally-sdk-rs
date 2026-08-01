mod common;

use common::create_client_from_env;

fn main() {
    let client = create_client_from_env();

    let company = client.get_company_details().unwrap_or_else(|err| {
        eprintln!("Failed to fetch company details: {err}");
        std::process::exit(1);
    });

    let Some(company) = company else {
        eprintln!("No company details available.");
        std::process::exit(1);
    };

    println!("Company: {}", company.name);
    println!(
        "Formal name: {}",
        company.formal_name.as_deref().unwrap_or("-")
    );
    println!(
        "Base currency: {}",
        company.currency_name.as_deref().unwrap_or("-")
    );
    println!(
        "Books from: {}",
        company.books_from.as_deref().unwrap_or("-")
    );
    println!(
        "Starting from: {}",
        company.starting_from.as_deref().unwrap_or("-")
    );
    println!("Email: {}", company.email.as_deref().unwrap_or("-"));
    println!("Website: {}", company.website.as_deref().unwrap_or("-"));
    println!(
        "State / Country: {} / {}",
        company.state_name.as_deref().unwrap_or("-"),
        company.country_name.as_deref().unwrap_or("-")
    );
    println!("Pincode: {}", company.pincode.as_deref().unwrap_or("-"));
    if company.address.is_empty() {
        println!("Address: -");
    } else {
        println!("Address:");
        for line in &company.address {
            println!("  - {line}");
        }
    }
    println!(
        "Flags: accounting={} inventory={} gst={} billwise={} invoicing={} multicurrency={}",
        fmt_flag(company.is_accounting_on),
        fmt_flag(company.is_inventory_on),
        fmt_flag(company.is_gst_on),
        fmt_flag(company.is_bill_wise_on),
        fmt_flag(company.is_invoicing_on),
        fmt_flag(company.is_multi_currency_on)
    );
}

fn fmt_flag(value: Option<bool>) -> &'static str {
    match value {
        Some(true) => "Yes",
        Some(false) => "No",
        None => "-",
    }
}
