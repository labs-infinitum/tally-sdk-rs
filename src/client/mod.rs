//! Blocking HTTP client for TallyPrime XML integration.
//!
//! The main type is [`TallyClient`]. Methods are implemented across this module
//! tree (`company`, `masters`, `vouchers`, `reports`, …) but all appear on
//! `TallyClient` in rustdoc.
//!
//! Advanced helpers:
//! - [`parse_simple_response`] — parse import/create response counters
//! - [`voucher_parser`] — parse raw Day Book / voucher XML

use crate::config::TallyConfig;
use crate::errors::{Result, TallyError};
use crate::xml_builder::XmlBuilder;
use std::sync::Mutex;

mod company;
mod extract;
mod gst_parser;
mod http;
mod masters;
pub mod parse;
mod report_parser;
mod reports;
pub mod voucher_parser;
mod vouchers;

/// Blocking client for TallyPrime XML/HTTP.
///
/// Construct with [`TallyClient::new`], optionally call [`Self::test_connection`],
/// then use the typed master/voucher/report methods. Dates are generally
/// `YYYYMMDD`. Company context comes from [`TallyConfig::current_company`] or
/// the company currently loaded in Tally.
pub struct TallyClient {
    cfg: TallyConfig,
    http: reqwest::blocking::Client,
    base_url: String,
    current_company: Mutex<Option<String>>,
}

impl TallyClient {
    /// Build a client from [`TallyConfig`].
    ///
    /// Configures a blocking `reqwest` client with `text/xml;charset=utf-16`
    /// and optional Tally.NET headers.
    pub fn new(cfg: TallyConfig) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        // UTF-16 is required for currency/special symbols such as ₹.
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("text/xml;charset=utf-16"),
        );
        headers.insert(
            reqwest::header::CACHE_CONTROL,
            reqwest::header::HeaderValue::from_static("no-cache"),
        );
        if let (Some(acc), Some(pw)) = (&cfg.tally_net_account, &cfg.tally_net_password) {
            headers.insert(
                "X-Tally-Account",
                reqwest::header::HeaderValue::from_str(acc)
                    .unwrap_or(reqwest::header::HeaderValue::from_static("")),
            );
            headers.insert(
                "X-Tally-Password",
                reqwest::header::HeaderValue::from_str(pw)
                    .unwrap_or(reqwest::header::HeaderValue::from_static("")),
            );
        }

        let http = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(cfg.timeout_secs))
            .build()
            .map_err(|e| TallyError::Unexpected(e.to_string()))?;
        let base_url = format!("http://{}:{}", cfg.host, cfg.port);
        Ok(Self {
            current_company: Mutex::new(cfg.current_company.clone()),
            cfg,
            http,
            base_url,
        })
    }

    /// Probe Tally with a lightweight company-list export.
    ///
    /// Returns `Ok(true)` when Tally answers successfully.
    pub fn test_connection(&self) -> Result<bool> {
        let xml = XmlBuilder::create_company_list_export_request()?;
        let _resp = self.post_raw_xml(&xml)?;
        Ok(true)
    }
}

/// Parse a Tally import/create XML response into [`crate::ImportResult`].
pub use parse::parse_simple_response_public as parse_simple_response;
