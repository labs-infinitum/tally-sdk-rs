//! Blocking Rust SDK for [TallyPrime](https://tallysolutions.com/) over XML/HTTP.
//!
//! This crate talks to a running TallyPrime instance (default `http://localhost:9000`)
//! using Tally's XML messaging format. Requests and responses use UTF-16 so currency
//! symbols such as `₹` round-trip correctly.
//!
//! # Requirements
//!
//! - TallyPrime with XML/HTTP enabled
//! - Dates in `YYYYMMDD` form for most public methods (for example `20250401`)
//! - Optional company selection via [`config::TallyConfig::current_company`] or the
//!   active company loaded in Tally
//!
//! # Quick start
//!
//! ```no_run
//! use tallyprime_sdk::config::TallyConfig;
//! use tallyprime_sdk::TallyClient;
//!
//! fn main() -> tallyprime_sdk::Result<()> {
//!     let client = TallyClient::new(TallyConfig {
//!         host: "localhost".into(),
//!         port: 9000,
//!         current_company: Some("My Company Pvt. Ltd.".into()),
//!         ..Default::default()
//!     })?;
//!
//!     client.test_connection()?;
//!     let ledgers = client.get_ledgers()?;
//!     println!("Found {} ledgers", ledgers.len());
//!     Ok(())
//! }
//! ```
//!
//! # Capabilities
//!
//! - **Masters**: read/create ledgers, groups, stock items; read currencies and rich
//!   ledger/stock details
//! - **Vouchers**: create accounting vouchers; fetch day-book style voucher ranges
//! - **Reports**: trial balance, balance sheet, profit and loss, GST computation
//! - **GST**: voucher-derived GSTR-1 style summary ([`TallyClient::get_gstr1`]) when
//!   Tally does not expose a builtin HTTP `GSTR-1` report
//! - **Advanced**: lower-level [`xml_builder::XmlBuilder`] and [`TallyClient::post_xml`]
//!   for custom Tally XML flows
//!
//! # Modules
//!
//! - [`client`] — [`TallyClient`] and response helpers
//! - [`config`] — connection and company settings
//! - [`models`] — typed request/response structs
//! - [`xml_builder`] — XML envelope builders for advanced integrations
//! - [`errors`] — [`TallyError`] and [`Result`]

#![warn(missing_docs)]

pub mod client;
pub mod config;
pub mod errors;
pub mod models;
pub mod xml_builder;

pub use crate::client::TallyClient;
pub use crate::errors::*;
pub use crate::models::*;

pub use crate::client::voucher_parser;
