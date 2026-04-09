use crate::config::TallyConfig;
use crate::errors::{Result, TallyError};
use crate::models::{Group, Ledger, StockItem, Voucher};
use crate::xml_builder::XmlBuilder;
use regex::Regex;
use std::sync::Mutex;

mod extract;
mod http;
pub mod parse;
pub mod voucher_parser;

use crate::client::extract::{
    extract_groups_from_xml, extract_ledgers_from_xml, extract_stock_items_from_xml,
};

pub struct TallyClient {
    cfg: TallyConfig,
    http: reqwest::blocking::Client,
    base_url: String,
    current_company: Mutex<Option<String>>,
}

impl TallyClient {
    pub fn new(cfg: TallyConfig) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("text/xml"),
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

    pub fn test_connection(&self) -> Result<bool> {
        let header = serde_json::json!({
            "VERSION": "1",
            "TALLYREQUEST": "EXPORT",
            "TYPE": "COLLECTION",
            "ID": "Company",
        })
        .as_object()
        .unwrap()
        .clone();
        let mut body = serde_json::Map::new();
        let mut desc = serde_json::Map::new();
        let mut stat = serde_json::Map::new();
        stat.insert(
            "SVEXPORTFORMAT".into(),
            serde_json::Value::String("$$SysName:XML".into()),
        );
        desc.insert("STATICVARIABLES".into(), serde_json::Value::Object(stat));
        body.insert("DESC".into(), serde_json::Value::Object(desc));
        let xml = XmlBuilder::create_envelope(&header, Some(&body))?;
        let _resp = self.post_xml(&xml)?;
        Ok(true)
    }

    pub fn create_ledger(&self, ledger: &Ledger) -> Result<serde_json::Value> {
        ledger.validate()?;
        let map = ledger.to_map();
        let xml = XmlBuilder::create_ledger_request(&map)?;
        let resp = self.post_xml(&xml)?;
        Ok(parse::parse_simple_response_public(&resp))
    }

    pub fn create_ledger_debug(&self, ledger: &Ledger) -> Result<serde_json::Value> {
        ledger.validate()?;
        let map = ledger.to_map();
        let xml = self.prepare_request_xml(&XmlBuilder::create_ledger_request(&map)?)?;
        println!("\n================ XML Request ================\nPOST {}\nContent-Type: text/xml\n\n{}\n============================================\n", self.base_url, xml);
        let resp = self.post_raw_xml(&xml)?;
        println!("\n================ Raw Response ================\n{}\n============================================\n", resp);
        Ok(parse::parse_simple_response_public(&resp))
    }

    pub fn create_group(&self, group: &Group) -> Result<serde_json::Value> {
        group.validate()?;
        let map = group.to_map();
        let xml = XmlBuilder::create_group_request(&map)?;
        let resp = self.post_xml(&xml)?;
        Ok(parse::parse_simple_response_public(&resp))
    }

    pub fn create_group_debug(&self, group: &Group) -> Result<serde_json::Value> {
        group.validate()?;
        let map = group.to_map();
        let xml = self.prepare_request_xml(&XmlBuilder::create_group_request(&map)?)?;
        println!("\n================ XML Request ================\nPOST {}\nContent-Type: text/xml\n\n{}\n============================================\n", self.base_url, xml);
        let resp = self.post_raw_xml(&xml)?;
        println!("\n================ Raw Response ================\n{}\n============================================\n", resp);
        Ok(parse::parse_simple_response_public(&resp))
    }

    pub fn create_stock_item(&self, item: &StockItem) -> Result<serde_json::Value> {
        item.validate()?;
        let map = item.to_map();
        let xml = XmlBuilder::create_stock_item_request(&map)?;
        let resp = self.post_xml(&xml)?;
        Ok(parse::parse_simple_response_public(&resp))
    }

    pub fn create_stock_item_debug(&self, item: &StockItem) -> Result<serde_json::Value> {
        item.validate()?;
        let map = item.to_map();
        let xml = self.prepare_request_xml(&XmlBuilder::create_stock_item_request(&map)?)?;
        println!("\n================ XML Request ================\nPOST {}\nContent-Type: text/xml\n\n{}\n============================================\n", self.base_url, xml);
        let resp = self.post_raw_xml(&xml)?;
        println!("\n================ Raw Response ================\n{}\n============================================\n", resp);
        Ok(parse::parse_simple_response_public(&resp))
    }

    pub fn get_ledgers(&self) -> Result<Vec<(String, Option<String>)>> {
        let mut stat = serde_json::Map::new();
        let fetch = serde_json::json!({ "FETCH": ["NAME", "PARENT"] });
        stat.insert("FETCHLIST".into(), fetch);
        let xml = XmlBuilder::create_export_request("COLLECTION", "Ledger", Some(&stat))?;
        let resp = self.post_xml(&xml)?;
        Ok(extract_ledgers_from_xml(&resp))
    }

    pub fn get_groups(&self) -> Result<Vec<(String, Option<String>)>> {
        let mut stat = serde_json::Map::new();
        let fetch = serde_json::json!({ "FETCH": ["NAME", "PARENT"] });
        stat.insert("FETCHLIST".into(), fetch);
        let xml = XmlBuilder::create_export_request("COLLECTION", "Group", Some(&stat))?;
        let resp = self.post_xml(&xml)?;
        Ok(extract_groups_from_xml(&resp))
    }

    pub fn get_stock_items(&self) -> Result<Vec<(String, Option<String>)>> {
        let mut stat = serde_json::Map::new();
        let fetch = serde_json::json!({ "FETCH": ["NAME", "PARENT"] });
        stat.insert("FETCHLIST".into(), fetch);
        let xml = XmlBuilder::create_export_request("COLLECTION", "Stock Item", Some(&stat))?;
        let resp = self.post_xml(&xml)?;
        let rows = extract_stock_items_from_xml(&resp);
        Ok(rows)
    }

    pub fn active_company_name(&self) -> Result<Option<String>> {
        self.current_company_name()
    }

    fn prepare_request_xml(&self, xml: &str) -> Result<String> {
        if xml.contains("<SVCURRENTCOMPANY>") {
            return Ok(xml.to_string());
        }

        if !xml.contains("<TALLYREQUEST>Import Data</TALLYREQUEST>") {
            return Ok(xml.to_string());
        }

        let company = self.current_company_name()?.ok_or_else(|| {
            TallyError::Validation(
                "No active Tally company is available. Load a company in Tally or set `current_company`/`TALLY_COMPANY`."
                    .into(),
            )
        })?;

        if xml.contains("<REQUESTDESC>") && xml.contains("</REQUESTDESC>") {
            let static_vars = format!(
                "<STATICVARIABLES><SVCURRENTCOMPANY>{}</SVCURRENTCOMPANY></STATICVARIABLES>",
                XmlBuilder::escape_simple(&company)
            );
            return Ok(xml.replacen("</REQUESTDESC>", &(static_vars + "</REQUESTDESC>"), 1));
        }

        Ok(xml.to_string())
    }

    fn current_company_name(&self) -> Result<Option<String>> {
        if let Ok(cache) = self.current_company.lock() {
            if let Some(company) = cache.clone() {
                return Ok(Some(company));
            }
        }

        let company = self.discover_current_company()?;
        if let Some(ref discovered) = company {
            if let Ok(mut cache) = self.current_company.lock() {
                *cache = Some(discovered.clone());
            }
        }
        Ok(company)
    }

    fn discover_current_company(&self) -> Result<Option<String>> {
        let header = serde_json::json!({
            "VERSION": "1",
            "TALLYREQUEST": "EXPORT",
            "TYPE": "COLLECTION",
            "ID": "Company",
        })
        .as_object()
        .unwrap()
        .clone();
        let mut body = serde_json::Map::new();
        let mut desc = serde_json::Map::new();
        let mut stat = serde_json::Map::new();
        stat.insert(
            "SVEXPORTFORMAT".into(),
            serde_json::Value::String("$$SysName:XML".into()),
        );
        desc.insert("STATICVARIABLES".into(), serde_json::Value::Object(stat));
        body.insert("DESC".into(), serde_json::Value::Object(desc));
        let xml = XmlBuilder::create_envelope(&header, Some(&body))?;
        let resp = self.post_raw_xml(&xml)?;

        let company_attr = Regex::new(r#"<COMPANY[^>]*\bNAME="([^"]+)""#)
            .ok()
            .and_then(|re| re.captures(&resp))
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().trim().to_string())
            .filter(|name| !name.is_empty());
        if company_attr.is_some() {
            return Ok(company_attr);
        }

        let company_tag = Regex::new(r"(?s)<COMPANY(?:\s[^>]*)?>.*?<NAME>(.*?)</NAME>")
            .ok()
            .and_then(|re| re.captures(&resp))
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().trim().to_string())
            .filter(|name| !name.is_empty());
        Ok(company_tag)
    }
}

pub use parse::parse_simple_response_public as parse_simple_response;

impl TallyClient {
    /// Fetch vouchers from Tally server
    ///
    /// # Arguments
    /// * `from_date` - Optional start date in YYYYMMDD format (e.g., "20250101")
    /// * `to_date` - Optional end date in YYYYMMDD format (e.g., "20251231")
    ///
    /// If no dates are provided, fetches all vouchers (uses "19000101" to current date)
    pub fn get_vouchers(
        &self,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<Vec<Voucher>> {
        let current_company = self.current_company_name()?;
        let xml = XmlBuilder::create_voucher_export_request(
            from_date,
            to_date,
            current_company.as_deref(),
        )?;
        let resp = self.post_xml(&xml)?;
        Ok(voucher_parser::parse_vouchers_from_xml(&resp))
    }

    /// Fetch vouchers and enforce the date window client-side.
    ///
    /// Tally's Day Book export does not always honor date filters consistently
    /// across environments, so this method applies an exact YYYYMMDD filter on
    /// the parsed vouchers before returning them.
    pub fn get_vouchers_in_range(&self, from_date: &str, to_date: &str) -> Result<Vec<Voucher>> {
        let vouchers = self.get_vouchers(Some(from_date), Some(to_date))?;
        Ok(vouchers
            .into_iter()
            .filter(|voucher| is_yyyymmdd_in_range(&voucher.date_yyyymmdd, from_date, to_date))
            .collect())
    }
}

fn is_yyyymmdd_in_range(date: &str, from_date: &str, to_date: &str) -> bool {
    if date.len() != 8 || from_date.len() != 8 || to_date.len() != 8 {
        return false;
    }
    date >= from_date && date <= to_date
}
