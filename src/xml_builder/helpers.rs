use crate::errors::{Result, TallyError};
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use serde_json::Value;
use std::io::Cursor;

use super::XmlBuilder;

impl XmlBuilder {
    pub(crate) fn append_all_masters_import_start(s: &mut String) {
        s.push_str(
            "<ENVELOPE>\n<HEADER>\n<TALLYREQUEST>Import Data</TALLYREQUEST>\n</HEADER>\n<BODY>\n<IMPORTDATA>\n<REQUESTDESC>\n<REPORTNAME>All Masters</REPORTNAME>\n</REQUESTDESC>\n<REQUESTDATA>\n<TALLYMESSAGE xmlns:UDF=\"TallyUDF\">\n",
        );
    }

    pub(crate) fn append_import_end(s: &mut String) {
        s.push_str("</TALLYMESSAGE>\n</REQUESTDATA>\n</IMPORTDATA>\n</BODY>\n</ENVELOPE>");
    }

    pub(crate) fn append_parent_tag(s: &mut String, parent: Option<&str>, allow_empty: bool) {
        if let Some(parent) = parent {
            s.push_str(&format!(
                "<PARENT>{}</PARENT>\n",
                XmlBuilder::escape_simple(parent)
            ));
        } else if allow_empty {
            s.push_str("<PARENT/>\n");
        }
    }

    pub(crate) fn append_language_name_list(
        s: &mut String,
        name: &str,
        aliases: Option<&Value>,
        escape_values: bool,
        spaced_language_id: bool,
    ) {
        s.push_str("<LANGUAGENAME.LIST>\n<NAME.LIST TYPE=\"String\">\n");
        XmlBuilder::append_name_entry(s, name, escape_values);
        if let Some(Value::Array(alias_arr)) = aliases {
            for alias in alias_arr {
                if let Value::String(alias) = alias {
                    XmlBuilder::append_name_entry(s, alias, escape_values);
                }
            }
        }
        let language_id = if spaced_language_id { " 1033" } else { "1033" };
        s.push_str(&format!(
            "</NAME.LIST>\n<LANGUAGEID>{language_id}</LANGUAGEID>\n</LANGUAGENAME.LIST>\n"
        ));
    }

    pub(crate) fn append_hsn_details_block(
        s: &mut String,
        obj: Option<&serde_json::Map<String, Value>>,
        keys: &[&str],
    ) {
        if let Some(obj) = obj {
            s.push_str("<HSNDETAILS.LIST>\n");
            for key in keys {
                XmlBuilder::append_simple_if(obj, key, s);
            }
            s.push_str("</HSNDETAILS.LIST>\n");
        }
    }

    pub(crate) fn append_gst_details_block(
        s: &mut String,
        obj: Option<&serde_json::Map<String, Value>>,
        keys: &[&str],
        state_fallback_any: bool,
        preserve_numeric_entity_state: bool,
    ) {
        if let Some(obj) = obj {
            s.push_str("<GSTDETAILS.LIST>\n");
            for key in keys {
                XmlBuilder::append_simple_if(obj, key, s);
            }
            if let Some(state) = obj.get("STATEWISEDETAILS.LIST").and_then(|x| x.as_object()) {
                XmlBuilder::append_statewise_details_block(
                    s,
                    state,
                    state_fallback_any,
                    preserve_numeric_entity_state,
                );
            }
            s.push_str("</GSTDETAILS.LIST>\n");
        }
    }

    pub(crate) fn append_tds_category_details_block(
        s: &mut String,
        obj: Option<&serde_json::Map<String, Value>>,
    ) {
        if let Some(obj) = obj {
            s.push_str("<TDSCATEGORYDETAILS.LIST>\n");
            XmlBuilder::append_simple_if(obj, "CATEGORYDATE", s);
            XmlBuilder::append_simple_if(obj, "CATEGORYNAME", s);
            s.push_str("</TDSCATEGORYDETAILS.LIST>\n");
        }
    }

    pub(crate) fn append_statewise_details_block(
        s: &mut String,
        obj: &serde_json::Map<String, Value>,
        state_fallback_any: bool,
        preserve_numeric_entity_state: bool,
    ) {
        s.push_str("<STATEWISEDETAILS.LIST>\n");
        if let Some(name) = obj.get("STATENAME") {
            if preserve_numeric_entity_state
                && name.is_string()
                && name.as_str().unwrap_or("").starts_with("&#")
            {
                if let Some(raw) = name.as_str() {
                    s.push_str(&format!("<STATENAME>{raw}</STATENAME>\n"));
                }
            } else {
                s.push_str(&format!(
                    "<STATENAME>{}</STATENAME>\n",
                    XmlBuilder::escape_text(name)
                ));
            }
        } else if state_fallback_any {
            s.push_str("<STATENAME>&#4; Any</STATENAME>\n");
        }
        if let Some(rate) = obj.get("RATEDETAILS.LIST").and_then(|x| x.as_object()) {
            XmlBuilder::append_rate_details_block(s, rate);
        }
        s.push_str("</STATEWISEDETAILS.LIST>\n");
    }

    pub(crate) fn append_rate_details_block(s: &mut String, obj: &serde_json::Map<String, Value>) {
        s.push_str("<RATEDETAILS.LIST>\n");
        for key in ["GSTRATEDUTYHEAD", "GSTRATEVALUATIONTYPE", "GSTRATE"] {
            XmlBuilder::append_simple_if(obj, key, s);
        }
        s.push_str("</RATEDETAILS.LIST>\n");
    }

    pub(crate) fn write_simple(
        writer: &mut Writer<Cursor<Vec<u8>>>,
        key: &str,
        value: &Value,
    ) -> Result<()> {
        if value.is_null() {
            return Ok(());
        }
        let text = XmlBuilder::escape_text(value);
        XmlBuilder::write_text_node(writer, key, &text)
    }

    pub(crate) fn write_text_node(
        writer: &mut Writer<Cursor<Vec<u8>>>,
        key: &str,
        text: &str,
    ) -> Result<()> {
        writer
            .write_event(Event::Start(BytesStart::new(key)))
            .map_err(|e| TallyError::Xml(e.to_string()))?;
        writer
            .write_event(Event::Text(BytesText::new(text)))
            .map_err(|e| TallyError::Xml(e.to_string()))?;
        writer
            .write_event(Event::End(BytesEnd::new(key)))
            .map_err(|e| TallyError::Xml(e.to_string()))?;
        Ok(())
    }

    pub(crate) fn write_kv_recursive(
        writer: &mut Writer<Cursor<Vec<u8>>>,
        key: &str,
        value: &Value,
    ) -> Result<()> {
        match value {
            Value::Object(obj) => {
                writer
                    .write_event(Event::Start(BytesStart::new(key)))
                    .map_err(|e| TallyError::Xml(e.to_string()))?;
                for (k, v) in obj {
                    XmlBuilder::write_kv_recursive(writer, k, v)?;
                }
                writer
                    .write_event(Event::End(BytesEnd::new(key)))
                    .map_err(|e| TallyError::Xml(e.to_string()))?;
            }
            Value::Array(arr) => {
                for item in arr {
                    XmlBuilder::write_kv_recursive(writer, key, item)?;
                }
            }
            _ => {
                XmlBuilder::write_simple(writer, key, value)?;
            }
        }
        Ok(())
    }

    pub(crate) fn write_value_recursive(
        writer: &mut Writer<Cursor<Vec<u8>>>,
        value: &Value,
    ) -> Result<()> {
        match value {
            Value::Object(obj) => {
                for (k, v) in obj {
                    XmlBuilder::write_kv_recursive(writer, k, v)?;
                }
            }
            Value::Array(arr) => {
                for item in arr {
                    XmlBuilder::write_value_recursive(writer, item)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub(crate) fn append_simple_if(
        obj: &serde_json::Map<String, Value>,
        key: &str,
        s: &mut String,
    ) {
        if let Some(v) = obj.get(key) {
            if !v.is_null() {
                s.push_str(&format!(
                    "<{0}>{1}</{0}>\n",
                    key,
                    XmlBuilder::escape_text(v)
                ));
            }
        }
    }

    pub(crate) fn escape_text(v: &Value) -> String {
        XmlBuilder::escape_simple_allow_numeric_entities(&value_to_string(v))
    }

    pub(crate) fn escape_simple(s: &str) -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
    }

    pub(crate) fn escape_simple_allow_numeric_entities(s: &str) -> String {
        if s.starts_with("&#") {
            s.to_string()
        } else {
            XmlBuilder::escape_simple(s)
        }
    }

    fn append_name_entry(s: &mut String, value: &str, escape_value: bool) {
        let text = if escape_value {
            XmlBuilder::escape_simple(value)
        } else {
            value.to_string()
        };
        s.push_str(&format!("<NAME>{text}</NAME>\n"));
    }
}

fn value_to_string(v: &Value) -> String {
    match v {
        Value::Null => String::new(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(_) | Value::Object(_) => serde_json::to_string(v).unwrap_or_default(),
    }
}
