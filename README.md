# `tally-sdk-rust`

Rust SDK for integrating with TallyPrime over XML/HTTP.

This crate provides a blocking client for:

- reading masters such as ledgers, groups, stock items, and currencies
- reading vouchers and day-book style voucher ranges
- reading built-in reports such as trial balance, balance sheet, and profit and loss
- creating masters such as ledgers, groups, and stock items

The SDK is built around Tally's XML interface and uses typed Rust models for both inputs and outputs.

## Status

This project is working against a live Tally instance and has integration tests for:

- group creation
- ledger creation
- stock item creation
- voucher fetching
- purchase voucher creation using the lower-level XML builder

The high-level client is stable for read flows and master creation. Advanced voucher import flows exist, but some of them are still exposed through lower-level XML builder utilities rather than a dedicated top-level client method.

## Requirements

- Rust 1.75+ recommended
- TallyPrime running with XML over HTTP enabled
- Default endpoint: `http://localhost:9000`

If you use a specific company, set it explicitly with `TALLY_COMPANY` or `TallyConfig.current_company`. If you do not, the SDK will try to discover the active company loaded in Tally.

## Installation

If you are using this crate from another local workspace:

```toml
[dependencies]
tally-sdk-rust = { path = "../tally-sdk-rust" }
```

If you want to reference it from Git:

```toml
[dependencies]
tally-sdk-rust = { git = "https://github.com/labs-infinitum/tally-sdk-rs" }
```

## Quick Start

```rust
use tally_sdk_rust::config::TallyConfig;
use tally_sdk_rust::TallyClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TallyClient::new(TallyConfig {
        host: "localhost".into(),
        port: 9000,
        current_company: Some("My Company Pvt. Ltd.".into()),
        ..Default::default()
    })?;

    client.test_connection()?;

    let company = client.active_company_name()?;
    println!("Active company: {:?}", company);

    let ledgers = client.get_ledgers()?;
    println!("Found {} ledgers", ledgers.len());

    let vouchers = client.get_vouchers_in_range("20250401", "20250731")?;
    println!("Found {} vouchers", vouchers.len());

    Ok(())
}
```

## Configuration

The client is configured through [`TallyConfig`](src/config.rs):

```rust
#[derive(Debug, Clone)]
pub struct TallyConfig {
    pub host: String,
    pub port: u16,
    pub timeout_secs: u64,
    pub retry_attempts: u32,
    pub current_company: Option<String>,
    pub tally_net_account: Option<String>,
    pub tally_net_password: Option<String>,
}
```

Defaults:

- `host = "localhost"`
- `port = 9000`
- `timeout_secs = 30`
- `retry_attempts = 3`

## Public API

Main entry point: [`TallyClient`](src/client/mod.rs)

Connection and session:

- `TallyClient::new`
- `TallyClient::test_connection`
- `TallyClient::active_company_name`

Master reads:

- `get_ledgers() -> Vec<LedgerSummary>`
- `get_groups() -> Vec<GroupSummary>`
- `get_stock_items() -> Vec<StockItemSummary>`
- `get_currencies() -> Vec<CurrencySummary>`

Master writes:

- `create_ledger(&Ledger) -> ImportResult`
- `create_group(&Group) -> ImportResult`
- `create_stock_item(&StockItem) -> ImportResult`

Debug variants are also available and print the raw XML request/response:

- `create_ledger_debug`
- `create_group_debug`
- `create_stock_item_debug`

Voucher reads:

- `get_vouchers(from, to) -> Vec<Voucher>`
- `get_vouchers_in_range(from, to) -> Vec<Voucher>`

Reports:

- `get_trial_balance(from, to, explode_flag) -> Vec<TrialBalanceEntry>`
- `get_balance_sheet(from, to, explode_flag) -> Vec<BalanceSheetEntry>`
- `get_profit_and_loss(from, to, explode_flag) -> Vec<ProfitAndLossEntry>`

## Typed Results

Collection reads return typed summaries rather than tuples:

- [`LedgerSummary`](src/models/collection.rs)
- [`GroupSummary`](src/models/collection.rs)
- [`StockItemSummary`](src/models/collection.rs)
- [`CurrencySummary`](src/models/collection.rs)

Create/import calls return [`ImportResult`](src/models/import.rs), which includes:

- `created`
- `altered`
- `deleted`
- `combined`
- `ignored`
- `errors`
- `cancelled`
- `exceptions`
- `last_voucher_id`
- `last_master_id`
- `line_errors`

Example:

```rust
use tally_sdk_rust::config::TallyConfig;
use tally_sdk_rust::{Group, TallyClient};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TallyClient::new(TallyConfig::default())?;

    let result = client.create_group(&Group {
        name: "SDK Demo Group".into(),
        parent: None,
        group_type: None,
        alias: None,
        basic_group_is_calculable: None,
        is_addable: None,
        is_subledger: None,
        addl_alloc_type: None,
        as_original: None,
        affects_gross_profit: None,
        hsn_applicable_from: None,
        hsn_code: None,
        hsn_description: None,
        hsn_classification_name: None,
        hsn_source_of_details: None,
        gst_applicable_from: None,
        gst_taxability: None,
        gst_source_of_details: None,
        gst_classification_name: None,
        gst_rate_duty_head: None,
        gst_rate_valuation_type: None,
        gst_rate: None,
        gst_state_name: None,
    })?;

    println!(
        "created={}, altered={}, exceptions={}",
        result.created, result.altered, result.exceptions
    );

    Ok(())
}
```

## Examples

The crate includes runnable examples under [`examples/`](examples):

- [`fetch_all_accounts.rs`](examples/fetch_all_accounts.rs)
- [`fetch_all_groups.rs`](examples/fetch_all_groups.rs)
- [`fetch_all_currencies.rs`](examples/fetch_all_currencies.rs)
- [`fetch_day_book.rs`](examples/fetch_day_book.rs)
- [`fetch_trial_balance.rs`](examples/fetch_trial_balance.rs)
- [`fetch_balance_sheet.rs`](examples/fetch_balance_sheet.rs)
- [`fetch_profit_and_loss.rs`](examples/fetch_profit_and_loss.rs)

Run them with:

```bash
git clone https://github.com/labs-infinitum/tally-sdk-rs
cd tally-sdk-rs

cargo run --example fetch_all_accounts
cargo run --example fetch_all_groups
cargo run --example fetch_all_currencies
cargo run --example fetch_day_book -- --fy 2025-2026
cargo run --example fetch_day_book -- --from 20250401 --to 20250731 --verbose
cargo run --example fetch_trial_balance -- --fy 2025-2026
cargo run --example fetch_balance_sheet -- --fy 2025-2026
cargo run --example fetch_profit_and_loss -- --fy 2025-2026
```

Supported example flags:

- `--fy YYYY-YYYY`
- `--from YYYYMMDD`
- `--to YYYYMMDD`
- `--verbose` for day book voucher ledger-line output
- `--flat` for report exports without `EXPLODEFLAG`

Environment variables used by the examples:

- `TALLY_HOST`
- `TALLY_PORT`
- `TALLY_COMPANY`

## Date Handling

The SDK expects date inputs in `YYYYMMDD` format for most public methods.

Example:

- `20250401`
- `20260331`

For voucher reads, `get_vouchers_in_range` applies an exact client-side range filter after parsing the XML response. This exists because Tally's voucher/day book exports are not always consistent about honoring date filters across environments.

## Lower-Level XML Access

For advanced or not-yet-wrapped flows, the crate also exposes the lower-level XML pieces:

- [`xml_builder`](src/xml_builder/mod.rs)
- [`client::post_xml`](src/client/http.rs)
- [`client::parse_simple_response`](src/client/parse.rs)

That is how the current purchase item-invoice integration test is implemented.

## Development

Useful commands:

```bash
cargo fmt
cargo check --examples
cargo test
```

The integration tests expect a reachable Tally instance. If no company is active and `TALLY_COMPANY` is not set, some tests will skip or fail depending on the flow.

## Limitations

- The client is blocking today and uses `reqwest::blocking`.
- Voucher creation is not yet exposed as a single high-level `create_voucher(...)` client API.
- Some advanced Tally/TDL workflows still require the lower-level XML builder layer.

## License

Add the appropriate license for this repository here.
