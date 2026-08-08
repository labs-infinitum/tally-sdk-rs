//! Financial report row types.

/// One Trial Balance line (name + closing debit/credit).
#[derive(Debug, Clone)]
pub struct TrialBalanceEntry {
    /// Account or group display name.
    pub name: String,
    /// Closing debit amount when present.
    pub debit: Option<f64>,
    /// Closing credit amount when present.
    pub credit: Option<f64>,
}

/// One Balance Sheet line.
#[derive(Debug, Clone)]
pub struct BalanceSheetEntry {
    /// Account or group display name.
    pub name: String,
    /// Primary amount column from Tally.
    pub main_amount: Option<f64>,
    /// Secondary/sub amount column from Tally.
    pub sub_amount: Option<f64>,
}

/// One Profit and Loss line.
#[derive(Debug, Clone)]
pub struct ProfitAndLossEntry {
    /// Account or group display name.
    pub name: String,
    /// Primary amount column from Tally.
    pub main_amount: Option<f64>,
    /// Secondary/sub amount column from Tally.
    pub sub_amount: Option<f64>,
}
