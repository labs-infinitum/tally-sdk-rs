use crate::errors::{Result, TallyError};

#[derive(Debug, Clone)]
pub struct Group {
    pub name: String,
    pub parent: Option<String>,
    pub group_type: Option<String>,
    pub alias: Option<Vec<String>>,

    pub basic_group_is_calculable: Option<String>,
    pub is_addable: Option<String>,
    pub is_subledger: Option<String>,
    pub addl_alloc_type: Option<String>,
    pub as_original: Option<String>,
    pub affects_gross_profit: Option<String>,

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
    pub gst_rate_duty_head: Option<String>,
    pub gst_rate_valuation_type: Option<String>,
    pub gst_rate: Option<f64>,
    pub gst_state_name: Option<String>,
}

impl Group {
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

        // HSN
        if self.hsn_applicable_from.is_some()
            || self.hsn_code.is_some()
            || self.hsn_description.is_some()
            || self.hsn_classification_name.is_some()
            || self.hsn_source_of_details.is_some()
        {
            let mut hsn = serde_json::Map::new();
            hsn.insert(
                "APPLICABLEFROM".into(),
                json!(self
                    .hsn_applicable_from
                    .clone()
                    .unwrap_or_else(|| "20250401".into())),
            );
            if let Some(v) = &self.hsn_source_of_details {
                hsn.insert("SRCOFHSNDETAILS".into(), json!(v));
            }
            if self.hsn_source_of_details.as_deref() == Some("Specify Details Here") {
                if let Some(v) = &self.hsn_code {
                    hsn.insert("HSNCODE".into(), json!(v));
                }
                if let Some(v) = &self.hsn_description {
                    hsn.insert("HSN".into(), json!(v));
                }
            }
            if self.hsn_source_of_details.as_deref() == Some("Use GST Classification") {
                if let Some(v) = &self.hsn_classification_name {
                    hsn.insert("HSNCLASSIFICATIONNAME".into(), json!(v));
                }
            }
            m.insert("HSNDETAILS.LIST".into(), serde_json::Value::Object(hsn));
        }

        // GST
        if self.gst_applicable_from.is_some()
            || self.gst_taxability.is_some()
            || self.gst_source_of_details.is_some()
            || self.gst_classification_name.is_some()
            || self.gst_state_name.is_some()
            || self.gst_rate_duty_head.is_some()
            || self.gst_rate_valuation_type.is_some()
            || self.gst_rate.is_some()
        {
            let mut gst = serde_json::Map::new();
            gst.insert(
                "APPLICABLEFROM".into(),
                json!(self
                    .gst_applicable_from
                    .clone()
                    .unwrap_or_else(|| "20250401".into())),
            );
            if let Some(v) = &self.gst_taxability {
                gst.insert("TAXABILITY".into(), json!(v));
            }
            if let Some(v) = &self.gst_source_of_details {
                gst.insert("SRCOFGSTDETAILS".into(), json!(v));
            }
            if self.gst_source_of_details.as_deref() == Some("Use GST Classification") {
                if let Some(v) = &self.gst_classification_name {
                    gst.insert("HSNMASTERNAME".into(), json!(v));
                }
            }
            if self.gst_state_name.is_some()
                || self.gst_rate_duty_head.is_some()
                || self.gst_rate_valuation_type.is_some()
                || self.gst_rate.is_some()
            {
                let mut state = serde_json::Map::new();
                if let Some(v) = &self.gst_state_name {
                    state.insert("STATENAME".into(), json!(v));
                } else {
                    state.insert("STATENAME".into(), json!("&#4; Any"));
                }
                if self.gst_rate_duty_head.is_some()
                    || self.gst_rate_valuation_type.is_some()
                    || self.gst_rate.is_some()
                {
                    let mut rate = serde_json::Map::new();
                    if let Some(v) = &self.gst_rate_duty_head {
                        rate.insert("GSTRATEDUTYHEAD".into(), json!(v));
                    }
                    if let Some(v) = &self.gst_rate_valuation_type {
                        rate.insert("GSTRATEVALUATIONTYPE".into(), json!(v));
                    }
                    if let Some(v) = self.gst_rate {
                        rate.insert("GSTRATE".into(), json!(v.to_string()));
                    }
                    state.insert("RATEDETAILS.LIST".into(), serde_json::Value::Object(rate));
                }
                gst.insert(
                    "STATEWISEDETAILS.LIST".into(),
                    serde_json::Value::Object(state),
                );
            }
            m.insert("GSTDETAILS.LIST".into(), serde_json::Value::Object(gst));
        }

        m
    }
}
