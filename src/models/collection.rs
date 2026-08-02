#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerSummary {
    pub name: String,
    pub parent: Option<String>,
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
