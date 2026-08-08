//! GST / GSTR report models.
//!
//! TallyPrime does not expose a builtin HTTP report named `GSTR-1` on current
//! builds (export returns "Could not find Report 'GSTR-1'"). This crate therefore:
//! - exports the native **GST Computation** report when available
//! - builds a **GSTR-1 style** summary from voucher data for the period

#[derive(Debug, Clone, Default)]
pub struct GstComputationEntry {
    pub particulars: String,
    pub taxable_value: Option<f64>,
    pub igst: Option<f64>,
    pub cgst: Option<f64>,
    pub sgst: Option<f64>,
    pub cess: Option<f64>,
    pub total_tax: Option<f64>,
}

#[derive(Debug, Clone, Default)]
pub struct GstComputationReport {
    pub name_field: Option<String>,
    pub entries: Vec<GstComputationEntry>,
}

/// How [`Gstr1Report`] was produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gstr1Source {
    /// Aggregated from sales / credit-note / debit-note vouchers.
    VoucherDerived,
}

#[derive(Debug, Clone, Default)]
pub struct Gstr1TaxBreakup {
    pub taxable_value: f64,
    pub igst: f64,
    pub cgst: f64,
    pub sgst: f64,
    pub cess: f64,
}

impl Gstr1TaxBreakup {
    pub fn total_tax(&self) -> f64 {
        self.igst + self.cgst + self.sgst + self.cess
    }

    pub fn invoice_value(&self) -> f64 {
        self.taxable_value + self.total_tax()
    }
}

#[derive(Debug, Clone)]
pub struct Gstr1B2bInvoice {
    pub party_gstin: String,
    pub party_name: Option<String>,
    pub invoice_number: Option<String>,
    pub invoice_date: String,
    pub place_of_supply: Option<String>,
    pub voucher_type: String,
    pub taxes: Gstr1TaxBreakup,
}

#[derive(Debug, Clone)]
pub struct Gstr1B2cInvoice {
    pub party_name: Option<String>,
    pub invoice_number: Option<String>,
    pub invoice_date: String,
    pub place_of_supply: Option<String>,
    pub voucher_type: String,
    pub taxes: Gstr1TaxBreakup,
}

#[derive(Debug, Clone)]
pub struct Gstr1B2cSummary {
    pub place_of_supply: Option<String>,
    pub rate: Option<f64>,
    pub taxes: Gstr1TaxBreakup,
    pub invoice_count: usize,
}

#[derive(Debug, Clone)]
pub struct Gstr1CdnrNote {
    pub party_gstin: Option<String>,
    pub party_name: Option<String>,
    pub note_number: Option<String>,
    pub note_date: String,
    pub note_type: String,
    pub place_of_supply: Option<String>,
    pub taxes: Gstr1TaxBreakup,
}

#[derive(Debug, Clone)]
pub struct Gstr1HsnRow {
    pub hsn_code: String,
    pub description: Option<String>,
    pub uqc: Option<String>,
    pub total_quantity: Option<f64>,
    pub taxes: Gstr1TaxBreakup,
}

#[derive(Debug, Clone)]
pub struct Gstr1DocumentSummary {
    pub document_type: String,
    pub count: usize,
    pub cancelled: usize,
}

#[derive(Debug, Clone)]
pub struct Gstr1Report {
    pub from_date: String,
    pub to_date: String,
    pub company_gstin: Option<String>,
    pub source: Gstr1Source,
    pub b2b: Vec<Gstr1B2bInvoice>,
    pub b2cl: Vec<Gstr1B2cInvoice>,
    pub b2cs: Vec<Gstr1B2cSummary>,
    pub cdnr: Vec<Gstr1CdnrNote>,
    pub hsn: Vec<Gstr1HsnRow>,
    pub documents: Vec<Gstr1DocumentSummary>,
}
