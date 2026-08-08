use crate::errors::{Result, TallyError};
use std::fmt;

/// Per-duty GST rate detail on an inventory line.
#[derive(Debug, Clone)]
pub struct GstRateDetail {
    /// Duty head (CGST, SGST/UTGST, IGST, Cess, …).
    pub duty_head: String, // CGST, SGST/UTGST, IGST, Cess, State Cess
    /// Rate per unit.
    pub rate: f32,
    /// Valuation type for the duty rate.
    pub valuation_type: Option<String>,
}

/// Multicurrency details embedded in Tally `AMOUNT` values such as
/// `EUR29.37 @ D$1.1675/EUR = D$34.29`.
#[derive(Debug, Clone, PartialEq)]
pub struct ForexDetails {
    /// Amount in the foreign/transaction currency.
    pub foreign_amount: f32,
    /// Foreign/transaction currency symbol, e.g. `EUR`.
    pub foreign_currency: String,
    /// Resolved foreign currency name when available, e.g. `EURO`.
    pub foreign_currency_name: Option<String>,
    /// Company/base currency symbol, e.g. `D$`.
    pub base_currency: String,
    /// Resolved base currency name when available, e.g. `Dollar`.
    pub base_currency_name: Option<String>,
    /// Exchange rate as base-currency units per 1 foreign unit.
    pub exchange_rate: f32,
}

impl ForexDetails {
    /// Prefer the resolved currency name, otherwise the symbol.
    pub fn foreign_currency_label(&self) -> &str {
        self.foreign_currency_name
            .as_deref()
            .unwrap_or(self.foreign_currency.as_str())
    }

    /// Prefer the resolved currency name, otherwise the symbol.
    pub fn base_currency_label(&self) -> &str {
        self.base_currency_name
            .as_deref()
            .unwrap_or(self.base_currency.as_str())
    }
}

/// Batch/godown allocation on an inventory line.
#[derive(Debug, Clone)]
pub struct BatchAllocation {
    /// Godown / location name.
    pub godown_name: String,
    /// Batch name.
    pub batch_name: String,
    /// Amount in company/base currency.
    pub amount: f32,
    /// Optional multicurrency breakdown for the amount.
    pub forex: Option<ForexDetails>,
    /// Actual quantity.
    pub actual_qty: Option<f32>,
    /// Billed quantity.
    pub billed_qty: Option<f32>,
}

/// Accounting allocation attached to an inventory line.
#[derive(Debug, Clone)]
pub struct AccountingAllocation {
    /// Ledger name.
    pub ledger_name: String,
    /// Amount in company/base currency.
    pub amount: f32,
    /// Optional multicurrency breakdown for the amount.
    pub forex: Option<ForexDetails>,
    /// Tally `ISDEEMEDPOSITIVE` flag.
    pub is_deemed_positive: bool,
}

/// Bill-wise allocation on a ledger entry (for invoice settlement).
#[derive(Debug, Clone, PartialEq)]
pub struct BillAllocation {
    /// Bill / invoice reference name.
    pub bill_name: String,
    /// Bill type (New Ref, Agst Ref, …).
    pub bill_type: Option<String>,
    /// Amount in company/base currency.
    pub amount: f32,
    /// Optional multicurrency breakdown for the amount.
    pub forex: Option<ForexDetails>,
}

/// One ledger line on an accounting voucher.
#[derive(Debug, Clone)]
pub struct VoucherEntry {
    /// Ledger name.
    pub ledger_name: String,
    /// Amount in company/base currency.
    pub amount: f32,
    /// Optional multicurrency breakdown for the amount.
    pub forex: Option<ForexDetails>,
    /// Whether this ledger entry is a debit.
    pub is_debit: bool,
    /// Whether this is the party ledger line.
    pub is_party_ledger: bool,
    /// Bill-wise allocations on this entry.
    pub bill_allocations: Vec<BillAllocation>,
}

impl VoucherEntry {
    /// Convert to the Tally XML field map used by [`crate::xml_builder::XmlBuilder`].
    pub fn to_map(&self) -> serde_json::Map<String, serde_json::Value> {
        use serde_json::json;
        let mut m = serde_json::Map::new();
        m.insert("LEDGERNAME".into(), json!(self.ledger_name.clone()));
        // Tally import convention: debit = ISDEEMEDPOSITIVE=Yes + negative AMOUNT;
        // credit = ISDEEMEDPOSITIVE=No + positive AMOUNT.
        m.insert(
            "ISDEEMEDPOSITIVE".into(),
            json!(if self.is_debit { "Yes" } else { "No" }),
        );
        m.insert(
            "ISPARTYLEDGER".into(),
            json!(if self.is_party_ledger { "Yes" } else { "No" }),
        );
        // Tally imports are more reliable with fixed 2-decimal string amounts
        // (matches working Payment/Receipt XML samples).
        let amt = if self.is_debit {
            format!("-{:.2}", self.amount.abs())
        } else {
            format!("{:.2}", self.amount.abs())
        };
        m.insert("AMOUNT".into(), json!(amt));
        m
    }
}

/// Inventory line on a voucher.
#[derive(Debug, Clone)]
pub struct Item {
    /// Display name.
    pub name: String,
    /// Amount in company/base currency.
    pub amount: f32,
    /// Optional multicurrency breakdown for the amount.
    pub forex: Option<ForexDetails>,
    /// Rate per unit.
    pub rate: Option<f32>,
    /// Discount percent or amount when present.
    pub discount: Option<f32>,
    /// Actual quantity.
    pub actual_qty: Option<f32>,
    /// Billed quantity.
    pub billed_qty: Option<f32>,
    /// Line-level HSN/SAC code.
    pub gst_hsn_code: Option<String>,
    /// Line-level HSN/SAC description.
    pub gst_hsn_description: Option<String>,
    /// GST taxability (for example Taxable).
    pub gst_taxability: Option<String>,
    /// GST type of supply (Goods, Services, Capital Goods).
    pub gst_type_of_supply: Option<String>,
    /// Batch / godown allocations.
    pub batch_allocations: Vec<BatchAllocation>,
    /// Accounting allocations for inventory lines.
    pub accounting_allocations: Vec<AccountingAllocation>,
    /// Per-duty GST rate rows.
    pub gst_rate_details: Vec<GstRateDetail>,
}

/// Accounting or inventory voucher (create or export).
#[derive(Debug, Clone)]
pub struct Voucher {
    /// Tally voucher GUID / id.
    pub voucher_id: String,
    /// Remote id attribute when present.
    pub remote_id: Option<String>,
    /// Tally `VCHKEY` when present.
    pub vch_key: Option<String>,
    /// Voucher type name (for example Sales, Purchase, Payment).
    pub voucher_type: String,
    /// Import action (Create, Alter, …).
    pub action: Option<String>,
    /// Voucher date in `YYYYMMDD` form.
    pub date_yyyymmdd: String,
    /// Amount in company/base currency.
    pub amount: Option<f32>,
    /// Optional multicurrency breakdown for the voucher amount.
    pub amount_forex: Option<ForexDetails>,
    /// Voucher number.
    pub voucher_number: Option<String>,
    /// Reference / supplier invoice number.
    pub reference: Option<String>,
    /// Party ledger name.
    pub party_ledger_name: Option<String>,
    /// Company GST registration type on the voucher.
    pub cmp_gst_registration_type: Option<String>,
    /// Party GSTIN.
    pub party_gstin: Option<String>,
    /// Company GSTIN on the voucher.
    pub cmp_gstin: Option<String>,
    /// Place of supply.
    pub place_of_supply: Option<String>,
    /// Report rows or ledger entries.
    pub entries: Vec<VoucherEntry>,
    /// Inventory / item lines.
    pub items: Vec<Item>,
    /// Narration / remarks.
    pub narration: Option<String>,
    /// Reference date (`YYYYMMDD`).
    pub reference_date: Option<String>,
    /// Effective date (`YYYYMMDD`).
    pub effective_date: Option<String>,
    /// Whether the voucher is an invoice.
    pub is_invoice: bool,
    /// Whether the voucher is cancelled.
    pub is_cancelled: bool,
    /// Whether the voucher is optional.
    pub is_optional: bool,
    /// Entry mode (for example Item Invoice).
    pub entry_mode: Option<String>,
    /// Tally alter id.
    pub alter_id: Option<i32>,
    /// Tally master id.
    pub master_id: Option<i32>,
}

impl Voucher {
    /// Validate required fields and basic invariants before import.
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

    /// Convert to the Tally XML field map used by [`crate::xml_builder::XmlBuilder`].
    pub fn to_map(&self) -> serde_json::Map<String, serde_json::Value> {
        use serde_json::{json, Value};
        let mut m = serde_json::Map::new();
        m.insert("VOUCHERTYPENAME".into(), json!(self.voucher_type.clone()));
        m.insert("OBJVIEW".into(), json!("Accounting Voucher View"));
        m.insert("PERSISTEDVIEW".into(), json!("Accounting Voucher View"));
        m.insert(
            "ISINVOICE".into(),
            json!(if self.is_invoice { "Yes" } else { "No" }),
        );
        m.insert("DATE".into(), json!(self.date_yyyymmdd.clone()));
        m.insert(
            "EFFECTIVEDATE".into(),
            json!(self
                .effective_date
                .clone()
                .unwrap_or_else(|| self.date_yyyymmdd.clone())),
        );
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
        // Accounting Payment/Receipt samples typically use ALLLEDGERENTRIES.LIST.
        m.insert("ALLLEDGERENTRIES.LIST".into(), Value::Array(arr));
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

impl fmt::Display for ForexDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:.4} {} @ {} {:.6}/{} = {:.4} {}",
            self.foreign_amount,
            self.foreign_currency_label(),
            self.base_currency_label(),
            self.exchange_rate,
            self.foreign_currency_label(),
            self.foreign_amount.abs() * self.exchange_rate,
            self.base_currency_label()
        )
    }
}

impl fmt::Display for BatchAllocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} @ {}: {}",
            self.batch_name, self.godown_name, self.amount
        )?;
        if let Some(forex) = &self.forex {
            write!(f, " ({})", forex)?;
        }
        Ok(())
    }
}

impl fmt::Display for AccountingAllocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.ledger_name, self.amount)?;
        if let Some(forex) = &self.forex {
            write!(f, " ({})", forex)?;
        }
        Ok(())
    }
}

impl fmt::Display for VoucherEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let side = if self.is_debit { "Dr" } else { "Cr" };
        write!(f, "{}: {} ({})", self.ledger_name, self.amount, side)?;
        if self.is_party_ledger {
            write!(f, " [Party]")?;
        }
        if let Some(forex) = &self.forex {
            write!(f, " | {}", forex)?;
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
        if let Some(forex) = &self.amount_forex {
            writeln!(f, "Amount Forex: {}", forex)?;
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
