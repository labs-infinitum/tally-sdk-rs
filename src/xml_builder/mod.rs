//! Lower-level builders for Tally XML request envelopes.
//!
//! Prefer [`crate::TallyClient`] for common flows. Use [`XmlBuilder`] when you need
//! a custom export/import request (for example purchase item invoices) and then
//! send it with [`crate::TallyClient::post_xml`].

/// Constructs Tally XML envelopes for export and import requests.
pub struct XmlBuilder;

mod envelope;
mod export;
mod group;
mod helpers;
mod item_invoice;
mod ledger;
mod stock_item;
mod voucher;
