fn main() {
    println!("tally-sdk-rust");
    println!();
    println!("Run an example binary instead of the default crate binary.");
    println!("Examples:");
    println!("  cargo run --example fetch_all_accounts");
    println!("  cargo run --example fetch_all_groups");
    println!("  cargo run --example fetch_all_currencies");
    println!("  cargo run --example fetch_trial_balance -- --fy 2025-2026");
    println!("  cargo run --example fetch_balance_sheet -- --fy 2025-2026");
    println!("  cargo run --example fetch_profit_and_loss -- --fy 2025-2026");
    println!("  cargo run --example fetch_day_book -- --fy 2025-2026 --verbose");
    println!();
    println!("Environment:");
    println!("  TALLY_HOST defaults to localhost");
    println!("  TALLY_PORT defaults to 9000");
    println!("  TALLY_COMPANY is optional");
}
