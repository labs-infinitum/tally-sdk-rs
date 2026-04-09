use crate::errors::{Result, TallyError};
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use serde_json::Value;
use std::io::Cursor;

use super::XmlBuilder;

impl XmlBuilder {
    pub(crate) fn create_all_masters_import_request<F>(build: F) -> Result<String>
    where
        F: FnOnce(&mut Writer<Cursor<Vec<u8>>>) -> Result<()>,
    {
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        XmlBuilder::write_start_tag(&mut writer, "ENVELOPE")?;
        XmlBuilder::write_start_tag(&mut writer, "HEADER")?;
        XmlBuilder::write_text_node(&mut writer, "TALLYREQUEST", "Import Data")?;
        XmlBuilder::write_end_tag(&mut writer, "HEADER")?;
        XmlBuilder::write_start_tag(&mut writer, "BODY")?;
        XmlBuilder::write_start_tag(&mut writer, "IMPORTDATA")?;
        XmlBuilder::write_start_tag(&mut writer, "REQUESTDESC")?;
        XmlBuilder::write_text_node(&mut writer, "REPORTNAME", "All Masters")?;
        XmlBuilder::write_end_tag(&mut writer, "REQUESTDESC")?;
        XmlBuilder::write_start_tag(&mut writer, "REQUESTDATA")?;
        XmlBuilder::write_start_tag_with_attrs(
            &mut writer,
            "TALLYMESSAGE",
            &[("xmlns:UDF", "TallyUDF")],
        )?;

        build(&mut writer)?;

        XmlBuilder::write_end_tag(&mut writer, "TALLYMESSAGE")?;
        XmlBuilder::write_end_tag(&mut writer, "REQUESTDATA")?;
        XmlBuilder::write_end_tag(&mut writer, "IMPORTDATA")?;
        XmlBuilder::write_end_tag(&mut writer, "BODY")?;
        XmlBuilder::write_end_tag(&mut writer, "ENVELOPE")?;

        XmlBuilder::finish_writer(writer)
    }

    pub(crate) fn write_parent_tag(
        writer: &mut Writer<Cursor<Vec<u8>>>,
        parent: Option<&str>,
        allow_empty: bool,
    ) -> Result<()> {
        if let Some(parent) = parent {
            XmlBuilder::write_text_node(writer, "PARENT", parent)?;
        } else if allow_empty {
            writer
                .write_event(Event::Empty(BytesStart::new("PARENT")))
                .map_err(|e| TallyError::Xml(e.to_string()))?;
        }
        Ok(())
    }

    pub(crate) fn write_language_name_list(
        writer: &mut Writer<Cursor<Vec<u8>>>,
        name: &str,
        aliases: Option<&Value>,
        escape_values: bool,
        spaced_language_id: bool,
    ) -> Result<()> {
        XmlBuilder::write_start_tag(writer, "LANGUAGENAME.LIST")?;
        XmlBuilder::write_start_tag_with_attrs(writer, "NAME.LIST", &[("TYPE", "String")])?;
        XmlBuilder::write_name_entry(writer, name, escape_values)?;
        if let Some(Value::Array(alias_arr)) = aliases {
            for alias in alias_arr {
                if let Value::String(alias) = alias {
                    XmlBuilder::write_name_entry(writer, alias, escape_values)?;
                }
            }
        }
        let language_id = if spaced_language_id { " 1033" } else { "1033" };
        XmlBuilder::write_end_tag(writer, "NAME.LIST")?;
        XmlBuilder::write_text_node(writer, "LANGUAGEID", language_id)?;
        XmlBuilder::write_end_tag(writer, "LANGUAGENAME.LIST")?;
        Ok(())
    }

    pub(crate) fn write_hsn_details_block(
        writer: &mut Writer<Cursor<Vec<u8>>>,
        obj: Option<&serde_json::Map<String, Value>>,
        keys: &[&str],
    ) -> Result<()> {
        if let Some(obj) = obj {
            XmlBuilder::write_start_tag(writer, "HSNDETAILS.LIST")?;
            for key in keys {
                XmlBuilder::write_simple_if(writer, obj, key)?;
            }
            XmlBuilder::write_end_tag(writer, "HSNDETAILS.LIST")?;
        }
        Ok(())
    }

    pub(crate) fn write_gst_details_block(
        writer: &mut Writer<Cursor<Vec<u8>>>,
        obj: Option<&serde_json::Map<String, Value>>,
        keys: &[&str],
        state_fallback_any: bool,
        preserve_numeric_entity_state: bool,
    ) -> Result<()> {
        if let Some(obj) = obj {
            XmlBuilder::write_start_tag(writer, "GSTDETAILS.LIST")?;
            for key in keys {
                XmlBuilder::write_simple_if(writer, obj, key)?;
            }
            if let Some(state) = obj.get("STATEWISEDETAILS.LIST").and_then(|x| x.as_object()) {
                XmlBuilder::write_statewise_details_block(
                    writer,
                    state,
                    state_fallback_any,
                    preserve_numeric_entity_state,
                )?;
            }
            XmlBuilder::write_end_tag(writer, "GSTDETAILS.LIST")?;
        }
        Ok(())
    }

    pub(crate) fn write_tds_category_details_block(
        writer: &mut Writer<Cursor<Vec<u8>>>,
        obj: Option<&serde_json::Map<String, Value>>,
    ) -> Result<()> {
        if let Some(obj) = obj {
            XmlBuilder::write_start_tag(writer, "TDSCATEGORYDETAILS.LIST")?;
            XmlBuilder::write_simple_if(writer, obj, "CATEGORYDATE")?;
            XmlBuilder::write_simple_if(writer, obj, "CATEGORYNAME")?;
            XmlBuilder::write_end_tag(writer, "TDSCATEGORYDETAILS.LIST")?;
        }
        Ok(())
    }

    pub(crate) fn write_statewise_details_block(
        writer: &mut Writer<Cursor<Vec<u8>>>,
        obj: &serde_json::Map<String, Value>,
        state_fallback_any: bool,
        preserve_numeric_entity_state: bool,
    ) -> Result<()> {
        XmlBuilder::write_start_tag(writer, "STATEWISEDETAILS.LIST")?;
        if let Some(name) = obj.get("STATENAME") {
            if preserve_numeric_entity_state
                && name.is_string()
                && name.as_str().unwrap_or("").starts_with("&#")
            {
                if let Some(raw) = name.as_str() {
                    XmlBuilder::write_text_node_escaped(writer, "STATENAME", raw)?;
                }
            } else {
                XmlBuilder::write_simple(writer, "STATENAME", name)?;
            }
        } else if state_fallback_any {
            XmlBuilder::write_text_node_escaped(writer, "STATENAME", "&#4; Any")?;
        }
        if let Some(rate) = obj.get("RATEDETAILS.LIST").and_then(|x| x.as_object()) {
            XmlBuilder::write_rate_details_block(writer, rate)?;
        }
        XmlBuilder::write_end_tag(writer, "STATEWISEDETAILS.LIST")?;
        Ok(())
    }

    pub(crate) fn write_rate_details_block(
        writer: &mut Writer<Cursor<Vec<u8>>>,
        obj: &serde_json::Map<String, Value>,
    ) -> Result<()> {
        XmlBuilder::write_start_tag(writer, "RATEDETAILS.LIST")?;
        for key in ["GSTRATEDUTYHEAD", "GSTRATEVALUATIONTYPE", "GSTRATE"] {
            XmlBuilder::write_simple_if(writer, obj, key)?;
        }
        XmlBuilder::write_end_tag(writer, "RATEDETAILS.LIST")?;
        Ok(())
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
        XmlBuilder::write_text_node_escaped(writer, key, &text)
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

    pub(crate) fn write_text_node_with_attrs(
        writer: &mut Writer<Cursor<Vec<u8>>>,
        key: &str,
        text: &str,
        attrs: &[(&str, &str)],
    ) -> Result<()> {
        let mut start = BytesStart::new(key);
        for (attr_key, attr_value) in attrs {
            start.push_attribute((*attr_key, *attr_value));
        }
        writer
            .write_event(Event::Start(start))
            .map_err(|e| TallyError::Xml(e.to_string()))?;
        writer
            .write_event(Event::Text(BytesText::new(text)))
            .map_err(|e| TallyError::Xml(e.to_string()))?;
        writer
            .write_event(Event::End(BytesEnd::new(key)))
            .map_err(|e| TallyError::Xml(e.to_string()))?;
        Ok(())
    }

    pub(crate) fn write_text_node_escaped(
        writer: &mut Writer<Cursor<Vec<u8>>>,
        key: &str,
        text: &str,
    ) -> Result<()> {
        writer
            .write_event(Event::Start(BytesStart::new(key)))
            .map_err(|e| TallyError::Xml(e.to_string()))?;
        writer
            .write_event(Event::Text(BytesText::from_escaped(text)))
            .map_err(|e| TallyError::Xml(e.to_string()))?;
        writer
            .write_event(Event::End(BytesEnd::new(key)))
            .map_err(|e| TallyError::Xml(e.to_string()))?;
        Ok(())
    }

    pub(crate) fn write_simple_if(
        writer: &mut Writer<Cursor<Vec<u8>>>,
        obj: &serde_json::Map<String, Value>,
        key: &str,
    ) -> Result<()> {
        if let Some(v) = obj.get(key) {
            if !v.is_null() {
                XmlBuilder::write_simple(writer, key, v)?;
            }
        }
        Ok(())
    }

    pub(crate) fn write_start_tag(writer: &mut Writer<Cursor<Vec<u8>>>, tag: &str) -> Result<()> {
        writer
            .write_event(Event::Start(BytesStart::new(tag)))
            .map_err(|e| TallyError::Xml(e.to_string()))?;
        Ok(())
    }

    pub(crate) fn write_start_tag_with_attrs(
        writer: &mut Writer<Cursor<Vec<u8>>>,
        tag: &str,
        attrs: &[(&str, &str)],
    ) -> Result<()> {
        let mut start = BytesStart::new(tag);
        for (key, value) in attrs {
            start.push_attribute((*key, *value));
        }
        writer
            .write_event(Event::Start(start))
            .map_err(|e| TallyError::Xml(e.to_string()))?;
        Ok(())
    }

    pub(crate) fn write_end_tag(writer: &mut Writer<Cursor<Vec<u8>>>, tag: &str) -> Result<()> {
        writer
            .write_event(Event::End(BytesEnd::new(tag)))
            .map_err(|e| TallyError::Xml(e.to_string()))?;
        Ok(())
    }

    pub(crate) fn write_export_envelope_header(
        writer: &mut Writer<Cursor<Vec<u8>>>,
        tally_request: &str,
        request_type: &str,
        id: &str,
    ) -> Result<()> {
        XmlBuilder::write_start_tag(writer, "ENVELOPE")?;
        XmlBuilder::write_start_tag(writer, "HEADER")?;
        XmlBuilder::write_text_node(writer, "VERSION", "1")?;
        XmlBuilder::write_text_node(writer, "TALLYREQUEST", tally_request)?;
        XmlBuilder::write_text_node(writer, "TYPE", request_type)?;
        XmlBuilder::write_text_node(writer, "ID", id)?;
        XmlBuilder::write_end_tag(writer, "HEADER")?;
        Ok(())
    }

    pub(crate) fn write_current_company_tag(
        writer: &mut Writer<Cursor<Vec<u8>>>,
        current_company: Option<&str>,
    ) -> Result<()> {
        if let Some(company) = current_company.filter(|name| !name.trim().is_empty()) {
            XmlBuilder::write_text_node(writer, "SVCURRENTCOMPANY", company.trim())?;
        }
        Ok(())
    }

    pub(crate) fn finish_writer(writer: Writer<Cursor<Vec<u8>>>) -> Result<String> {
        let bytes = writer.into_inner().into_inner();
        String::from_utf8(bytes).map_err(|e| TallyError::Xml(e.to_string()))
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

    fn write_name_entry(
        writer: &mut Writer<Cursor<Vec<u8>>>,
        value: &str,
        escape_value: bool,
    ) -> Result<()> {
        let text = if escape_value {
            XmlBuilder::escape_simple(value)
        } else {
            value.to_string()
        };
        XmlBuilder::write_text_node_escaped(writer, "NAME", &text)
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
