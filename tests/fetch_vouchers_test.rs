use tally_sdk_rust::{TallyClient};
use tally_sdk_rust::config::TallyConfig;

fn cfg_from_env() -> TallyConfig {
    let host = std::env::var("TALLY_HOST").unwrap_or_else(|_| "localhost".into());
    let port = std::env::var("TALLY_PORT").ok().and_then(|s| s.parse::<u16>().ok()).unwrap_or(9000);
    TallyConfig { host, port, ..Default::default() }
}

#[test]
fn fetch_all_vouchers() {
    let client = TallyClient::new(cfg_from_env()).expect("client");
    // just ensure we can fetch and parse some vouchers
    let vouchers = client.get_vouchers().expect("fetch vouchers");
    // We don't assert count because it depends on company data; just ensure call succeeded
    // Optionally print a few for debugging
    if let Some((vchtype, date)) = vouchers.get(0) {
        println!("first voucher: type={}, date={}", vchtype, date);
    }
}


