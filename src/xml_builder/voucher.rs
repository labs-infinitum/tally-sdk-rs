use crate::errors::{Result, TallyError};
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::Writer;
use serde_json::Value;
use std::io::Cursor;

use super::XmlBuilder;

impl XmlBuilder {
    pub fn create_voucher_request(voucher_map: &serde_json::Map<String, Value>) -> Result<String> {
        let mut w = Writer::new(Cursor::new(Vec::new()));
        w.write_event(Event::Start(BytesStart::new("ENVELOPE"))).map_err(|e| TallyError::Xml(e.to_string()))?;
        w.write_event(Event::Start(BytesStart::new("HEADER"))).map_err(|e| TallyError::Xml(e.to_string()))?;
        XmlBuilder::write_text_node(&mut w, "VERSION", "1")?;
        XmlBuilder::write_text_node(&mut w, "TALLYREQUEST", "Import")?;
        XmlBuilder::write_text_node(&mut w, "TYPE", "Data")?;
        XmlBuilder::write_text_node(&mut w, "ID", "Vouchers")?;
        w.write_event(Event::End(BytesEnd::new("HEADER"))).map_err(|e| TallyError::Xml(e.to_string()))?;

        w.write_event(Event::Start(BytesStart::new("BODY"))).map_err(|e| TallyError::Xml(e.to_string()))?;
        w.write_event(Event::Start(BytesStart::new("DESC"))).map_err(|e| TallyError::Xml(e.to_string()))?;
        w.write_event(Event::End(BytesEnd::new("DESC"))).map_err(|e| TallyError::Xml(e.to_string()))?;
        w.write_event(Event::Start(BytesStart::new("DATA"))).map_err(|e| TallyError::Xml(e.to_string()))?;
        w.write_event(Event::Start(BytesStart::new("TALLYMESSAGE"))).map_err(|e| TallyError::Xml(e.to_string()))?;

        let vch_type = voucher_map.get("VOUCHERTYPENAME").and_then(|v| v.as_str()).unwrap_or("Receipt");
        let mut start = BytesStart::new("VOUCHER");
        start.push_attribute(("VCHTYPE", vch_type));
        start.push_attribute(("ACTION", "Create"));
        start.push_attribute(("OBJVIEW", "Accounting Voucher View"));
        w.write_event(Event::Start(start)).map_err(|e| TallyError::Xml(e.to_string()))?;

        for (k, v) in voucher_map {
            match v {
                Value::Array(arr) => {
                    for entry in arr {
                        if let Some(obj) = entry.as_object() {
                            w.write_event(Event::Start(BytesStart::new(k))).map_err(|e| TallyError::Xml(e.to_string()))?;
                            for (ek, ev) in obj { XmlBuilder::write_simple(&mut w, ek, ev)?; }
                            w.write_event(Event::End(BytesEnd::new(k))).map_err(|e| TallyError::Xml(e.to_string()))?;
                        }
                    }
                }
                _ => { if k != "VOUCHERTYPENAME" { XmlBuilder::write_simple(&mut w, k, v)?; } }
            }
        }

        w.write_event(Event::End(BytesEnd::new("VOUCHER"))).map_err(|e| TallyError::Xml(e.to_string()))?;
        w.write_event(Event::End(BytesEnd::new("TALLYMESSAGE"))).map_err(|e| TallyError::Xml(e.to_string()))?;
        w.write_event(Event::End(BytesEnd::new("DATA"))).map_err(|e| TallyError::Xml(e.to_string()))?;
        w.write_event(Event::End(BytesEnd::new("BODY"))).map_err(|e| TallyError::Xml(e.to_string()))?;
        w.write_event(Event::End(BytesEnd::new("ENVELOPE"))).map_err(|e| TallyError::Xml(e.to_string()))?;
        let bytes = w.into_inner().into_inner();
        Ok(String::from_utf8(bytes).map_err(|e| TallyError::Xml(e.to_string()))?)
    }
}
