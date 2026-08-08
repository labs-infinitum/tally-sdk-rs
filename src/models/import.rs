//! Result of a Tally import/create request.

/// Counters and diagnostics returned after importing masters or vouchers.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ImportResult {
    /// Number of objects created.
    pub created: i64,
    /// Number of objects altered.
    pub altered: i64,
    /// Number of objects deleted.
    pub deleted: i64,
    /// Number of objects combined.
    pub combined: i64,
    /// Number of objects ignored.
    pub ignored: i64,
    /// Error count reported by Tally.
    pub errors: i64,
    /// Cancelled count reported by Tally.
    pub cancelled: i64,
    /// Exception count reported by Tally.
    pub exceptions: i64,
    /// Last voucher id touched, when present.
    pub last_voucher_id: Option<String>,
    /// Last master id touched, when present.
    pub last_master_id: Option<String>,
    /// Per-line error messages from the response.
    pub line_errors: Vec<String>,
}

impl ImportResult {
    /// `true` when Tally reported errors, exceptions, or line errors.
    pub fn has_errors(&self) -> bool {
        self.errors > 0 || self.exceptions > 0 || !self.line_errors.is_empty()
    }

    /// `true` when at least one object was created or altered.
    pub fn created_or_altered(&self) -> bool {
        self.created > 0 || self.altered > 0
    }
}
