use crate::errors::Result;
use serde_json::Value;

use super::XmlBuilder;

impl XmlBuilder {
    pub fn create_stock_item_request(item_map: &serde_json::Map<String, Value>) -> Result<String> {
        let name = item_map.get("NAME").and_then(|v| v.as_str()).unwrap_or("");
        let mut s = String::new();
        s.push_str(&format!("<ENVELOPE>\n<HEADER>\n<TALLYREQUEST>Import Data</TALLYREQUEST>\n</HEADER>\n<BODY>\n<IMPORTDATA>\n<REQUESTDESC>\n<REPORTNAME>All Masters</REPORTNAME>\n</REQUESTDESC>\n<REQUESTDATA>\n<TALLYMESSAGE xmlns:UDF=\"TallyUDF\">\n<STOCKITEM NAME=\"{}\" RESERVEDNAME=\"\">\n", XmlBuilder::escape_simple(name)));
        if let Some(parent) = item_map.get("PARENT").and_then(|v| v.as_str()) { s.push_str(&format!("<PARENT>{}</PARENT>\n", XmlBuilder::escape_simple(parent))); } else { s.push_str("<PARENT/>\n"); }
        for k in ["BASEUNITS","ADDITIONALUNITS","GSTAPPLICABLE","GSTTYPEOFSUPPLY","BASICRATEOFEXCISE","OPENINGBALANCE"] { XmlBuilder::append_simple_if(item_map, k, &mut s); }
        if let Some(v) = item_map.get("HSNDETAILS.LIST").and_then(|v| v.as_object()) {
            s.push_str("<HSNDETAILS.LIST>\n");
            for k in ["APPLICABLEFROM","HSNCODE","HSN","SRCOFHSNDETAILS","HSNCLASSIFICATIONNAME"] { XmlBuilder::append_simple_if(v, k, &mut s); }
            s.push_str("</HSNDETAILS.LIST>\n");
        }
        if let Some(v) = item_map.get("GSTDETAILS.LIST").and_then(|v| v.as_object()) {
            s.push_str("<GSTDETAILS.LIST>\n");
            for k in ["APPLICABLEFROM","TAXABILITY","SRCOFGSTDETAILS","HSNMASTERNAME"] { XmlBuilder::append_simple_if(v, k, &mut s); }
            if let Some(state) = v.get("STATEWISEDETAILS.LIST").and_then(|x| x.as_object()) {
                s.push_str("<STATEWISEDETAILS.LIST>\n");
                if let Some(name) = state.get("STATENAME") { s.push_str(&format!("<STATENAME>{}</STATENAME>\n", XmlBuilder::escape_text(name))); } else { s.push_str("<STATENAME>&#4; Any</STATENAME>\n"); }
                if let Some(rate) = state.get("RATEDETAILS.LIST").and_then(|x| x.as_object()) {
                    s.push_str("<RATEDETAILS.LIST>\n");
                    for k in ["GSTRATEDUTYHEAD","GSTRATEVALUATIONTYPE","GSTRATE"] { XmlBuilder::append_simple_if(rate, k, &mut s); }
                    s.push_str("</RATEDETAILS.LIST>\n");
                }
                s.push_str("</STATEWISEDETAILS.LIST>\n");
            }
            s.push_str("</GSTDETAILS.LIST>\n");
        }
        s.push_str("<LANGUAGENAME.LIST>\n<NAME.LIST TYPE=\"String\">\n");
        s.push_str(&format!("<NAME>{}</NAME>\n", XmlBuilder::escape_simple(name)));
        if let Some(Value::Array(alias_arr)) = item_map.get("ALIAS") { for alias in alias_arr { if let Value::String(a) = alias { s.push_str(&format!("<NAME>{}</NAME>\n", XmlBuilder::escape_simple(a))); } } }
        s.push_str("</NAME.LIST>\n<LANGUAGEID> 1033</LANGUAGEID>\n</LANGUAGENAME.LIST>\n");
        s.push_str("</STOCKITEM>\n</TALLYMESSAGE>\n</REQUESTDATA>\n</IMPORTDATA>\n</BODY>\n</ENVELOPE>");
        Ok(s)
    }
}


