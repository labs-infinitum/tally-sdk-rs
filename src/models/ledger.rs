use crate::errors::{Result, TallyError};

#[derive(Debug, Clone)]
pub struct Ledger {
    pub name: String,
    pub parent: Option<String>,
    pub alias: Option<Vec<String>>,

    pub opening_balance: Option<f64>,

    // Mailing
    pub mailing_name: Option<String>,
    pub mailing_address: Option<Vec<String>>, // ADDRESS.LIST -> [{ ADDRESS: line }]
    pub mailing_state: Option<String>,
    pub mailing_country: Option<String>,
    pub mailing_pincode: Option<String>,

    // Tax/PAN
    pub income_tax_number: Option<String>,

    // GST top-level
    pub gst_applicable: Option<String>,
    pub appropriate_for: Option<String>,
    pub gst_appropriate_to: Option<String>,
    pub excise_alloc_type: Option<String>,
    pub gst_type_of_supply: Option<String>,
    pub gst_duty_head: Option<String>,
    pub rate_of_tax_calculation: Option<f64>,
    pub tax_type: Option<String>,
    pub bill_credit_period_days: Option<u32>,
    pub is_billwise_on: Option<String>,
    pub is_credit_days_chk_on: Option<String>,

    // Bank
    pub account_number: Option<String>,
    pub ifsc_code: Option<String>,
    pub bank_name: Option<String>,
    pub bank_account_holder_name: Option<String>,
    pub swift_code: Option<String>,
    pub branch_name: Option<String>,
    pub bank_bsr_code: Option<String>,
    pub od_limit: Option<f64>,

    // Payment
    pub default_transaction_type: Option<String>,
    pub payment_favouring: Option<String>,
    pub transaction_name: Option<String>,
    pub set_as_default: Option<String>,
    pub cheque_cross_comment: Option<String>,
    pub virtual_payment_address: Option<String>,
    pub beneficiary_code: Option<String>,

    // TDS
    pub is_tds_applicable: Option<String>,
    pub tds_deductee_type: Option<String>,
    pub deduct_tds_in_same_voucher: Option<String>,
    pub tds_applicable: Option<String>,
    pub tds_category_date: Option<String>,
    pub tds_category_name: Option<String>,

    // HSN
    pub hsn_applicable_from: Option<String>,
    pub hsn_code: Option<String>,
    pub hsn_description: Option<String>,
    pub hsn_classification_name: Option<String>,
    pub hsn_source_of_details: Option<String>,

    // GST details nested
    pub gst_applicable_from: Option<String>,
    pub gst_taxability: Option<String>,
    pub gst_source_of_details: Option<String>,
    pub gst_classification_name: Option<String>,
    pub gst_state_name: Option<String>,
    pub gst_rate_duty_head: Option<String>,
    pub gst_rate_valuation_type: Option<String>,
    pub gst_rate: Option<f64>,
}

impl Ledger {
    pub fn validate(&self) -> Result<()> {
        if self.name.trim().is_empty() {
            return Err(TallyError::Validation("Ledger name is required".into()));
        }
        if let Some(v) = &self.is_billwise_on {
            if v != "Yes" && v != "No" {
                return Err(TallyError::Validation(
                    "is_billwise_on must be 'Yes' or 'No'".into(),
                ));
            }
        }
        if let Some(v) = &self.is_credit_days_chk_on {
            if v != "Yes" && v != "No" {
                return Err(TallyError::Validation(
                    "is_credit_days_chk_on must be 'Yes' or 'No'".into(),
                ));
            }
        }
        if let Some(v) = &self.gst_applicable {
            if v != "Applicable" && v != "Not Applicable" {
                return Err(TallyError::Validation(
                    "GSTAPPLICABLE must be 'Applicable' or 'Not Applicable'".into(),
                ));
            }
        }
        if let Some(v) = &self.tds_applicable {
            if v != "Applicable" && v != "Not Applicable" {
                return Err(TallyError::Validation(
                    "TDSAPPLICABLE must be 'Applicable' or 'Not Applicable'".into(),
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
        if let Some(v) = self.opening_balance {
            m.insert("OPENINGBALANCE".into(), json!(v));
        }
        if let Some(v) = &self.income_tax_number {
            m.insert("INCOMETAXNUMBER".into(), json!(v));
        }

        // top-level GST
        if let Some(v) = &self.gst_applicable {
            m.insert("GSTAPPLICABLE".into(), json!(v));
        }
        if let Some(v) = &self.appropriate_for {
            m.insert("APPROPRIATEFOR".into(), json!(v));
        }
        if let Some(v) = &self.gst_appropriate_to {
            m.insert("GSTAPPROPRIATETO".into(), json!(v));
        }
        if let Some(v) = &self.excise_alloc_type {
            m.insert("EXCISEALLOCTYPE".into(), json!(v));
        }
        if let Some(v) = &self.gst_type_of_supply {
            m.insert("GSTTYPEOFSUPPLY".into(), json!(v));
        }
        if let Some(v) = &self.gst_duty_head {
            m.insert("GSTDUTYHEAD".into(), json!(v));
        }
        if let Some(v) = self.rate_of_tax_calculation {
            m.insert("RATEOFTAXCALCULATION".into(), json!(format!(" {}", v)));
        }
        if let Some(v) = &self.tax_type {
            m.insert("TAXTYPE".into(), json!(v));
        }

        if let Some(days) = self.bill_credit_period_days {
            m.insert("BILLCREDITPERIOD".into(), json!(format!("{} Days", days)));
        }
        if let Some(v) = &self.is_billwise_on {
            m.insert("ISBILLWISEON".into(), json!(v));
        }
        if let Some(v) = &self.is_credit_days_chk_on {
            m.insert("ISCREDITDAYSCHKON".into(), json!(v));
        }

        // mailing block
        if self.mailing_name.is_some()
            || self.mailing_address.is_some()
            || self.mailing_state.is_some()
            || self.mailing_country.is_some()
            || self.mailing_pincode.is_some()
        {
            let mut mailing = serde_json::Map::new();
            mailing.insert("APPLICABLEFROM".into(), json!("20250401"));
            mailing.insert(
                "MAILINGNAME".into(),
                json!(self
                    .mailing_name
                    .clone()
                    .unwrap_or_else(|| self.name.clone())),
            );
            if let Some(addr) = &self.mailing_address {
                let arr: Vec<Value> = addr.iter().map(|line| json!({"ADDRESS": line})).collect();
                mailing.insert("ADDRESS.LIST".into(), Value::Array(arr));
            }
            mailing.insert(
                "COUNTRY".into(),
                json!(self
                    .mailing_country
                    .clone()
                    .unwrap_or_else(|| "India".into())),
            );
            if let Some(v) = &self.mailing_state {
                mailing.insert("STATE".into(), json!(v));
            }
            if let Some(v) = &self.mailing_pincode {
                mailing.insert("PINCODE".into(), json!(v));
            }
            m.insert("LEDMAILINGDETAILS.LIST".into(), Value::Object(mailing));
        }

        // Bank top-level fields
        if let Some(v) = &self.account_number {
            m.insert("BANKDETAILS".into(), json!(v));
        }
        if let Some(v) = &self.ifsc_code {
            m.insert("IFSCODE".into(), json!(v));
        }
        if let Some(v) = &self.bank_account_holder_name {
            m.insert("BANKACCHOLDERNAME".into(), json!(v));
        }
        if let Some(v) = &self.swift_code {
            m.insert("SWIFTCODE".into(), json!(v));
        }
        if let Some(v) = &self.branch_name {
            m.insert("BRANCHNAME".into(), json!(v));
        }
        if let Some(v) = &self.bank_bsr_code {
            m.insert("BANKBSRCODE".into(), json!(v));
        }
        if let Some(v) = self.od_limit {
            m.insert("ODLIMIT".into(), json!(v.to_string()));
        }

        // Payment details
        if self.default_transaction_type.is_some()
            || self.payment_favouring.is_some()
            || self.cheque_cross_comment.is_some()
            || self.virtual_payment_address.is_some()
            || self.beneficiary_code.is_some()
            || self.account_number.is_some()
            || self.ifsc_code.is_some()
            || self.bank_name.is_some()
        {
            let mut pay = serde_json::Map::new();
            pay.insert(
                "PAYMENTFAVOURING".into(),
                json!(self
                    .payment_favouring
                    .clone()
                    .unwrap_or_else(|| self.name.clone())),
            );
            pay.insert(
                "TRANSACTIONNAME".into(),
                json!(self
                    .transaction_name
                    .clone()
                    .unwrap_or_else(|| "Primary".into())),
            );
            pay.insert(
                "SETASDEFAULT".into(),
                json!(self.set_as_default.clone().unwrap_or_else(|| "No".into())),
            );
            if let Some(v) = &self.default_transaction_type {
                pay.insert("DEFAULTTRANSACTIONTYPE".into(), json!(v));
            }
            if let Some(v) = &self.cheque_cross_comment {
                pay.insert("CHEQUECROSSCOMMENT".into(), json!(v));
            }
            if let Some(v) = &self.virtual_payment_address {
                pay.insert("VIRTUALPAYMENTADDRESS".into(), json!(v));
            }
            if let Some(v) = &self.ifsc_code {
                pay.insert("IFSCODE".into(), json!(v));
            }
            if let Some(v) = &self.bank_name {
                pay.insert("BANKNAME".into(), json!(v));
            }
            if let Some(v) = &self.account_number {
                pay.insert("ACCOUNTNUMBER".into(), json!(v));
            }
            if let Some(code) = &self.beneficiary_code {
                let mut ben = serde_json::Map::new();
                ben.insert("BENEFICIARYCODE".into(), json!(code));
                pay.insert(
                    "BENEFICIARYCODEDETAILS".into(),
                    serde_json::Value::Object(ben),
                );
            }
            m.insert("PAYMENTDETAILS".into(), serde_json::Value::Object(pay));
        }

        // TDS block/flags
        if let Some(v) = &self.is_tds_applicable {
            m.insert("ISTDSAPPLICABLE".into(), json!(v));
        }
        if let Some(v) = &self.tds_deductee_type {
            m.insert("TDSDEDUCTEETYPE".into(), json!(v));
        }
        if let Some(v) = &self.deduct_tds_in_same_voucher {
            m.insert("DEDUCTINSAMEVCH".into(), json!(v));
        }
        if let Some(v) = &self.tds_applicable {
            m.insert("TDSAPPLICABLE".into(), json!(v));
        }
        if self.tds_category_name.is_some() {
            let mut tds = serde_json::Map::new();
            tds.insert(
                "CATEGORYDATE".into(),
                json!(self
                    .tds_category_date
                    .clone()
                    .unwrap_or_else(|| "20250401".into())),
            );
            tds.insert(
                "CATEGORYNAME".into(),
                json!(self.tds_category_name.clone().unwrap()),
            );
            m.insert(
                "TDSCATEGORYDETAILS.LIST".into(),
                serde_json::Value::Object(tds),
            );
        }

        // HSN nested
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

        // GST nested
        if self.gst_applicable_from.is_some()
            || self.gst_taxability.is_some()
            || self.gst_source_of_details.is_some()
            || self.gst_rate_duty_head.is_some()
            || self.gst_rate_valuation_type.is_some()
            || self.gst_rate.is_some()
            || self.gst_state_name.is_some()
            || self.gst_classification_name.is_some()
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
