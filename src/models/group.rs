use crate::errors::{Result, TallyError};
use crate::models::helpers::{build_gst_details, build_hsn_details};

/// Account group master used when creating groups in Tally.
#[derive(Debug, Clone)]
pub struct Group {
    /// Display name.
    pub name: String,
    /// Parent group or stock group name.
    pub parent: Option<String>,
    /// Group type.
    pub group_type: Option<String>,
    /// Alternate names / aliases.
    pub alias: Option<Vec<String>>,

    /// Whether the group is calculable (`Yes`/`No`).
    pub basic_group_is_calculable: Option<String>,
    /// Whether the group is addable (`Yes`/`No`).
    pub is_addable: Option<String>,
    /// Whether this is a sub-ledger group.
    pub is_subledger: Option<String>,
    /// Additional allocation type.
    pub addl_alloc_type: Option<String>,
    /// Keep values as original (`Yes`/`No`).
    pub as_original: Option<String>,
    /// Whether the group affects gross profit.
    pub affects_gross_profit: Option<String>,

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
    /// Duty head for the GST rate row.
    pub gst_rate_duty_head: Option<String>,
    /// Valuation type for the GST rate row.
    pub gst_rate_valuation_type: Option<String>,
    /// GST rate percent.
    pub gst_rate: Option<f64>,
    /// State name for GST rate details.
    pub gst_state_name: Option<String>,
}

impl Group {
    /// Validate required fields and basic invariants before import.
    pub fn validate(&self) -> Result<()> {
        if self.name.trim().is_empty() {
            return Err(TallyError::Validation("Group name is required".into()));
        }
        if let Some(v) = &self.is_addable {
            if v != "Yes" && v != "No" {
                return Err(TallyError::Validation(
                    "ISADDABLE must be 'Yes' or 'No'".into(),
                ));
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
        if let Some(v) = &self.group_type {
            m.insert("GROUP_TYPE".into(), json!(v));
        }
        if let Some(v) = &self.alias {
            m.insert("ALIAS".into(), json!(v));
        }
        if let Some(v) = &self.basic_group_is_calculable {
            m.insert("BASICGROUPISCALCULABLE".into(), json!(v));
        }
        if let Some(v) = &self.is_addable {
            m.insert("ISADDABLE".into(), json!(v));
        }
        if let Some(v) = &self.is_subledger {
            m.insert("ISSUBLEDGER".into(), json!(v));
        }
        if let Some(v) = &self.addl_alloc_type {
            m.insert("ADDLALLOCTYPE".into(), json!(v));
        }
        if let Some(v) = &self.as_original {
            m.insert("ASORIGINAL".into(), json!(v));
        }
        if let Some(v) = &self.affects_gross_profit {
            m.insert("AFFECTSGROSSPROFIT".into(), json!(v));
        }

        if let Some(hsn) = build_hsn_details(
            self.hsn_applicable_from.as_ref(),
            self.hsn_source_of_details.as_ref(),
            self.hsn_code.as_ref(),
            self.hsn_description.as_ref(),
            self.hsn_classification_name.as_ref(),
            true,
        ) {
            m.insert("HSNDETAILS.LIST".into(), serde_json::Value::Object(hsn));
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
            true,
            true,
        ) {
            m.insert("GSTDETAILS.LIST".into(), serde_json::Value::Object(gst));
        }

        m
    }
}
