use crate::errors::{Result, TallyError};
use crate::models::helpers::{build_gst_details, build_hsn_details};

/// Stock item master used when creating inventory items in Tally.
#[derive(Debug, Clone)]
pub struct StockItem {
    /// Display name.
    pub name: String,
    /// Parent group or stock group name.
    pub parent: Option<String>,
    /// Alternate names / aliases.
    pub alias: Option<Vec<String>>,
    // Units
    /// Base unit of measure.
    pub base_units: Option<String>,
    /// Additional / alternate unit.
    pub additional_units: Option<String>,
    // Statutory
    /// GST applicability (`Applicable` / `Not Applicable`).
    pub gst_applicable: Option<String>,
    /// GST type of supply (Goods, Services, Capital Goods).
    pub gst_type_of_supply: Option<String>,
    /// Basic rate of excise.
    pub basic_rate_of_excise: Option<f64>,
    /// Opening balance.
    pub opening_balance: Option<f64>,

    // HSN/GST
    /// HSN details applicable-from date (`YYYYMMDD`).
    pub hsn_applicable_from: Option<String>,
    /// HSN/SAC code.
    pub hsn_code: Option<String>,
    /// HSN/SAC description.
    pub hsn_description: Option<String>,
    /// HSN classification name when sourced from another master.
    pub hsn_classification_name: Option<String>,
    /// Where HSN details are sourced from.
    pub hsn_source_of_details: Option<String>,
    /// GST details applicable-from date (`YYYYMMDD`).
    pub gst_applicable_from: Option<String>,
    /// GST taxability (for example Taxable).
    pub gst_taxability: Option<String>,
    /// Where GST details are sourced from.
    pub gst_source_of_details: Option<String>,
    /// GST classification name when sourced from another master.
    pub gst_classification_name: Option<String>,
    /// State name for GST rate details.
    pub gst_state_name: Option<String>,
    /// Duty head for the GST rate row.
    pub gst_rate_duty_head: Option<String>,
    /// Valuation type for the GST rate row.
    pub gst_rate_valuation_type: Option<String>,
    /// GST rate percent.
    pub gst_rate: Option<f64>,
}

impl StockItem {
    /// Validate required fields and basic invariants before import.
    pub fn validate(&self) -> Result<()> {
        if self.name.trim().is_empty() {
            return Err(TallyError::Validation("Stock Item name is required".into()));
        }
        if let Some(v) = &self.gst_applicable {
            if v != "Applicable" && v != "Not Applicable" {
                return Err(TallyError::Validation(
                    "GSTAPPLICABLE must be 'Applicable' or 'Not Applicable'".into(),
                ));
            }
        }
        if let Some(v) = &self.gst_type_of_supply {
            let ok = ["Capital Goods", "Goods", "Services"];
            if !ok.contains(&v.as_str()) {
                return Err(TallyError::Validation("Invalid GST Type of Supply".into()));
            }
        }
        Ok(())
    }

    /// Convert to the Tally XML field map used by [`crate::xml_builder::XmlBuilder`].
    pub fn to_map(&self) -> serde_json::Map<String, serde_json::Value> {
        use serde_json::{json, Value};
        let mut m = serde_json::Map::new();
        m.insert("NAME".into(), json!(self.name));
        if let Some(v) = &self.parent {
            m.insert("PARENT".into(), json!(v));
        } else {
            m.insert("PARENT".into(), Value::Null);
        }
        if let Some(v) = &self.alias {
            m.insert("ALIAS".into(), json!(v));
        }
        if let Some(v) = &self.base_units {
            m.insert("BASEUNITS".into(), json!(v));
        }
        if let Some(v) = &self.additional_units {
            m.insert("ADDITIONALUNITS".into(), json!(v));
        }
        if let Some(v) = &self.gst_applicable {
            m.insert("GSTAPPLICABLE".into(), json!(v));
        }
        if let Some(v) = &self.gst_type_of_supply {
            m.insert("GSTTYPEOFSUPPLY".into(), json!(v));
        }
        if let Some(v) = self.basic_rate_of_excise {
            m.insert("BASICRATEOFEXCISE".into(), json!(format!(" {}", v)));
        }
        if let Some(v) = self.opening_balance {
            m.insert("OPENINGBALANCE".into(), json!(format!(" {}", v)));
        }

        if let Some(hsn) = build_hsn_details(
            self.hsn_applicable_from.as_ref(),
            self.hsn_source_of_details.as_ref(),
            self.hsn_code.as_ref(),
            self.hsn_description.as_ref(),
            self.hsn_classification_name.as_ref(),
            false,
        ) {
            m.insert("HSNDETAILS.LIST".into(), Value::Object(hsn));
        }

        if let Some(gst) = build_gst_details(
            self.gst_applicable_from.as_ref(),
            self.gst_taxability.as_ref(),
            self.gst_source_of_details.as_ref(),
            self.gst_classification_name.as_ref(),
            self.gst_state_name.as_ref(),
            self.gst_rate_duty_head.as_ref(),
            self.gst_rate_valuation_type.as_ref(),
            self.gst_rate,
            false,
            true,
        ) {
            m.insert("GSTDETAILS.LIST".into(), Value::Object(gst));
        }

        m
    }
}
