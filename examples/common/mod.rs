#![allow(dead_code)]

use chrono::{Datelike, Local};
use tally_sdk_rust::config::TallyConfig;
use tally_sdk_rust::TallyClient;

pub fn create_client_from_env() -> TallyClient {
    let cfg = TallyConfig {
        host: std::env::var("TALLY_HOST").unwrap_or_else(|_| "localhost".into()),
        port: std::env::var("TALLY_PORT")
            .ok()
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(9000),
        current_company: std::env::var("TALLY_COMPANY").ok(),
        ..Default::default()
    };

    let client = TallyClient::new(cfg).unwrap_or_else(|err| {
        eprintln!("Failed to initialize Tally client: {err}");
        std::process::exit(1);
    });

    if let Err(err) = client.test_connection() {
        eprintln!("Failed to connect to Tally: {err}");
        std::process::exit(1);
    }

    client
}

pub fn active_company_label(client: &TallyClient) -> String {
    client
        .active_company_name()
        .unwrap_or_else(|err| {
            eprintln!("Failed to determine active company: {err}");
            std::process::exit(1);
        })
        .unwrap_or_else(|| "(no active company)".into())
}

pub fn has_flag(flag: &str) -> bool {
    std::env::args().any(|arg| arg == flag)
}

pub fn arg_value(flag: &str) -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    args.windows(2)
        .find(|window| window[0] == flag)
        .map(|window| window[1].clone())
}

pub fn resolve_financial_period() -> (String, String) {
    if let Some(fy) = arg_value("--fy") {
        if let Some(period) = parse_financial_year(&fy) {
            return period;
        }
        eprintln!("Invalid --fy value `{fy}`. Expected YYYY-YYYY, for example 2025-2026.");
        std::process::exit(1);
    }

    match (arg_value("--from"), arg_value("--to")) {
        (Some(from), Some(to)) => (from, to),
        (None, None) => current_financial_year(),
        _ => {
            eprintln!("Use either both --from and --to, or provide --fy.");
            std::process::exit(1);
        }
    }
}

pub fn format_yyyymmdd(date: &str) -> String {
    if date.len() == 8 {
        format!("{}-{}-{}", &date[0..4], &date[4..6], &date[6..8])
    } else {
        date.to_string()
    }
}

fn current_financial_year() -> (String, String) {
    let today = Local::now().date_naive();
    let start_year = if today.month() >= 4 {
        today.year()
    } else {
        today.year() - 1
    };

    (
        format!("{start_year}0401"),
        format!("{}0331", start_year + 1),
    )
}

fn parse_financial_year(value: &str) -> Option<(String, String)> {
    let (start, end) = value.split_once('-')?;
    let start_year = start.parse::<i32>().ok()?;
    let end_year = end.parse::<i32>().ok()?;
    if end_year != start_year + 1 {
        return None;
    }
    Some((format!("{start_year}0401"), format!("{end_year}0331")))
}
