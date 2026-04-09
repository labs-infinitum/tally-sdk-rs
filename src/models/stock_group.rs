use crate::errors::{Result, TallyError};
use crate::models::helpers::{build_gst_details, build_hsn_details};

#[derive(Debug, Clone)]
pub struct StockGroup {
    pub name: String,
    pub parent: Option<String>,
    pub alias: Option<Vec<String>>,
    pub is_addable: Option<String>,
    pub as_original: Option<String>,
    // HSN/GST reuse from Group
    pub hsn_applicable_from: Option<String>,
    pub hsn_code: Option<String>,
    pub hsn_description: Option<String>,
    pub hsn_classification_name: Option<String>,
    pub hsn_source_of_details: Option<String>,
    pub gst_applicable_from: Option<String>,
    pub gst_taxability: Option<String>,
    pub gst_source_of_details: Option<String>,
    pub gst_classification_name: Option<String>,
    pub gst_state_name: Option<String>,
    pub gst_rate_duty_head: Option<String>,
    pub gst_rate_valuation_type: Option<String>,
    pub gst_rate: Option<f64>,
}

impl StockGroup {
    pub fn validate(&self) -> Result<()> {
        if self.name.trim().is_empty() {
            return Err(TallyError::Validation(
                "Stock Group name is required".into(),
            ));
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

    pub fn to_map(&self) -> serde_json::Map<String, serde_json::Value> {
        use serde_json::{json, Value};
        let mut m = serde_json::Map::new();
        m.insert("NAME".into(), json!(self.name));
        if let Some(v) = &self.parent {
            m.insert("PARENT".into(), json!(v));
        }
        if let Some(v) = &self.alias {
            m.insert("ALIAS".into(), json!(v));
        }
        if let Some(v) = &self.is_addable {
            m.insert("ISADDABLE".into(), json!(v));
        }
        if let Some(v) = &self.as_original {
            m.insert("ASORIGINAL".into(), json!(v));
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
