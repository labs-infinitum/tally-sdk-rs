use crate::errors::{Result, TallyError};
use std::fmt;

#[derive(Debug, Clone)]
pub struct GstRateDetail {
    pub duty_head: String, // CGST, SGST/UTGST, IGST, Cess, State Cess
    pub rate: f32,
    pub valuation_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BatchAllocation {
    pub godown_name: String,
    pub batch_name: String,
    pub amount: f32,
    pub actual_qty: Option<f32>,
    pub billed_qty: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct AccountingAllocation {
    pub ledger_name: String,
    pub amount: f32,
    pub is_deemed_positive: bool,
}

#[derive(Debug, Clone)]
pub struct VoucherEntry {
    pub ledger_name: String,
    pub amount: f32,
    pub is_debit: bool,
    pub is_party_ledger: bool,
}

impl VoucherEntry {
    pub fn to_map(&self) -> serde_json::Map<String, serde_json::Value> {
        use serde_json::json;
        let mut m = serde_json::Map::new();
        m.insert("LEDGERNAME".into(), json!(self.ledger_name.clone()));
        m.insert(
            "ISDEEMEDPOSITIVE".into(),
            json!(if self.is_debit { "No" } else { "Yes" }),
        );
        m.insert(
            "ISPARTYLEDGER".into(),
            json!(if self.is_party_ledger { "Yes" } else { "No" }),
        );
        let amt = if self.is_debit {
            self.amount
        } else {
            -self.amount.abs()
        };
        m.insert("AMOUNT".into(), json!(amt));
        m
    }
}

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub amount: f32,
    pub rate: Option<f32>,
    pub discount: Option<f32>,
    pub actual_qty: Option<f32>,
    pub billed_qty: Option<f32>,
    pub gst_hsn_code: Option<String>,
    pub gst_hsn_description: Option<String>,
    pub gst_taxability: Option<String>,
    pub gst_type_of_supply: Option<String>,
    pub batch_allocations: Vec<BatchAllocation>,
    pub accounting_allocations: Vec<AccountingAllocation>,
    pub gst_rate_details: Vec<GstRateDetail>,
}

#[derive(Debug, Clone)]
pub struct Voucher {
    pub voucher_id: String,
    pub remote_id: Option<String>,
    pub vch_key: Option<String>,
    pub voucher_type: String,
    pub action: Option<String>,
    pub date_yyyymmdd: String,
    pub amount: Option<f32>,
    pub voucher_number: Option<String>,
    pub reference: Option<String>,
    pub party_ledger_name: Option<String>,
    pub cmp_gst_registration_type: Option<String>,
    pub party_gstin: Option<String>,
    pub cmp_gstin: Option<String>,
    pub place_of_supply: Option<String>,
    pub entries: Vec<VoucherEntry>,
    pub items: Vec<Item>,
    pub narration: Option<String>,
    pub reference_date: Option<String>,
    pub effective_date: Option<String>,
    pub is_invoice: bool,
    pub is_cancelled: bool,
    pub is_optional: bool,
    pub entry_mode: Option<String>,
    pub alter_id: Option<i32>,
    pub master_id: Option<i32>,
}

impl Voucher {
    pub fn validate(&self) -> Result<()> {
        if self.voucher_type.trim().is_empty() {
            return Err(TallyError::Validation("Voucher type is required".into()));
        }
        if self.entries.len() < 2 {
            return Err(TallyError::Validation(
                "Voucher must have at least 2 entries".into(),
            ));
        }
        let mut deb = 0.0;
        let mut cred = 0.0;
        for e in &self.entries {
            if e.is_debit {
                deb += e.amount;
            } else {
                cred += e.amount;
            }
        }
        if (deb - cred).abs() > 0.01 {
            return Err(TallyError::Validation(format!(
                "Voucher not balanced. Debits: {}, Credits: {}",
                deb, cred
            )));
        }
        Ok(())
    }

    pub fn to_map(&self) -> serde_json::Map<String, serde_json::Value> {
        use serde_json::{json, Value};
        let mut m = serde_json::Map::new();
        m.insert("VOUCHERTYPENAME".into(), json!(self.voucher_type.clone()));
        m.insert("DATE".into(), json!(self.date_yyyymmdd.clone()));
        if let Some(n) = &self.narration {
            m.insert("NARRATION".into(), json!(n));
        }
        if let Some(vn) = &self.voucher_number {
            m.insert("VOUCHERNUMBER".into(), json!(vn));
        }
        if let Some(r) = &self.reference {
            m.insert("REFERENCE".into(), json!(r));
        }
        if let Some(p) = &self.party_ledger_name {
            m.insert("PARTYLEDGERNAME".into(), json!(p));
        }
        let arr: Vec<Value> = self
            .entries
            .iter()
            .map(|e| Value::Object(e.to_map()))
            .collect();
        m.insert("LEDGERENTRIES.LIST".into(), Value::Array(arr));
        m
    }
}

// Display implementations
impl fmt::Display for GstRateDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}%", self.duty_head, self.rate)?;
        if let Some(vt) = &self.valuation_type {
            write!(f, " ({})", vt)?;
        }
        Ok(())
    }
}

impl fmt::Display for BatchAllocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} @ {}: {}",
            self.batch_name, self.godown_name, self.amount
        )
    }
}

impl fmt::Display for AccountingAllocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.ledger_name, self.amount)
    }
}

impl fmt::Display for VoucherEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let side = if self.is_debit { "Dr" } else { "Cr" };
        write!(f, "{}: {} ({})", self.ledger_name, self.amount, side)?;
        if self.is_party_ledger {
            write!(f, " [Party]")?;
        }
        Ok(())
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.amount)?;
        if let Some(hsn) = &self.gst_hsn_code {
            write!(f, " (HSN: {})", hsn)?;
        }
        writeln!(f)?;

        if let Some(qty) = self.actual_qty {
            write!(f, "    Qty: {}", qty)?;
            if let Some(rate) = self.rate {
                write!(f, " @ {}", rate)?;
            }
            writeln!(f)?;
        }

        if !self.batch_allocations.is_empty() {
            writeln!(f, "    Batches:")?;
            for batch in &self.batch_allocations {
                writeln!(f, "      - {}", batch)?;
            }
        }

        if !self.accounting_allocations.is_empty() {
            writeln!(f, "    Ledgers:")?;
            for acct in &self.accounting_allocations {
                writeln!(f, "      - {}", acct)?;
            }
        }

        if !self.gst_rate_details.is_empty() {
            let non_zero: Vec<_> = self
                .gst_rate_details
                .iter()
                .filter(|r| r.rate > 0.0)
                .collect();
            if !non_zero.is_empty() {
                writeln!(f, "    GST Rates:")?;
                for rate in non_zero {
                    writeln!(f, "      - {}", rate)?;
                }
            }
        }

        Ok(())
    }
}

impl fmt::Display for Voucher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== Voucher ===")?;
        writeln!(f, "ID: {}", self.voucher_id)?;
        if let Some(rid) = &self.remote_id {
            writeln!(f, "Remote ID: {}", rid)?;
        }
        writeln!(f, "Type: {}", self.voucher_type)?;
        if let Some(act) = &self.action {
            writeln!(f, "Action: {}", act)?;
        }
        writeln!(f, "Date: {}", self.date_yyyymmdd)?;
        if let Some(amount) = self.amount {
            writeln!(f, "Amount: {}", amount)?;
        }

        if let Some(vn) = &self.voucher_number {
            writeln!(f, "Number: {}", vn)?;
        }
        if let Some(r) = &self.reference {
            writeln!(f, "Reference: {}", r)?;
        }
        if let Some(p) = &self.party_ledger_name {
            writeln!(f, "Party: {}", p)?;
        }
        if let Some(n) = &self.narration {
            writeln!(f, "Narration: {}", n)?;
        }
        if let Some(em) = &self.entry_mode {
            writeln!(f, "Entry Mode: {}", em)?;
        }

        if self.is_cancelled {
            writeln!(f, "Status: CANCELLED")?;
        }

        if !self.items.is_empty() {
            writeln!(f, "\nItems ({}):", self.items.len())?;
            for item in &self.items {
                write!(f, "  - {}", item)?;
            }
        }

        if !self.entries.is_empty() {
            writeln!(f, "\nLedger Entries ({}):", self.entries.len())?;
            for entry in &self.entries {
                writeln!(f, "  - {}", entry)?;
            }
        }

        Ok(())
    }
}
