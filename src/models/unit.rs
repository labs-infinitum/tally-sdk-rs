use crate::errors::{Result, TallyError};

#[derive(Debug, Clone)]
pub struct Unit {
    pub symbol: String,
    pub formal_name: Option<String>,
    pub decimal_places: Option<u8>,
    pub uqc_name: Option<String>,
    pub applicable_from: String,
}

impl Unit {
    pub fn validate(&self) -> Result<()> {
        if self.symbol.trim().is_empty() { return Err(TallyError::Validation("Unit symbol is required".into())); }
        if let Some(dp) = self.decimal_places { if dp > 6 { return Err(TallyError::Validation("Decimal places must be 0-6".into())); } }
        Ok(())
    }

    pub fn to_map(&self) -> serde_json::Map<String, serde_json::Value> {
        use serde_json::json;
        let mut m = serde_json::Map::new();
        m.insert("NAME".into(), json!(self.symbol));
        if let Some(v) = &self.formal_name { m.insert("ORIGINALNAME".into(), json!(v)); }
        m.insert("ISSIMPLEUNIT".into(), json!("Yes"));
        if let Some(dp) = self.decimal_places { m.insert("DECIMALPLACES".into(), json!(format!(" {}", dp))); }
        if let Some(uqc) = &self.uqc_name {
            let mut u = serde_json::Map::new();
            u.insert("APPLICABLEFROM".into(), json!(self.applicable_from.clone()));
            u.insert("REPORTINGUQCNAME".into(), json!(uqc));
            m.insert("REPORTINGUQCDETAILS.LIST".into(), serde_json::Value::Object(u));
        }
        m
    }
}
