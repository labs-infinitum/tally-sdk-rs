//! GST / GSTR report models.
//!
//! TallyPrime does not expose a builtin HTTP report named `GSTR-1` on current
//! builds (export returns "Could not find Report 'GSTR-1'"). This crate therefore:
//! - exports the native **GST Computation** report when available
//! - builds a **GSTR-1 style** summary from voucher data for the period

/// One row from Tally's GST Computation report.
#[derive(Debug, Clone, Default)]
pub struct GstComputationEntry {
    /// Row label / particulars from GST Computation.
    pub particulars: String,
    /// Taxable value.
    pub taxable_value: Option<f64>,
    /// IGST amount.
    pub igst: Option<f64>,
    /// CGST amount.
    pub cgst: Option<f64>,
    /// SGST / UTGST amount.
    pub sgst: Option<f64>,
    /// Cess amount.
    pub cess: Option<f64>,
    /// Total tax amount.
    pub total_tax: Option<f64>,
}

/// Parsed GST Computation builtin report.
#[derive(Debug, Clone, Default)]
pub struct GstComputationReport {
    /// Report name field from the export, when present.
    pub name_field: Option<String>,
    /// Report rows or ledger entries.
    pub entries: Vec<GstComputationEntry>,
}

/// How [`Gstr1Report`] was produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gstr1Source {
    /// Aggregated from sales / credit-note / debit-note vouchers.
    VoucherDerived,
}

/// Taxable value plus IGST/CGST/SGST/cess amounts.
#[derive(Debug, Clone, Default)]
pub struct Gstr1TaxBreakup {
    /// Taxable value.
    pub taxable_value: f64,
    /// IGST amount.
    pub igst: f64,
    /// CGST amount.
    pub cgst: f64,
    /// SGST / UTGST amount.
    pub sgst: f64,
    /// Cess amount.
    pub cess: f64,
}

impl Gstr1TaxBreakup {
    /// Sum of IGST + CGST + SGST + cess.
    pub fn total_tax(&self) -> f64 {
        self.igst + self.cgst + self.sgst + self.cess
    }

    /// Taxable value plus total tax.
    pub fn invoice_value(&self) -> f64 {
        self.taxable_value + self.total_tax()
    }
}

/// One B2B invoice row in a voucher-derived GSTR-1.
#[derive(Debug, Clone)]
pub struct Gstr1B2bInvoice {
    /// Party GSTIN.
    pub party_gstin: String,
    /// Party / customer name.
    pub party_name: Option<String>,
    /// Invoice number.
    pub invoice_number: Option<String>,
    /// Invoice date (`YYYYMMDD`).
    pub invoice_date: String,
    /// Place of supply.
    pub place_of_supply: Option<String>,
    /// Voucher type name (for example Sales, Purchase, Payment).
    pub voucher_type: String,
    /// Tax breakup for this row.
    pub taxes: Gstr1TaxBreakup,
}

/// One B2C (large) invoice row in a voucher-derived GSTR-1.
#[derive(Debug, Clone)]
pub struct Gstr1B2cInvoice {
    /// Party / customer name.
    pub party_name: Option<String>,
    /// Invoice number.
    pub invoice_number: Option<String>,
    /// Invoice date (`YYYYMMDD`).
    pub invoice_date: String,
    /// Place of supply.
    pub place_of_supply: Option<String>,
    /// Voucher type name (for example Sales, Purchase, Payment).
    pub voucher_type: String,
    /// Tax breakup for this row.
    pub taxes: Gstr1TaxBreakup,
}

/// Aggregated B2C (small) summary row.
#[derive(Debug, Clone)]
pub struct Gstr1B2cSummary {
    /// Place of supply.
    pub place_of_supply: Option<String>,
    /// GST rate percent for this B2CS aggregation, when known.
    pub rate: Option<f64>,
    /// Tax breakup for this row.
    pub taxes: Gstr1TaxBreakup,
    /// Number of invoices aggregated into this summary.
    pub invoice_count: usize,
}

/// Credit/debit note row for registered parties.
#[derive(Debug, Clone)]
pub struct Gstr1CdnrNote {
    /// Party GSTIN.
    pub party_gstin: Option<String>,
    /// Party / customer name.
    pub party_name: Option<String>,
    /// Credit/debit note number.
    pub note_number: Option<String>,
    /// Credit/debit note date (`YYYYMMDD`).
    pub note_date: String,
    /// Note type (Credit Note / Debit Note).
    pub note_type: String,
    /// Place of supply.
    pub place_of_supply: Option<String>,
    /// Tax breakup for this row.
    pub taxes: Gstr1TaxBreakup,
}

/// HSN-wise summary row.
#[derive(Debug, Clone)]
pub struct Gstr1HsnRow {
    /// HSN/SAC code.
    pub hsn_code: String,
    /// Description text.
    pub description: Option<String>,
    /// Unit quantity code.
    pub uqc: Option<String>,
    /// Total quantity for the HSN row.
    pub total_quantity: Option<f64>,
    /// Tax breakup for this row.
    pub taxes: Gstr1TaxBreakup,
}

/// Document count summary row.
#[derive(Debug, Clone)]
pub struct Gstr1DocumentSummary {
    /// Document type label.
    pub document_type: String,
    /// Document count.
    pub count: usize,
    /// Cancelled document count.
    pub cancelled: usize,
}

/// GSTR-1 style summary derived from vouchers for a period.
#[derive(Debug, Clone)]
pub struct Gstr1Report {
    /// Period start date (`YYYYMMDD`).
    pub from_date: String,
    /// Period end date (`YYYYMMDD`).
    pub to_date: String,
    /// Company GSTIN when known.
    pub company_gstin: Option<String>,
    /// How this GSTR-1 report was produced.
    pub source: Gstr1Source,
    /// B2B invoices.
    pub b2b: Vec<Gstr1B2bInvoice>,
    /// B2C large invoices.
    pub b2cl: Vec<Gstr1B2cInvoice>,
    /// B2C small summaries.
    pub b2cs: Vec<Gstr1B2cSummary>,
    /// Credit/debit notes (registered).
    pub cdnr: Vec<Gstr1CdnrNote>,
    /// HSN-wise summary rows.
    pub hsn: Vec<Gstr1HsnRow>,
    /// Document summary rows.
    pub documents: Vec<Gstr1DocumentSummary>,
}
