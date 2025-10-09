use tally_sdk_rust::client::voucher_parser;

fn main() {
    // Read the XML file
    let xml = std::fs::read_to_string("vouchers.xml")
        .expect("Failed to read vouchers.xml");
    
    // Parse vouchers using our parser
    let vouchers = voucher_parser::parse_vouchers_from_xml(&xml);
    
    // Display results
    println!("Parsed {} vouchers\n", vouchers.len());
    
    for voucher in &vouchers {
        println!("{}", voucher);
        println!("{}", "-".repeat(80));
    }
    
    // Summary
    println!("\n=== Summary ===");
    println!("Total vouchers: {}", vouchers.len());
    let total_items: usize = vouchers.iter().map(|v| v.items.len()).sum();
    println!("Total items: {}", total_items);
    let total_entries: usize = vouchers.iter().map(|v| v.entries.len()).sum();
    println!("Total ledger entries: {}", total_entries);
}
