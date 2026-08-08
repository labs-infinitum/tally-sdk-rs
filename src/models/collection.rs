#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerSummary {
    pub name: String,
    pub parent: Option<String>,
}

/// Rich ledger master exported from Tally for migration / sync.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LedgerDetails {
    pub name: String,
    pub parent: Option<String>,
    pub guid: Option<String>,
    pub mailing_name: Option<String>,
    pub address: Vec<String>,
    pub state_name: Option<String>,
    pub country_name: Option<String>,
    pub pincode: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub income_tax_number: Option<String>,
    pub party_gstin: Option<String>,
    pub gst_registration_type: Option<String>,
    pub opening_balance: Option<f64>,
    pub is_billwise_on: Option<bool>,
    pub bill_credit_period: Option<String>,
    pub account_number: Option<String>,
    pub ifsc_code: Option<String>,
    pub bank_name: Option<String>,
    pub bank_account_holder: Option<String>,
    pub swift_code: Option<String>,
    pub branch_name: Option<String>,
    pub currency_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupSummary {
    pub name: String,
    pub parent: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StockItemSummary {
    pub name: String,
    pub parent: Option<String>,
}

/// Rich stock item master exported from Tally for billing-offer sync.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StockItemDetails {
    pub name: String,
    pub parent: Option<String>,
    pub guid: Option<String>,
    pub base_units: Option<String>,
    pub additional_units: Option<String>,
    pub gst_applicable: Option<String>,
    pub gst_type_of_supply: Option<String>,
    pub hsn_code: Option<String>,
    pub hsn_description: Option<String>,
    pub gst_taxability: Option<String>,
    pub gst_rate: Option<f64>,
    pub opening_rate: Option<f64>,
    pub opening_balance: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CurrencySummary {
    /// Currency symbol as shown in Tally books (NAME), e.g. `D$`, `EUR`, `?`.
    pub name: String,
    /// Underlying/original symbol (ORIGINALNAME), e.g. `$`, `EUR`, `?`.
    pub original_name: Option<String>,
    /// Formal mailing name, e.g. `Dollar`, `EURO`, `INR`.
    pub mailing_name: Option<String>,
    /// Expanded symbol/name, e.g. `Dollar`, `EURO`, `INR`.
    pub expanded_symbol: Option<String>,
    /// Minor unit name, e.g. `Cent`.
    pub decimal_symbol: Option<String>,
    pub decimal_places: Option<i32>,
    pub decimal_places_for_printing: Option<i32>,
    pub is_suffix: Option<bool>,
    pub has_space: Option<bool>,
    pub in_millions: Option<bool>,
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

    pub fn matches_symbol(&self, symbol: &str) -> bool {
        self.name.eq_ignore_ascii_case(symbol)
            || self
                .original_name
                .as_deref()
                .is_some_and(|original| original.eq_ignore_ascii_case(symbol))
    }
}
