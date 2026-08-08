//! Company master details exported from Tally.

/// Detailed company master information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompanyDetails {
    /// Company name.
    pub name: String,
    /// Formal / mailing name.
    pub formal_name: Option<String>,
    /// Tally GUID.
    pub guid: Option<String>,
    /// Company number.
    pub company_number: Option<String>,
    /// Financial year start date.
    pub starting_from: Option<String>,
    /// Books-from date.
    pub books_from: Option<String>,
    /// Audited up to date.
    pub audited_upto: Option<String>,
    /// Base currency original name/symbol from Tally (`CURRENCYNAME`), e.g. `$` or `₹`.
    pub currency_name: Option<String>,
    /// Company email.
    pub email: Option<String>,
    /// Company website.
    pub website: Option<String>,
    /// Phone number.
    pub phone_number: Option<String>,
    /// Fax number.
    pub fax_number: Option<String>,
    /// Address lines.
    pub address: Vec<String>,
    /// State name.
    pub state_name: Option<String>,
    /// Country name.
    pub country_name: Option<String>,
    /// Country ISD code.
    pub country_isd_code: Option<String>,
    /// PIN / ZIP code.
    pub pincode: Option<String>,
    /// GSTIN / GST registration number.
    pub gst_registration_number: Option<String>,
    /// GST registration type (for example Regular).
    pub gst_registration_type: Option<String>,
    /// PAN / income tax number.
    pub income_tax_number: Option<String>,
    /// Whether GST features are enabled.
    pub is_gst_on: Option<bool>,
    /// Whether accounting features are enabled.
    pub is_accounting_on: Option<bool>,
    /// Whether inventory features are enabled.
    pub is_inventory_on: Option<bool>,
    /// Whether bill-wise details are enabled.
    pub is_bill_wise_on: Option<bool>,
    /// Whether payroll features are enabled.
    pub is_payroll_on: Option<bool>,
    /// Whether security features are enabled.
    pub is_security_on: Option<bool>,
    /// Whether invoicing features are enabled.
    pub is_invoicing_on: Option<bool>,
    /// Whether multi-currency is enabled.
    pub is_multi_currency_on: Option<bool>,
}
