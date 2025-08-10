use crate::errors::{Result, TallyError};

#[derive(Debug, Clone)]
pub struct StockItem {
    pub name: String,
    pub parent: Option<String>,
    pub alias: Option<Vec<String>>,
    // Units
    pub base_units: Option<String>,
    pub additional_units: Option<String>,
    // Statutory
    pub gst_applicable: Option<String>,
    pub gst_type_of_supply: Option<String>,
    pub basic_rate_of_excise: Option<f64>,
    pub opening_balance: Option<f64>,

    // HSN/GST
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

impl StockItem {
    pub fn validate(&self) -> Result<()> {
        if self.name.trim().is_empty() { return Err(TallyError::Validation("Stock Item name is required".into())); }
        if let Some(v) = &self.gst_applicable { if v != "Applicable" && v != "Not Applicable" { return Err(TallyError::Validation("GSTAPPLICABLE must be 'Applicable' or 'Not Applicable'".into())); } }
        if let Some(v) = &self.gst_type_of_supply { let ok = ["Capital Goods","Goods","Services"]; if !ok.contains(&v.as_str()) { return Err(TallyError::Validation("Invalid GST Type of Supply".into())); } }
        Ok(())
    }

    pub fn to_map(&self) -> serde_json::Map<String, serde_json::Value> {
        use serde_json::{json, Value};
        let mut m = serde_json::Map::new();
        m.insert("NAME".into(), json!(self.name));
        if let Some(v) = &self.parent { m.insert("PARENT".into(), json!(v)); } else { m.insert("PARENT".into(), Value::Null); }
        if let Some(v) = &self.alias { m.insert("ALIAS".into(), json!(v)); }
        if let Some(v) = &self.base_units { m.insert("BASEUNITS".into(), json!(v)); }
        if let Some(v) = &self.additional_units { m.insert("ADDITIONALUNITS".into(), json!(v)); }
        if let Some(v) = &self.gst_applicable { m.insert("GSTAPPLICABLE".into(), json!(v)); }
        if let Some(v) = &self.gst_type_of_supply { m.insert("GSTTYPEOFSUPPLY".into(), json!(v)); }
        if let Some(v) = self.basic_rate_of_excise { m.insert("BASICRATEOFEXCISE".into(), json!(format!(" {}", v))); }
        if let Some(v) = self.opening_balance { m.insert("OPENINGBALANCE".into(), json!(format!(" {}", v))); }

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
