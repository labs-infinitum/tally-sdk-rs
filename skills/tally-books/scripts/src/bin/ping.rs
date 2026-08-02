use serde_json::json;
use tally_books_skill_cli::client_from_env;

fn main() {
    let client = client_from_env();
    let company = client
        .active_company_name()
        .ok()
        .flatten()
        .unwrap_or_else(|| "(unknown)".into());

    println!(
        "{}",
        json!({
            "ok": true,
            "company": company,
            "host": std::env::var("TALLY_HOST").unwrap_or_else(|_| "localhost".into()),
            "port": std::env::var("TALLY_PORT")
                .ok()
                .and_then(|s| s.parse::<u16>().ok())
                .unwrap_or(9000),
        })
    );
}
