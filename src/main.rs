use tally_sdk_rust::{config::TallyConfig, TallyClient};

fn main() {
    println!("=== Tally SDK - Fetch Ledgers Example ===\n");

    // Create TallyClient with custom host
    let config = TallyConfig {
        host: "192.168.128.2".to_string(),
        port: 9000,
        timeout_secs: 30,
        retry_attempts: 3,
        current_company: std::env::var("TALLY_COMPANY").ok(),
        tally_net_account: None,
        tally_net_password: None,
    };
    println!("Connecting to Tally at {}:{}...", config.host, config.port);

    let client = match TallyClient::new(config) {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to create Tally client: {}", e);
            return;
        }
    };

    // Test connection
    println!("Testing connection...");
    match client.test_connection() {
        Ok(_) => println!("✓ Connection successful\n"),
        Err(e) => {
            eprintln!("✗ Connection failed: {}", e);
            eprintln!("\nMake sure Tally is running and listening on port 9000");
            return;
        }
    }

    // Fetch ledgers from Tally server
    println!("Fetching ledgers from Tally server...");
    match client.get_ledgers() {
        Ok(ledgers) => {
            println!("\n=== Ledgers ===");
            println!("Total ledgers found: {}\n", ledgers.len());

            for (name, parent) in &ledgers {
                match parent {
                    Some(p) => println!("  {} (Parent: {})", name, p),
                    None => println!("  {}", name),
                }
            }

            // Summary
            println!("\n=== Summary ===");
            println!("Total ledgers: {}", ledgers.len());
            let with_parent = ledgers.iter().filter(|(_, p)| p.is_some()).count();
            let without_parent = ledgers.len() - with_parent;
            println!("Ledgers with parent: {}", with_parent);
            println!("Ledgers without parent: {}", without_parent);
        }
        Err(e) => {
            eprintln!("Failed to fetch ledgers: {}", e);
        }
    }

    // Fetch vouchers from Tally server
    println!("\n\nFetching vouchers from Tally server...");
    // Fetch all vouchers (no date filter = from 1900-01-01 to today)
    /* match client.get_vouchers(None, None) {
        Ok(vouchers) => {
            println!("\n=== Vouchers ===");
            println!("Total vouchers found: {}\n", vouchers.len());

            for voucher in &vouchers {
                println!("{}", voucher);
                println!("{}", "-".repeat(80));
            }

            // Summary
            println!("\n=== Vouchers Summary ===");
            println!("Total vouchers: {}", vouchers.len());
            let total_items: usize = vouchers.iter().map(|v| v.items.len()).sum();
            println!("Total items: {}", total_items);
            let total_entries: usize = vouchers.iter().map(|v| v.entries.len()).sum();
            println!("Total ledger entries: {}", total_entries);
        }
        Err(e) => {
            eprintln!("Failed to fetch vouchers: {}", e);
        }
    } */
}
