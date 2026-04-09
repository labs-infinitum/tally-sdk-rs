use serde_json::{json, Map, Value};

const DEFAULT_APPLICABLE_FROM: &str = "20250401";
const DEFAULT_ANY_STATE: &str = "&#4; Any";

pub(crate) fn build_hsn_details(
    applicable_from: Option<&String>,
    source_of_details: Option<&String>,
    code: Option<&String>,
    description: Option<&String>,
    classification_name: Option<&String>,
    restrict_by_source: bool,
) -> Option<Map<String, Value>> {
    if applicable_from.is_none()
        && source_of_details.is_none()
        && code.is_none()
        && description.is_none()
        && classification_name.is_none()
    {
        return None;
    }

    let mut hsn = Map::new();
    hsn.insert(
        "APPLICABLEFROM".into(),
        json!(applicable_from
            .cloned()
            .unwrap_or_else(|| DEFAULT_APPLICABLE_FROM.into())),
    );

    if let Some(source) = source_of_details {
        hsn.insert("SRCOFHSNDETAILS".into(), json!(source));
    }

    let include_details = !restrict_by_source
        || matches!(
            source_of_details.map(|value| value.as_str()),
            Some("Specify Details Here")
        );
    if include_details {
        if let Some(value) = code {
            hsn.insert("HSNCODE".into(), json!(value));
        }
        if let Some(value) = description {
            hsn.insert("HSN".into(), json!(value));
        }
    }

    let include_classification = !restrict_by_source
        || matches!(
            source_of_details.map(|value| value.as_str()),
            Some("Use GST Classification")
        );
    if include_classification {
        if let Some(value) = classification_name {
            hsn.insert("HSNCLASSIFICATIONNAME".into(), json!(value));
        }
    }

    Some(hsn)
}

pub(crate) fn build_gst_details(
    applicable_from: Option<&String>,
    taxability: Option<&String>,
    source_of_details: Option<&String>,
    classification_name: Option<&String>,
    state_name: Option<&String>,
    rate_duty_head: Option<&String>,
    rate_valuation_type: Option<&String>,
    rate: Option<f64>,
    restrict_classification_by_source: bool,
    default_state_name: bool,
) -> Option<Map<String, Value>> {
    if applicable_from.is_none()
        && taxability.is_none()
        && source_of_details.is_none()
        && classification_name.is_none()
        && state_name.is_none()
        && rate_duty_head.is_none()
        && rate_valuation_type.is_none()
        && rate.is_none()
    {
        return None;
    }

    let mut gst = Map::new();
    gst.insert(
        "APPLICABLEFROM".into(),
        json!(applicable_from
            .cloned()
            .unwrap_or_else(|| DEFAULT_APPLICABLE_FROM.into())),
    );

    if let Some(value) = taxability {
        gst.insert("TAXABILITY".into(), json!(value));
    }
    if let Some(value) = source_of_details {
        gst.insert("SRCOFGSTDETAILS".into(), json!(value));
    }

    let include_classification = !restrict_classification_by_source
        || matches!(
            source_of_details.map(|value| value.as_str()),
            Some("Use GST Classification")
        );
    if include_classification {
        if let Some(value) = classification_name {
            gst.insert("HSNMASTERNAME".into(), json!(value));
        }
    }

    if let Some(state) = build_statewise_details(
        state_name,
        rate_duty_head,
        rate_valuation_type,
        rate,
        default_state_name,
    ) {
        gst.insert("STATEWISEDETAILS.LIST".into(), Value::Object(state));
    }

    Some(gst)
}

fn build_statewise_details(
    state_name: Option<&String>,
    rate_duty_head: Option<&String>,
    rate_valuation_type: Option<&String>,
    rate: Option<f64>,
    default_state_name: bool,
) -> Option<Map<String, Value>> {
    if state_name.is_none()
        && rate_duty_head.is_none()
        && rate_valuation_type.is_none()
        && rate.is_none()
        && !default_state_name
    {
        return None;
    }

    let mut state = Map::new();
    if let Some(value) = state_name {
        state.insert("STATENAME".into(), json!(value));
    } else if default_state_name {
        state.insert("STATENAME".into(), json!(DEFAULT_ANY_STATE));
    }

    if let Some(rate_details) = build_rate_details(rate_duty_head, rate_valuation_type, rate) {
        state.insert("RATEDETAILS.LIST".into(), Value::Object(rate_details));
    }

    Some(state)
}

fn build_rate_details(
    duty_head: Option<&String>,
    valuation_type: Option<&String>,
    rate: Option<f64>,
) -> Option<Map<String, Value>> {
    if duty_head.is_none() && valuation_type.is_none() && rate.is_none() {
        return None;
    }

    let mut rate_details = Map::new();
    if let Some(value) = duty_head {
        rate_details.insert("GSTRATEDUTYHEAD".into(), json!(value));
    }
    if let Some(value) = valuation_type {
        rate_details.insert("GSTRATEVALUATIONTYPE".into(), json!(value));
    }
    if let Some(value) = rate {
        rate_details.insert("GSTRATE".into(), json!(value.to_string()));
    }

    Some(rate_details)
}
