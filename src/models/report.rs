#[derive(Debug, Clone)]
pub struct TrialBalanceEntry {
    pub name: String,
    pub debit: Option<f64>,
    pub credit: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct BalanceSheetEntry {
    pub name: String,
    pub main_amount: Option<f64>,
    pub sub_amount: Option<f64>,
}
