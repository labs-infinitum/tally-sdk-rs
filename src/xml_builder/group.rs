use crate::errors::{Result, TallyError};
use serde_json::Value;

use super::XmlBuilder;

impl XmlBuilder {
    pub fn create_group_request(group_map: &serde_json::Map<String, Value>) -> Result<String> {
        let name = group_map.get("NAME").and_then(|v| v.as_str()).ok_or_else(|| TallyError::Validation("Group NAME is required".into()))?;
        let mut s = String::new();
        s.push_str(&format!("<ENVELOPE>\n<HEADER>\n<TALLYREQUEST>Import Data</TALLYREQUEST>\n</HEADER>\n<BODY>\n<IMPORTDATA>\n<REQUESTDESC>\n<REPORTNAME>All Masters</REPORTNAME>\n</REQUESTDESC>\n<REQUESTDATA>\n<TALLYMESSAGE xmlns:UDF=\"TallyUDF\">\n<GROUP NAME=\"{}\" RESERVEDNAME=\"\">\n", XmlBuilder::escape_simple(name)));
        if let Some(parent) = group_map.get("PARENT").and_then(|v| v.as_str()) { s.push_str(&format!("<PARENT>{}</PARENT>\n", XmlBuilder::escape_simple(parent))); } else { s.push_str("<PARENT/>\n"); }
        for k in ["ISADDABLE","BASICGROUPISCALCULABLE","ASORIGINAL","ISSUBLEDGER","ADDLALLOCTYPE","AFFECTSGROSSPROFIT"] { XmlBuilder::append_simple_if(group_map, k, &mut s); }

        if let Some(Value::String(gt)) = group_map.get("GROUP_TYPE") {
            match gt.as_str() {
                "Assets" => { s.push_str("<ISREVENUE>No</ISREVENUE>\n<AFFECTSGROSSPROFIT>No</AFFECTSGROSSPROFIT>\n<ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE>\n<AFFECTSSTOCK>No</AFFECTSSTOCK>\n"); }
                "Liabilities" => { s.push_str("<ISREVENUE>No</ISREVENUE>\n<AFFECTSGROSSPROFIT>No</AFFECTSGROSSPROFIT>\n<ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE>\n<AFFECTSSTOCK>No</AFFECTSSTOCK>\n"); }
                "Income" => { s.push_str("<ISREVENUE>Yes</ISREVENUE>\n<AFFECTSGROSSPROFIT>Yes</AFFECTSGROSSPROFIT>\n<ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE>\n<AFFECTSSTOCK>No</AFFECTSSTOCK>\n"); }
                "Expenses" => { s.push_str("<ISREVENUE>Yes</ISREVENUE>\n<AFFECTSGROSSPROFIT>No</AFFECTSGROSSPROFIT>\n<ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE>\n<AFFECTSSTOCK>No</AFFECTSSTOCK>\n"); }
                _ => {}
            }
        }

        if let Some(v) = group_map.get("HSNDETAILS.LIST").and_then(|v| v.as_object()) {
            s.push_str("<HSNDETAILS.LIST>\n");
            for k in ["APPLICABLEFROM","SRCOFHSNDETAILS","HSNCODE","HSN","HSNCLASSIFICATIONNAME"] { XmlBuilder::append_simple_if(v, k, &mut s); }
            s.push_str("</HSNDETAILS.LIST>\n");
        }

        if let Some(v) = group_map.get("GSTDETAILS.LIST").and_then(|v| v.as_object()) {
            s.push_str("<GSTDETAILS.LIST>\n");
            for k in ["APPLICABLEFROM","HSNMASTERNAME","TAXABILITY","SRCOFGSTDETAILS"] { XmlBuilder::append_simple_if(v, k, &mut s); }
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
        if let Some(Value::Array(alias_arr)) = group_map.get("ALIAS") { for alias in alias_arr { if let Value::String(a) = alias { s.push_str(&format!("<NAME>{}</NAME>\n", XmlBuilder::escape_simple(a))); } } }
        s.push_str("</NAME.LIST>\n<LANGUAGEID> 1033</LANGUAGEID>\n</LANGUAGENAME.LIST>\n");

        XmlBuilder::append_simple_if(group_map, "TDSAPPLICABLE", &mut s);
        if let Some(v) = group_map.get("TDSCATEGORYDETAILS.LIST").and_then(|v| v.as_object()) {
            s.push_str("<TDSCATEGORYDETAILS.LIST>\n");
            XmlBuilder::append_simple_if(v, "CATEGORYDATE", &mut s);
            XmlBuilder::append_simple_if(v, "CATEGORYNAME", &mut s);
            s.push_str("</TDSCATEGORYDETAILS.LIST>\n");
        }

        s.push_str("</GROUP>\n</TALLYMESSAGE>\n</REQUESTDATA>\n</IMPORTDATA>\n</BODY>\n</ENVELOPE>");
        Ok(s)
    }
}
