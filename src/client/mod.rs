use crate::config::TallyConfig;
use crate::errors::{Result, TallyError};
use crate::models::{Group, Ledger, StockItem};
use crate::xml_builder::XmlBuilder;

mod http;
pub mod parse;
mod extract;

use crate::client::extract::{extract_groups_from_xml, extract_ledgers_from_xml, extract_stock_items_from_xml, extract_vouchers_from_xml};

pub struct TallyClient {
    cfg: TallyConfig,
    http: reqwest::blocking::Client,
    base_url: String,
}

impl TallyClient {
    pub fn new(cfg: TallyConfig) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static("text/xml"));
        headers.insert(reqwest::header::CACHE_CONTROL, reqwest::header::HeaderValue::from_static("no-cache"));
        if let (Some(acc), Some(pw)) = (&cfg.tally_net_account, &cfg.tally_net_password) {
            headers.insert("X-Tally-Account", reqwest::header::HeaderValue::from_str(acc).unwrap_or(reqwest::header::HeaderValue::from_static("")));
            headers.insert("X-Tally-Password", reqwest::header::HeaderValue::from_str(pw).unwrap_or(reqwest::header::HeaderValue::from_static("")));
        }

        let http = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(cfg.timeout_secs))
            .build()
            .map_err(|e| TallyError::Unexpected(e.to_string()))?;
        let base_url = format!("http://{}:{}", cfg.host, cfg.port);
        Ok(Self { cfg, http, base_url })
    }

    pub fn test_connection(&self) -> Result<bool> {
        let header = serde_json::json!({
            "VERSION": "1",
            "TALLYREQUEST": "EXPORT",
            "TYPE": "COLLECTION",
            "ID": "Company",
        }).as_object().unwrap().clone();
        let mut body = serde_json::Map::new();
        let mut desc = serde_json::Map::new();
        let mut stat = serde_json::Map::new();
        stat.insert("SVEXPORTFORMAT".into(), serde_json::Value::String("$$SysName:XML".into()));
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
        let xml = XmlBuilder::create_ledger_request(&map)?;
        println!("\n================ XML Request ================\nPOST {}\nContent-Type: text/xml\n\n{}\n============================================\n", self.base_url, xml);
        let resp = self.post_xml(&xml)?;
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
        let xml = XmlBuilder::create_group_request(&map)?;
        println!("\n================ XML Request ================\nPOST {}\nContent-Type: text/xml\n\n{}\n============================================\n", self.base_url, xml);
        let resp = self.post_xml(&xml)?;
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
        let xml = XmlBuilder::create_stock_item_request(&map)?;
        println!("\n================ XML Request ================\nPOST {}\nContent-Type: text/xml\n\n{}\n============================================\n", self.base_url, xml);
        let resp = self.post_xml(&xml)?;
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
}

pub use parse::parse_simple_response_public as parse_simple_response;

impl TallyClient {
    pub fn get_vouchers(&self) -> Result<Vec<(String, String)>> {
        // Day Book export; FETCHLIST not needed/used
        let xml = XmlBuilder::create_export_request("DATA", "Voucher", None)?;
        let resp = self.post_xml(&xml)?;
        Ok(extract_vouchers_from_xml(&resp))
    }
}
