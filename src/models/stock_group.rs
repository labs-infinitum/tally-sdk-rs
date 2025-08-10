use crate::errors::{Result, TallyError};

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
        if self.name.trim().is_empty() { return Err(TallyError::Validation("Stock Group name is required".into())); }
        if let Some(v) = &self.is_addable { if v != "Yes" && v != "No" { return Err(TallyError::Validation("ISADDABLE must be 'Yes' or 'No'".into())); } }
        Ok(())
    }

    pub fn to_map(&self) -> serde_json::Map<String, serde_json::Value> {
        use serde_json::{json, Value};
        let mut m = serde_json::Map::new();
        m.insert("NAME".into(), json!(self.name));
        if let Some(v) = &self.parent { m.insert("PARENT".into(), json!(v)); }
        if let Some(v) = &self.alias { m.insert("ALIAS".into(), json!(v)); }
        if let Some(v) = &self.is_addable { m.insert("ISADDABLE".into(), json!(v)); }
        if let Some(v) = &self.as_original { m.insert("ASORIGINAL".into(), json!(v)); }

        // HSN
        if self.hsn_applicable_from.is_some() || self.hsn_code.is_some() || self.hsn_description.is_some() || self.hsn_classification_name.is_some() || self.hsn_source_of_details.is_some() {
            let mut hsn = serde_json::Map::new();
            hsn.insert("APPLICABLEFROM".into(), json!(self.hsn_applicable_from.clone().unwrap_or_else(|| "20250401".into())));
            if let Some(v) = &self.hsn_code { hsn.insert("HSNCODE".into(), json!(v)); }
            if let Some(v) = &self.hsn_description { hsn.insert("HSN".into(), json!(v)); }
            if let Some(v) = &self.hsn_source_of_details { hsn.insert("SRCOFHSNDETAILS".into(), json!(v)); }
            if let Some(v) = &self.hsn_classification_name { hsn.insert("HSNCLASSIFICATIONNAME".into(), json!(v)); }
            m.insert("HSNDETAILS.LIST".into(), Value::Object(hsn));
        }

        // GST
        if self.gst_applicable_from.is_some() || self.gst_taxability.is_some() || self.gst_source_of_details.is_some() || self.gst_classification_name.is_some() || self.gst_state_name.is_some() || self.gst_rate_duty_head.is_some() || self.gst_rate_valuation_type.is_some() || self.gst_rate.is_some() {
            let mut gst = serde_json::Map::new();
            gst.insert("APPLICABLEFROM".into(), json!(self.gst_applicable_from.clone().unwrap_or_else(|| "20250401".into())));
            if let Some(v) = &self.gst_taxability { gst.insert("TAXABILITY".into(), json!(v)); }
            if let Some(v) = &self.gst_source_of_details { gst.insert("SRCOFGSTDETAILS".into(), json!(v)); }
            if let Some(v) = &self.gst_classification_name { gst.insert("HSNMASTERNAME".into(), json!(v)); }
            if self.gst_state_name.is_some() || self.gst_rate_duty_head.is_some() || self.gst_rate_valuation_type.is_some() || self.gst_rate.is_some() {
                let mut state = serde_json::Map::new();
                state.insert("STATENAME".into(), json!(self.gst_state_name.clone().unwrap_or_else(|| "&#4; Any".into())));
                if self.gst_rate_duty_head.is_some() || self.gst_rate_valuation_type.is_some() || self.gst_rate.is_some() {
                    let mut rate = serde_json::Map::new();
                    if let Some(v) = &self.gst_rate_duty_head { rate.insert("GSTRATEDUTYHEAD".into(), json!(v)); }
                    if let Some(v) = &self.gst_rate_valuation_type { rate.insert("GSTRATEVALUATIONTYPE".into(), json!(v)); }
                    if let Some(v) = self.gst_rate { rate.insert("GSTRATE".into(), json!(v.to_string())); }
                    state.insert("RATEDETAILS.LIST".into(), serde_json::Value::Object(rate));
                }
                gst.insert("STATEWISEDETAILS.LIST".into(), serde_json::Value::Object(state));
            }
            m.insert("GSTDETAILS.LIST".into(), serde_json::Value::Object(gst));
        }
        m
    }
}
