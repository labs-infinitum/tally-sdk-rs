use crate::errors::{Result, TallyError};
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::Writer;
use serde_json::Value;
use std::io::Cursor;

use super::XmlBuilder;

impl XmlBuilder {
    pub fn create_envelope(header: &serde_json::Map<String, Value>, body: Option<&serde_json::Map<String, Value>>) -> Result<String> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        writer.write_event(Event::Start(BytesStart::new("ENVELOPE"))).map_err(|e| TallyError::Xml(e.to_string()))?;

        // HEADER
        writer.write_event(Event::Start(BytesStart::new("HEADER"))).map_err(|e| TallyError::Xml(e.to_string()))?;
        for (k, v) in header { if !v.is_null() { XmlBuilder::write_simple(&mut writer, k, v)?; } }
        writer.write_event(Event::End(BytesEnd::new("HEADER"))).map_err(|e| TallyError::Xml(e.to_string()))?;

        if let Some(body_map) = body {
            writer.write_event(Event::Start(BytesStart::new("BODY"))).map_err(|e| TallyError::Xml(e.to_string()))?;

            // DESC
            if let Some(desc) = body_map.get("DESC").and_then(|v| v.as_object()) {
                writer.write_event(Event::Start(BytesStart::new("DESC"))).map_err(|e| TallyError::Xml(e.to_string()))?;
                if let Some(stat) = desc.get("STATICVARIABLES").and_then(|v| v.as_object()) {
                    writer.write_event(Event::Start(BytesStart::new("STATICVARIABLES"))).map_err(|e| TallyError::Xml(e.to_string()))?;
                    for (k, v) in stat { XmlBuilder::write_simple(&mut writer, k, v)?; }
                    writer.write_event(Event::End(BytesEnd::new("STATICVARIABLES"))).map_err(|e| TallyError::Xml(e.to_string()))?;
                }
                if let Some(tdl) = desc.get("TDL").and_then(|v| v.as_object()) {
                    writer.write_event(Event::Start(BytesStart::new("TDL"))).map_err(|e| TallyError::Xml(e.to_string()))?;
                    if let Some(tmsg) = tdl.get("TDLMESSAGE").and_then(|v| v.as_object()) {
                        writer.write_event(Event::Start(BytesStart::new("TDLMESSAGE"))).map_err(|e| TallyError::Xml(e.to_string()))?;
                        for (k, v) in tmsg { XmlBuilder::write_kv_recursive(&mut writer, k, v)?; }
                        writer.write_event(Event::End(BytesEnd::new("TDLMESSAGE"))).map_err(|e| TallyError::Xml(e.to_string()))?;
                    }
                    writer.write_event(Event::End(BytesEnd::new("TDL"))).map_err(|e| TallyError::Xml(e.to_string()))?;
                }
                if let Some(fetchlist) = desc.get("FETCHLIST").and_then(|v| v.as_object()) {
                    writer.write_event(Event::Start(BytesStart::new("FETCHLIST"))).map_err(|e| TallyError::Xml(e.to_string()))?;
                    for (k, v) in fetchlist { XmlBuilder::write_kv_recursive(&mut writer, k, v)?; }
                    writer.write_event(Event::End(BytesEnd::new("FETCHLIST"))).map_err(|e| TallyError::Xml(e.to_string()))?;
                }
                writer.write_event(Event::End(BytesEnd::new("DESC"))).map_err(|e| TallyError::Xml(e.to_string()))?;
            }

            if let Some(data_val) = body_map.get("DATA") {
                writer.write_event(Event::Start(BytesStart::new("DATA"))).map_err(|e| TallyError::Xml(e.to_string()))?;
                XmlBuilder::write_value_recursive(&mut writer, data_val)?;
                writer.write_event(Event::End(BytesEnd::new("DATA"))).map_err(|e| TallyError::Xml(e.to_string()))?;
            }

            writer.write_event(Event::End(BytesEnd::new("BODY"))).map_err(|e| TallyError::Xml(e.to_string()))?;
        }

        writer.write_event(Event::End(BytesEnd::new("ENVELOPE"))).map_err(|e| TallyError::Xml(e.to_string()))?;
        let bytes = writer.into_inner().into_inner();
        Ok(String::from_utf8(bytes).map_err(|e| TallyError::Xml(e.to_string()))?)
    }
}
