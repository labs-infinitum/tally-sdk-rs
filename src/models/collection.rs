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
