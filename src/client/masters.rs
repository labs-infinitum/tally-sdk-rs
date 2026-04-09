use super::extract::{
    extract_groups_from_xml, extract_ledgers_from_xml, extract_stock_items_from_xml,
};
use super::{parse, TallyClient};
use crate::errors::{Result, TallyError};
use crate::models::{Group, Ledger, StockItem};
use crate::xml_builder::XmlBuilder;
use regex::Regex;

impl TallyClient {
    pub fn create_ledger(&self, ledger: &Ledger) -> Result<serde_json::Value> {
        ledger.validate()?;
        let map = ledger.to_map();
        let xml = XmlBuilder::create_ledger_request(&map)?;
        self.execute_create_request(&xml)
    }

    pub fn create_ledger_debug(&self, ledger: &Ledger) -> Result<serde_json::Value> {
        ledger.validate()?;
        let map = ledger.to_map();
        let xml = XmlBuilder::create_ledger_request(&map)?;
        self.execute_debug_create_request(&xml)
    }

    pub fn create_group(&self, group: &Group) -> Result<serde_json::Value> {
        group.validate()?;
        let map = group.to_map();
        let xml = XmlBuilder::create_group_request(&map)?;
        self.execute_create_request(&xml)
    }

    pub fn create_group_debug(&self, group: &Group) -> Result<serde_json::Value> {
        group.validate()?;
        let map = group.to_map();
        let xml = XmlBuilder::create_group_request(&map)?;
        self.execute_debug_create_request(&xml)
    }

    pub fn create_stock_item(&self, item: &StockItem) -> Result<serde_json::Value> {
        item.validate()?;
        let map = item.to_map();
        let xml = XmlBuilder::create_stock_item_request(&map)?;
        self.execute_create_request(&xml)
    }

    pub fn create_stock_item_debug(&self, item: &StockItem) -> Result<serde_json::Value> {
        item.validate()?;
        let map = item.to_map();
        let xml = XmlBuilder::create_stock_item_request(&map)?;
        self.execute_debug_create_request(&xml)
    }

    pub fn get_ledgers(&self) -> Result<Vec<(String, Option<String>)>> {
        self.fetch_name_parent_collection("Ledger", extract_ledgers_from_xml)
    }

    pub fn get_groups(&self) -> Result<Vec<(String, Option<String>)>> {
        self.fetch_name_parent_collection("Group", extract_groups_from_xml)
    }

    pub fn get_stock_items(&self) -> Result<Vec<(String, Option<String>)>> {
        self.fetch_name_parent_collection("Stock Item", extract_stock_items_from_xml)
    }

    pub fn get_currencies(&self) -> Result<Vec<String>> {
        let current_company = self.current_company_name()?;
        let xml = XmlBuilder::create_currency_export_request(current_company.as_deref())?;
        let resp = self.post_xml(&xml)?;

        let currency_re = Regex::new(r#"<CURRENCY\b[^>]*\bNAME="([^"]+)""#)
            .map_err(|e| TallyError::Unexpected(e.to_string()))?;

        let mut currencies = Vec::new();
        for caps in currency_re.captures_iter(&resp) {
            if let Some(name) = caps.get(1).map(|m| m.as_str().trim().to_string()) {
                if !name.is_empty() && !currencies.contains(&name) {
                    currencies.push(name);
                }
            }
        }

        Ok(currencies)
    }

    fn execute_create_request(&self, xml: &str) -> Result<serde_json::Value> {
        let resp = self.post_xml(xml)?;
        Ok(parse::parse_simple_response_public(&resp))
    }

    fn execute_debug_create_request(&self, xml: &str) -> Result<serde_json::Value> {
        let prepared = self.prepare_request_xml(xml)?;
        println!(
            "\n================ XML Request ================\nPOST {}\nContent-Type: text/xml\n\n{}\n============================================\n",
            self.base_url, prepared
        );
        let resp = self.post_raw_xml(&prepared)?;
        println!(
            "\n================ Raw Response ================\n{}\n============================================\n",
            resp
        );
        Ok(parse::parse_simple_response_public(&resp))
    }

    fn fetch_name_parent_collection(
        &self,
        collection_type: &str,
        extractor: fn(&str) -> Vec<(String, Option<String>)>,
    ) -> Result<Vec<(String, Option<String>)>> {
        let mut stat = serde_json::Map::new();
        stat.insert(
            "FETCHLIST".into(),
            serde_json::json!({ "FETCH": ["NAME", "PARENT"] }),
        );
        let xml = XmlBuilder::create_export_request("COLLECTION", collection_type, Some(&stat))?;
        let resp = self.post_xml(&xml)?;
        Ok(extractor(&resp))
    }
}
