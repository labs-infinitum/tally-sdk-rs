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
    pub name: String,
}
