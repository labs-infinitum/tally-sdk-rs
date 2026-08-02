use tally_sdk_rust::config::TallyConfig;
use tally_sdk_rust::TallyClient;

pub fn client_from_env() -> TallyClient {
    let host = std::env::var("TALLY_HOST").unwrap_or_else(|_| "localhost".into());
    let port = std::env::var("TALLY_PORT")
        .ok()
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(9000);

    let cfg = TallyConfig {
        host: host.clone(),
        port,
        current_company: std::env::var("TALLY_COMPANY").ok(),
        ..Default::default()
    };

    let client = TallyClient::new(cfg).unwrap_or_else(|err| {
        eprintln!("Failed to initialize Tally client: {err}");
        std::process::exit(2);
    });

    if let Err(err) = client.test_connection() {
        eprintln!("Failed to connect to Tally at {host}:{port}: {err}");
        std::process::exit(2);
    }

    client
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
