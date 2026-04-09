mod common;

use common::{
    active_company_label, arg_value, create_client_from_env, format_yyyymmdd, has_flag,
    resolve_financial_period,
};
use tally_sdk_rust::{Voucher, VoucherEntry};

fn main() {
    let client = create_client_from_env();
    let company = active_company_label(&client);
    let (from_date, to_date) = resolve_day_book_period(&client);
    let verbose = has_flag("--verbose");

    let vouchers = client
        .get_vouchers_in_range(&from_date, &to_date)
        .unwrap_or_else(|err| {
            eprintln!("Failed to fetch day book vouchers: {err}");
            std::process::exit(1);
        });

    println!(
        "Day Book for {} from {} to {}",
        company,
        format_yyyymmdd(&from_date),
        format_yyyymmdd(&to_date)
    );
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
        format_yyyymmdd(&voucher.date_yyyymmdd),
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

fn resolve_day_book_period(client: &tally_sdk_rust::TallyClient) -> (String, String) {
    if arg_value("--fy").is_some() || arg_value("--from").is_some() || arg_value("--to").is_some() {
        return resolve_financial_period();
    }

    let vouchers = client.get_vouchers(None, None).unwrap_or_else(|err| {
        eprintln!("Failed to inspect vouchers for default day book period: {err}");
        std::process::exit(1);
    });

    let latest_date = vouchers
        .iter()
        .map(|voucher| voucher.date_yyyymmdd.as_str())
        .filter(|date| date.len() == 8 && date.bytes().all(|byte| byte.is_ascii_digit()))
        .max()
        .map(str::to_owned);

    match latest_date {
        Some(date) => financial_year_for_date(&date),
        None => resolve_financial_period(),
    }
}

fn financial_year_for_date(date: &str) -> (String, String) {
    let year = date[0..4].parse::<i32>().unwrap_or_else(|_| {
        eprintln!("Invalid voucher date `{date}` while deriving default day book period.");
        std::process::exit(1);
    });
    let month = date[4..6].parse::<u32>().unwrap_or_else(|_| {
        eprintln!("Invalid voucher date `{date}` while deriving default day book period.");
        std::process::exit(1);
    });

    let start_year = if month >= 4 { year } else { year - 1 };
    (
        format!("{start_year}0401"),
        format!("{}0331", start_year + 1),
    )
}
