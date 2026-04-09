use crate::errors::{Result, TallyError};
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use serde_json::Value;
use std::io::Cursor;

use super::XmlBuilder;

impl XmlBuilder {
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
