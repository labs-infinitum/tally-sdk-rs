# `tallyprime-sdk`

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

Add the crate from crates.io:

```toml
[dependencies]
tallyprime-sdk = "0.1"
```

If you want to reference it from Git:

```toml
[dependencies]
tallyprime-sdk = { git = "https://github.com/labs-infinitum/tallyprime-sdk" }
```

## Quick Start

```rust
use tallyprime_sdk::config::TallyConfig;
use tallyprime_sdk::TallyClient;

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
- `get_gst_computation(from, to) -> GstComputationReport`
- `get_gstr1(from, to) -> Gstr1Report` (voucher-derived; Tally has no builtin HTTP `GSTR-1` report ID)

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
use tallyprime_sdk::config::TallyConfig;
use tallyprime_sdk::{Group, TallyClient};

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
- [`fetch_gstr1.rs`](examples/fetch_gstr1.rs)
- [`create_ledger.rs`](examples/create_ledger.rs)
- [`create_ledger_entry.rs`](examples/create_ledger_entry.rs)

Run them with:

```bash
git clone https://github.com/labs-infinitum/tallyprime-sdk
cd tallyprime-sdk

cargo run --example fetch_all_accounts
cargo run --example fetch_all_groups
cargo run --example fetch_all_currencies
cargo run --example fetch_day_book -- --fy 2025-2026
cargo run --example fetch_day_book -- --from 20250401 --to 20250731 --verbose
cargo run --example fetch_trial_balance -- --fy 2025-2026
cargo run --example fetch_balance_sheet -- --fy 2025-2026
cargo run --example fetch_profit_and_loss -- --fy 2025-2026
cargo run --example fetch_gstr1 -- --fy 2025-2026
cargo run --example create_ledger
cargo run --example create_ledger -- --name "Acme Traders" --parent "Sundry Debtors"
cargo run --example create_ledger_entry -- --party "Acme Traders" --amount 1000 --date 20260701
```

Supported example flags:

- `--fy YYYY-YYYY`
- `--from YYYYMMDD`
- `--to YYYYMMDD`
- `--verbose` for day book voucher ledger-line output
- `--flat` for report exports without `EXPLODEFLAG`
- `--name`, `--parent`, `--opening-balance`, and `--debug` for ledger creation
- `--party`, `--account`, `--amount`, `--date`, `--bill-ref`, `--voucher-number`, `--voucher-type`, `--narration`, and `--debug` for ledger entries

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
cargo test --lib
cargo package
```

Unit tests under `src/` run with `cargo test` / `cargo test --lib` and do not need Tally.

Integration tests under `tests/` require a reachable TallyPrime instance and are marked `#[ignore]`:

```bash
cargo test -- --ignored
```

Set `TALLY_HOST`, `TALLY_PORT`, and optionally `TALLY_COMPANY` when running ignored tests. If no company is active and `TALLY_COMPANY` is not set, some flows will skip.

## Releases

Preferred path: run the **Create Release** workflow from the Actions tab.

1. Configure [Trusted Publishing](https://crates.io/docs/trusted-publishing) on crates.io for this repo, with workflow `release.yml`.
2. Choose a bump type:
   - `bugfix` → `x.y.z` → `x.y.(z+1)`
   - `minor` → `x.y.z` → `x.(y+1).0`
   - `major` → `x.y.z` → `(x+1).0.0`
3. Create Release updates `Cargo.toml` / `Cargo.lock`, commits to `main`, and tags `vX.Y.Z`.
4. It then dispatches the **Release** workflow, which publishes that tag to crates.io via OIDC (no `CARGO_REGISTRY_TOKEN` secret).

You can still publish by pushing a tag yourself:

```bash
git tag v0.1.0 && git push origin v0.1.0
```

The **Release** workflow also supports manual `workflow_dispatch` for publish-only / dry-run retries.

## Limitations

- The client is blocking today and uses `reqwest::blocking`.
- Voucher creation is not yet exposed as a single high-level `create_voucher(...)` client API.
- Some advanced Tally/TDL workflows still require the lower-level XML builder layer.

## License

This repository is licensed under the Apache License 2.0.

See [LICENSE](LICENSE) and [NOTICE](NOTICE).
