use crate::errors::{Result, TallyError};

#[derive(Debug, Clone)]
pub struct VoucherEntry {
    pub ledger_name: String,
    pub amount: f64,
    pub is_debit: bool,
}

impl VoucherEntry {
    pub fn to_map(&self) -> serde_json::Map<String, serde_json::Value> {
        use serde_json::json;
        let mut m = serde_json::Map::new();
        m.insert("LEDGERNAME".into(), json!(self.ledger_name.clone()));
        m.insert("ISDEEMEDPOSITIVE".into(), json!(if self.is_debit { "No" } else { "Yes" }));
        let amt = if self.is_debit { self.amount } else { -self.amount.abs() };
        m.insert("AMOUNT".into(), json!(amt));
        m
    }
}

#[derive(Debug, Clone)]
pub struct Voucher {
    pub voucher_type: String,
    pub date_yyyymmdd: String,
    pub entries: Vec<VoucherEntry>,
    pub narration: Option<String>,
}

impl Voucher {
    pub fn validate(&self) -> Result<()> {
        if self.voucher_type.trim().is_empty() { return Err(TallyError::Validation("Voucher type is required".into())); }
        if self.entries.len() < 2 { return Err(TallyError::Validation("Voucher must have at least 2 entries".into())); }
        let mut deb = 0.0; let mut cred = 0.0;
        for e in &self.entries { if e.is_debit { deb += e.amount; } else { cred += e.amount; } }
        if (deb - cred).abs() > 0.01 { return Err(TallyError::Validation(format!("Voucher not balanced. Debits: {}, Credits: {}", deb, cred))); }
        Ok(())
    }

    pub fn to_map(&self) -> serde_json::Map<String, serde_json::Value> {
        use serde_json::{json, Value};
        let mut m = serde_json::Map::new();
        m.insert("VOUCHERTYPENAME".into(), json!(self.voucher_type.clone()));
        m.insert("DATE".into(), json!(self.date_yyyymmdd.clone()));
        if let Some(n) = &self.narration { m.insert("NARRATION".into(), json!(n)); }
        let arr: Vec<Value> = self.entries.iter().map(|e| Value::Object(e.to_map())).collect();
        m.insert("LEDGERENTRIES.LIST".into(), Value::Array(arr));
        m
    }
}
