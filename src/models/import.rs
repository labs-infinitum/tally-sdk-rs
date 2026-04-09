#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ImportResult {
    pub created: i64,
    pub altered: i64,
    pub deleted: i64,
    pub combined: i64,
    pub ignored: i64,
    pub errors: i64,
    pub cancelled: i64,
    pub exceptions: i64,
    pub last_voucher_id: Option<String>,
    pub last_master_id: Option<String>,
    pub line_errors: Vec<String>,
}

impl ImportResult {
    pub fn has_errors(&self) -> bool {
        self.errors > 0 || self.exceptions > 0 || !self.line_errors.is_empty()
    }

    pub fn created_or_altered(&self) -> bool {
        self.created > 0 || self.altered > 0
    }
}
