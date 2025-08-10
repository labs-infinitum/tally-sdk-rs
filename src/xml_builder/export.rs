use crate::errors::Result;
use serde_json::Value;

use super::XmlBuilder;

impl XmlBuilder {
    pub fn create_export_request(request_type: &str, collection_type: &str, static_variables: Option<&serde_json::Map<String, Value>>) -> Result<String> {
        let id_value = match collection_type.to_lowercase().as_str() {
            "voucher" => "Day Book",
            "company" | "group" | "ledger" => collection_type,
            _ => &format!("List of {}s", collection_type),
        };

        let mut header = serde_json::Map::new();
        header.insert("VERSION".into(), Value::String("1".into()));
        header.insert("TALLYREQUEST".into(), Value::String("EXPORT".into()));
        header.insert("TYPE".into(), Value::String(request_type.to_string()));
        header.insert("ID".into(), Value::String(id_value.to_string()));

        let mut desc = serde_json::Map::new();
        let mut stat = serde_json::Map::new();
        stat.insert("SVEXPORTFORMAT".into(), Value::String("$$SysName:XML".into()));
        if let Some(sv) = static_variables { for (k, v) in sv { if k != "FETCHLIST" { stat.insert(k.clone(), v.clone()); } } }
        desc.insert("STATICVARIABLES".into(), Value::Object(stat));
        // Only include FETCHLIST for true collections, not for Day Book export
        if collection_type.to_lowercase().as_str() != "voucher" { 
            if let Some(sv) = static_variables { 
                if let Some(fetch) = sv.get("FETCHLIST") { 
                    desc.insert("FETCHLIST".into(), fetch.clone()); 
                } 
            } 
        }

        let mut body = serde_json::Map::new();
        body.insert("DESC".into(), Value::Object(desc));
        XmlBuilder::create_envelope(&header, Some(&body))
    }
}
