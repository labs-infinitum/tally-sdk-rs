use tally_sdk_rust::config::TallyConfig;
use tally_sdk_rust::{TallyClient, Voucher, VoucherEntry};

const FROM_DATE: &str = "20250401";
const TO_DATE: &str = "20250731";

fn main() {
    let verbose = std::env::args().any(|arg| arg == "--verbose");
    let cfg = TallyConfig {
        host: std::env::var("TALLY_HOST").unwrap_or_else(|_| "localhost".into()),
        port: std::env::var("TALLY_PORT")
            .ok()
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(9000),
        current_company: std::env::var("TALLY_COMPANY").ok(),
        ..Default::default()
    };

    let client = match TallyClient::new(cfg) {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to initialize Tally client: {err}");
            std::process::exit(1);
        }
    };

    if let Err(err) = client.test_connection() {
        eprintln!("Failed to connect to Tally at localhost:9000: {err}");
        std::process::exit(1);
    }

    let active_company = match client.active_company_name() {
        Ok(name) => name,
        Err(err) => {
            eprintln!("Failed to determine active company: {err}");
            std::process::exit(1);
        }
    };

    println!(
        "Fetching vouchers between 2025-04-01 and 2025-07-31 from company: {}",
        active_company.as_deref().unwrap_or("(no active company)")
    );

    let vouchers = match client.get_vouchers_in_range(FROM_DATE, TO_DATE) {
        Ok(vouchers) => vouchers,
        Err(err) => {
            eprintln!("Failed to fetch vouchers: {err}");
            std::process::exit(1);
        }
    };

    println!("Found {} voucher(s)", vouchers.len());
    for voucher in &vouchers {
        println!("{}", summarize_voucher(voucher));
        if verbose {
            for entry in &voucher.entries {
                println!("    {}", summarize_entry(entry));
            }
        }
    }
}

fn summarize_voucher(voucher: &Voucher) -> String {
    let voucher_number = voucher.voucher_number.as_deref().unwrap_or("-");
    let party = voucher.party_ledger_name.as_deref().unwrap_or("-");
    let amount = voucher
        .entries
        .iter()
        .find(|entry| entry.is_party_ledger)
        .map(|entry| entry.amount)
        .or(voucher.amount.map(|amount| amount.abs()))
        .unwrap_or(0.0);

    format!(
        "{} | {} | no. {} | party {} | amount {:.2}",
        format_date(&voucher.date_yyyymmdd),
        voucher.voucher_type,
        voucher_number,
        party,
        amount
    )
}

fn summarize_entry(entry: &VoucherEntry) -> String {
    let side = if entry.is_debit { "Dr" } else { "Cr" };
    let party = if entry.is_party_ledger {
        " [Party]"
    } else {
        ""
    };
    format!(
        "{} | {} {:.2}{}",
        entry.ledger_name, side, entry.amount, party
    )
}

fn format_date(date: &str) -> String {
    if date.len() == 8 {
        format!("{}-{}-{}", &date[0..4], &date[4..6], &date[6..8])
    } else {
        date.to_string()
    }
}
