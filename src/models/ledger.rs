use crate::errors::{Result, TallyError};
use crate::models::helpers::{build_gst_details, build_hsn_details};

/// Ledger master used when creating ledgers in Tally.
#[derive(Debug, Clone)]
pub struct Ledger {
    /// Display name.
    pub name: String,
    /// Parent group or stock group name.
    pub parent: Option<String>,
    /// Alternate names / aliases.
    pub alias: Option<Vec<String>>,

    /// Opening balance.
    pub opening_balance: Option<f64>,

    // Mailing
    /// Mailing name.
    pub mailing_name: Option<String>,
    /// Mailing address lines (ADDRESS.LIST).
    pub mailing_address: Option<Vec<String>>, // ADDRESS.LIST -> [{ ADDRESS: line }]
    /// Mailing state.
    pub mailing_state: Option<String>,
    /// Mailing country.
    pub mailing_country: Option<String>,
    /// Mailing PIN / ZIP code.
    pub mailing_pincode: Option<String>,

    // Tax/PAN
    /// PAN / income tax number.
    pub income_tax_number: Option<String>,

    // GST top-level
    /// GST applicability (`Applicable` / `Not Applicable`).
    pub gst_applicable: Option<String>,
    /// GST appropriate-for setting.
    pub appropriate_for: Option<String>,
    /// GST appropriate-to setting.
    pub gst_appropriate_to: Option<String>,
    /// Excise allocation type.
    pub excise_alloc_type: Option<String>,
    /// GST type of supply (Goods, Services, Capital Goods).
    pub gst_type_of_supply: Option<String>,
    /// GST duty head.
    pub gst_duty_head: Option<String>,
    /// Rate of tax calculation percent.
    pub rate_of_tax_calculation: Option<f64>,
    /// Tax type.
    pub tax_type: Option<String>,
    /// Bill credit period in days.
    pub bill_credit_period_days: Option<u32>,
    /// Whether bill-wise details are enabled (`Yes`/`No` or bool).
    pub is_billwise_on: Option<String>,
    /// Whether credit-days check is enabled.
    pub is_credit_days_chk_on: Option<String>,

    // Bank
    /// Bank account number.
    pub account_number: Option<String>,
    /// IFSC code.
    pub ifsc_code: Option<String>,
    /// Bank name.
    pub bank_name: Option<String>,
    /// Bank account holder name.
    pub bank_account_holder_name: Option<String>,
    /// SWIFT code.
    pub swift_code: Option<String>,
    /// Bank branch name.
    pub branch_name: Option<String>,
    /// Bank BSR code.
    pub bank_bsr_code: Option<String>,
    /// Overdraft limit.
    pub od_limit: Option<f64>,

    // Payment
    /// Default bank transaction type.
    pub default_transaction_type: Option<String>,
    /// Payment favouring name.
    pub payment_favouring: Option<String>,
    /// Bank transaction name.
    pub transaction_name: Option<String>,
    /// Whether this is the default payment setup.
    pub set_as_default: Option<String>,
    /// Cheque cross comment.
    pub cheque_cross_comment: Option<String>,
    /// Virtual payment address (VPA).
    pub virtual_payment_address: Option<String>,
    /// Beneficiary code.
    pub beneficiary_code: Option<String>,

    // TDS
    /// Whether TDS is applicable.
    pub is_tds_applicable: Option<String>,
    /// TDS deductee type.
    pub tds_deductee_type: Option<String>,
    /// Whether TDS is deducted in the same voucher.
    pub deduct_tds_in_same_voucher: Option<String>,
    /// TDS applicable flag.
    pub tds_applicable: Option<String>,
    /// TDS category applicable date.
    pub tds_category_date: Option<String>,
    /// TDS category name.
    pub tds_category_name: Option<String>,

    // HSN
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

    // GST details nested
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

impl Ledger {
    /// Validate required fields and basic invariants before import.
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

    /// Convert to the Tally XML field map used by [`crate::xml_builder::XmlBuilder`].
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
                let arr: Vec<Value> = addr.iter().map(|line| json!({ "ADDRESS": line })).collect();
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
            false,
        ) {
            m.insert("GSTDETAILS.LIST".into(), serde_json::Value::Object(gst));
        }

        m
    }
}
