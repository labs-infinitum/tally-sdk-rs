use crate::errors::{Result, TallyError};

/// Unit of measure master.
#[derive(Debug, Clone)]
pub struct Unit {
    /// Unit symbol (stored as NAME in Tally).
    pub symbol: String,
    /// Formal / original unit name.
    pub formal_name: Option<String>,
    /// Decimal places (0–6).
    pub decimal_places: Option<u8>,
    /// Reporting UQC name.
    pub uqc_name: Option<String>,
    /// UQC details applicable-from date (`YYYYMMDD`).
    pub applicable_from: String,
}

impl Unit {
    /// Validate required fields and basic invariants before import.
    pub fn validate(&self) -> Result<()> {
        if self.symbol.trim().is_empty() {
            return Err(TallyError::Validation("Unit symbol is required".into()));
        }
        if let Some(dp) = self.decimal_places {
            if dp > 6 {
                return Err(TallyError::Validation("Decimal places must be 0-6".into()));
            }
        }
        Ok(())
    }

    /// Convert to the Tally XML field map used by [`crate::xml_builder::XmlBuilder`].
    pub fn to_map(&self) -> serde_json::Map<String, serde_json::Value> {
        use serde_json::json;
        let mut m = serde_json::Map::new();
        m.insert("NAME".into(), json!(self.symbol));
        if let Some(v) = &self.formal_name {
            m.insert("ORIGINALNAME".into(), json!(v));
        }
        m.insert("ISSIMPLEUNIT".into(), json!("Yes"));
        if let Some(dp) = self.decimal_places {
            m.insert("DECIMALPLACES".into(), json!(format!(" {}", dp)));
        }
        if let Some(uqc) = &self.uqc_name {
            let mut u = serde_json::Map::new();
            u.insert("APPLICABLEFROM".into(), json!(self.applicable_from.clone()));
            u.insert("REPORTINGUQCNAME".into(), json!(uqc));
            m.insert(
                "REPORTINGUQCDETAILS.LIST".into(),
                serde_json::Value::Object(u),
            );
        }
        m
    }
}
