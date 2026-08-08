//! Lightweight collection summaries and rich master exports.

/// Ledger name/parent summary from a collection export.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerSummary {
    /// Ledger name.
    pub name: String,
    /// Parent group name when available.
    pub parent: Option<String>,
}

/// Rich ledger master exported from Tally for migration / sync.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LedgerDetails {
    /// Ledger name.
    pub name: String,
    /// Parent group.
    pub parent: Option<String>,
    /// Tally GUID.
    pub guid: Option<String>,
    /// Mailing name.
    pub mailing_name: Option<String>,
    /// Address lines.
    pub address: Vec<String>,
    /// State.
    pub state_name: Option<String>,
    /// Country.
    pub country_name: Option<String>,
    /// PIN code.
    pub pincode: Option<String>,
    /// Email.
    pub email: Option<String>,
    /// Phone.
    pub phone: Option<String>,
    /// PAN / income tax number.
    pub income_tax_number: Option<String>,
    /// Party GSTIN.
    pub party_gstin: Option<String>,
    /// GST registration type.
    pub gst_registration_type: Option<String>,
    /// Opening balance.
    pub opening_balance: Option<f64>,
    /// Whether bill-wise is on.
    pub is_billwise_on: Option<bool>,
    /// Bill credit period.
    pub bill_credit_period: Option<String>,
    /// Bank account number.
    pub account_number: Option<String>,
    /// IFSC code.
    pub ifsc_code: Option<String>,
    /// Bank name.
    pub bank_name: Option<String>,
    /// Account holder name.
    pub bank_account_holder: Option<String>,
    /// SWIFT code.
    pub swift_code: Option<String>,
    /// Branch name.
    pub branch_name: Option<String>,
    /// Currency name/symbol.
    pub currency_name: Option<String>,
}

/// Group name/parent summary from a collection export.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupSummary {
    /// Group name.
    pub name: String,
    /// Parent group when available.
    pub parent: Option<String>,
}

/// Stock item name/parent summary from a collection export.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StockItemSummary {
    /// Stock item name.
    pub name: String,
    /// Parent stock group when available.
    pub parent: Option<String>,
}

/// Rich stock item master exported from Tally for billing-offer sync.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StockItemDetails {
    /// Stock item name.
    pub name: String,
    /// Parent stock group.
    pub parent: Option<String>,
    /// Tally GUID.
    pub guid: Option<String>,
    /// Base unit.
    pub base_units: Option<String>,
    /// Additional unit.
    pub additional_units: Option<String>,
    /// GST applicability flag.
    pub gst_applicable: Option<String>,
    /// GST type of supply.
    pub gst_type_of_supply: Option<String>,
    /// HSN/SAC code.
    pub hsn_code: Option<String>,
    /// HSN/SAC description.
    pub hsn_description: Option<String>,
    /// GST taxability.
    pub gst_taxability: Option<String>,
    /// GST rate percent.
    pub gst_rate: Option<f64>,
    /// Opening rate.
    pub opening_rate: Option<f64>,
    /// Opening balance quantity/value depending on export.
    pub opening_balance: Option<f64>,
}

/// Currency master summary.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CurrencySummary {
    /// Currency symbol as shown in Tally books (NAME), e.g. `D$`, `EUR`, `₹`.
    pub name: String,
    /// Underlying/original symbol (ORIGINALNAME), e.g. `$`, `EUR`, `₹`.
    pub original_name: Option<String>,
    /// Formal mailing name, e.g. `Dollar`, `EURO`, `INR`.
    pub mailing_name: Option<String>,
    /// Expanded symbol/name, e.g. `Dollar`, `EURO`, `INR`.
    pub expanded_symbol: Option<String>,
    /// Minor unit name, e.g. `Cent`.
    pub decimal_symbol: Option<String>,
    /// Decimal places used in calculations.
    pub decimal_places: Option<i32>,
    /// Decimal places used when printing.
    pub decimal_places_for_printing: Option<i32>,
    /// Whether the symbol is shown as a suffix.
    pub is_suffix: Option<bool>,
    /// Whether a space is printed between amount and symbol.
    pub has_space: Option<bool>,
    /// Whether amounts use million format.
    pub in_millions: Option<bool>,
    /// Tally GUID.
    pub guid: Option<String>,
}

impl CurrencySummary {
    /// Prefer mailing/expanded name, otherwise the Tally symbol.
    pub fn display_name(&self) -> &str {
        self.mailing_name
            .as_deref()
            .or(self.expanded_symbol.as_deref())
            .unwrap_or(self.name.as_str())
    }

    /// `true` when `symbol` matches [`Self::name`] or [`Self::original_name`].
    pub fn matches_symbol(&self, symbol: &str) -> bool {
        self.name.eq_ignore_ascii_case(symbol)
            || self
                .original_name
                .as_deref()
                .is_some_and(|original| original.eq_ignore_ascii_case(symbol))
    }
}
