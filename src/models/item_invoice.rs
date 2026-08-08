use crate::errors::{Result, TallyError};

/// Simplified item-invoice voucher payload for Sales/Purchase.
#[derive(Debug, Clone)]
pub struct ItemInvoice {
    /// Voucher type name (for example Sales, Purchase, Payment).
    pub voucher_type: String, // Sales or Purchase
    /// Voucher date in `YYYYMMDD` form.
    pub date_yyyymmdd: String,
    /// Party ledger name.
    pub party_ledger_name: String,
    /// Sales/purchase ledger used on the inventory allocation.
    pub line_ledger_name: String,
    /// Stock item name.
    pub item_name: String,
    /// Quantity.
    pub quantity: f64,
    /// Unit of measure for quantity/rate.
    pub unit: String,
    /// Rate per unit.
    pub rate: f64,
    /// Narration / remarks.
    pub narration: Option<String>,
    /// Voucher number.
    pub voucher_number: Option<String>,
    /// Supplier invoice / reference number.
    pub supplier_invoice_no: Option<String>,
    /// Supplier invoice date (`YYYYMMDD`).
    pub supplier_invoice_date_yyyymmdd: Option<String>,
    /// Receipt note number.
    pub receipt_note_no: Option<String>,
    /// Receipt note date (`YYYYMMDD`).
    pub receipt_note_date_yyyymmdd: Option<String>,
    /// Shipping / receipt document number.
    pub receipt_doc_no: Option<String>,
    /// Dispatched-through / shipped-by value.
    pub dispatched_through: Option<String>,
    /// Final destination.
    pub destination: Option<String>,
    /// Carrier / agent name.
    pub carrier_name_agent: Option<String>,
    /// Bill of lading number.
    pub bill_of_lading_no: Option<String>,
    /// Bill of lading date (`YYYYMMDD`).
    pub bill_of_lading_date_yyyymmdd: Option<String>,
    /// Motor vehicle / vessel number.
    pub motor_vehicle_no: Option<String>,
    /// Override inventory `ISDEEMEDPOSITIVE` (defaults by voucher type).
    pub inventory_is_deemed_positive: Option<bool>,
    /// Override party `ISDEEMEDPOSITIVE`.
    pub party_is_deemed_positive: Option<bool>,
}

impl ItemInvoice {
    /// Validate required fields and basic invariants before import.
    pub fn validate(&self) -> Result<()> {
        if !matches!(self.voucher_type.as_str(), "Sales" | "Purchase") {
            return Err(TallyError::Validation(
                "voucher_type must be Sales or Purchase".into(),
            ));
        }
        if self.party_ledger_name.trim().is_empty()
            || self.line_ledger_name.trim().is_empty()
            || self.item_name.trim().is_empty()
            || self.unit.trim().is_empty()
        {
            return Err(TallyError::Validation(
                "party_ledger_name, line_ledger_name, item_name, unit required".into(),
            ));
        }
        if self.quantity <= 0.0 || self.rate <= 0.0 {
            return Err(TallyError::Validation(
                "quantity and rate must be > 0".into(),
            ));
        }
        Ok(())
    }

    /// Convert to the Tally XML field map used by [`crate::xml_builder::XmlBuilder`].
    pub fn to_map(&self) -> serde_json::Map<String, serde_json::Value> {
        use serde_json::{json, Value};
        let mut m = serde_json::Map::new();
        m.insert("VOUCHERTYPENAME".into(), json!(self.voucher_type.clone()));
        m.insert("DATE".into(), json!(self.date_yyyymmdd.clone()));
        m.insert("VCHENTRYMODE".into(), json!("Item Invoice"));
        if let Some(v) = &self.voucher_number {
            m.insert("VOUCHERNUMBER".into(), json!(v));
        }
        if let Some(v) = &self.narration {
            m.insert("NARRATION".into(), json!(v));
        }
        if self.voucher_type == "Purchase" {
            if let Some(v) = &self.supplier_invoice_no {
                m.insert("REFERENCE".into(), json!(v));
            }
            if let Some(v) = &self.supplier_invoice_date_yyyymmdd {
                m.insert("REFERENCEDATE".into(), json!(v));
            }
        }
        if let Some(v) = &self.receipt_doc_no {
            m.insert("BASICSHIPDOCUMENTNO".into(), json!(v));
        }
        if let Some(v) = &self.dispatched_through {
            m.insert("BASICSHIPPEDBY".into(), json!(v));
        }
        if let Some(v) = &self.destination {
            m.insert("BASICFINALDESTINATION".into(), json!(v));
        }
        if let Some(v) = &self.carrier_name_agent {
            m.insert("EICHECKPOST".into(), json!(v));
        }
        if let Some(v) = &self.bill_of_lading_no {
            m.insert("BILLOFLADINGNO".into(), json!(v));
        }
        if let Some(v) = &self.bill_of_lading_date_yyyymmdd {
            m.insert("BILLOFLADINGDATE".into(), json!(v));
        }
        if let Some(v) = &self.motor_vehicle_no {
            m.insert("BASICSHIPVESSELNO".into(), json!(v));
        }
        if self.receipt_note_no.is_some() || self.receipt_note_date_yyyymmdd.is_some() {
            let mut inv_del = serde_json::Map::new();
            if let Some(v) = &self.receipt_note_date_yyyymmdd {
                inv_del.insert("BASICSHIPPINGDATE".into(), json!(v));
            }
            if let Some(v) = &self.receipt_note_no {
                inv_del.insert("BASICSHIPDELIVERYNOTE".into(), json!(v));
            }
            m.insert("INVOICEDELNOTES.LIST".into(), Value::Object(inv_del));
        }
        let total = self.quantity * self.rate;
        let inv_yes = self
            .inventory_is_deemed_positive
            .unwrap_or(self.voucher_type == "Sales");
        let party_yes = self.party_is_deemed_positive.unwrap_or(!inv_yes);
        let inv_amount = if inv_yes {
            format!("-{:.2}", total)
        } else {
            format!("{:.2}", total)
        };
        let party_amount = if party_yes {
            format!("-{:.2}", total)
        } else {
            format!("{:.2}", total)
        };

        let mut allinv = serde_json::Map::new();
        allinv.insert("STOCKITEMNAME".into(), json!(self.item_name.clone()));
        allinv.insert(
            "ISDEEMEDPOSITIVE".into(),
            json!(if inv_yes { "Yes" } else { "No" }),
        );
        allinv.insert(
            "RATE".into(),
            json!(format!(" {}/ {}", self.rate, self.unit)),
        );
        allinv.insert("AMOUNT".into(), json!(inv_amount));
        allinv.insert(
            "ACTUALQTY".into(),
            json!(format!(" {} {}", self.quantity, self.unit)),
        );
        allinv.insert(
            "BILLEDQTY".into(),
            json!(format!(" {} {}", self.quantity, self.unit)),
        );
        let mut acc = serde_json::Map::new();
        acc.insert("LEDGERNAME".into(), json!(self.line_ledger_name.clone()));
        acc.insert(
            "ISDEEMEDPOSITIVE".into(),
            json!(if inv_yes { "Yes" } else { "No" }),
        );
        acc.insert("AMOUNT".into(), json!(inv_amount));
        allinv.insert(
            "ACCOUNTINGALLOCATIONS.LIST".into(),
            serde_json::Value::Object(acc),
        );
        m.insert(
            "ALLINVENTORYENTRIES.LIST".into(),
            serde_json::Value::Object(allinv),
        );

        let mut party = serde_json::Map::new();
        party.insert("LEDGERNAME".into(), json!(self.party_ledger_name.clone()));
        party.insert(
            "ISDEEMEDPOSITIVE".into(),
            json!(if party_yes { "Yes" } else { "No" }),
        );
        party.insert("AMOUNT".into(), json!(party_amount));
        party.insert("ISPARTYLEDGER".into(), json!("Yes"));
        m.insert(
            "LEDGERENTRIES.LIST".into(),
            serde_json::Value::Object(party),
        );

        m
    }
}
