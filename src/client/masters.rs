use super::extract::{
    extract_currencies_from_xml, extract_groups_from_xml, extract_ledgers_from_xml,
    extract_stock_items_from_xml,
};
use super::{parse, TallyClient};
use crate::errors::Result;
use crate::models::{
    CurrencySummary, Group, GroupSummary, ImportResult, Ledger, LedgerSummary, StockItem,
    StockItemSummary,
};
use crate::xml_builder::XmlBuilder;

impl TallyClient {
    pub fn create_ledger(&self, ledger: &Ledger) -> Result<ImportResult> {
        ledger.validate()?;
        let map = ledger.to_map();
        let xml = XmlBuilder::create_ledger_request(&map)?;
        self.execute_create_request(&xml)
    }

    pub fn create_ledger_debug(&self, ledger: &Ledger) -> Result<ImportResult> {
        ledger.validate()?;
        let map = ledger.to_map();
        let xml = XmlBuilder::create_ledger_request(&map)?;
        self.execute_debug_create_request(&xml)
    }

    pub fn create_group(&self, group: &Group) -> Result<ImportResult> {
        group.validate()?;
        let map = group.to_map();
        let xml = XmlBuilder::create_group_request(&map)?;
        self.execute_create_request(&xml)
    }

    pub fn create_group_debug(&self, group: &Group) -> Result<ImportResult> {
        group.validate()?;
        let map = group.to_map();
        let xml = XmlBuilder::create_group_request(&map)?;
        self.execute_debug_create_request(&xml)
    }

    pub fn create_stock_item(&self, item: &StockItem) -> Result<ImportResult> {
        item.validate()?;
        let map = item.to_map();
        let xml = XmlBuilder::create_stock_item_request(&map)?;
        self.execute_create_request(&xml)
    }

    pub fn create_stock_item_debug(&self, item: &StockItem) -> Result<ImportResult> {
        item.validate()?;
        let map = item.to_map();
        let xml = XmlBuilder::create_stock_item_request(&map)?;
        self.execute_debug_create_request(&xml)
    }

    pub fn get_ledgers(&self) -> Result<Vec<LedgerSummary>> {
        self.fetch_name_parent_collection("Ledger", extract_ledgers_from_xml)
    }

    pub fn get_groups(&self) -> Result<Vec<GroupSummary>> {
        self.fetch_name_parent_collection("Group", extract_groups_from_xml)
    }

    pub fn get_stock_items(&self) -> Result<Vec<StockItemSummary>> {
        self.fetch_name_parent_collection("Stock Item", extract_stock_items_from_xml)
    }

    pub fn get_currencies(&self) -> Result<Vec<CurrencySummary>> {
        let current_company = self.current_company_name()?;
        let xml = XmlBuilder::create_currency_export_request(current_company.as_deref())?;
        let resp = self.post_xml(&xml)?;
        Ok(extract_currencies_from_xml(&resp))
    }

    fn execute_create_request(&self, xml: &str) -> Result<ImportResult> {
        let resp = self.post_xml(xml)?;
        Ok(parse::parse_simple_response_public(&resp))
    }

    fn execute_debug_create_request(&self, xml: &str) -> Result<ImportResult> {
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

    fn fetch_name_parent_collection<T>(
        &self,
        collection_type: &str,
        extractor: fn(&str) -> Vec<T>,
    ) -> Result<Vec<T>> {
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
