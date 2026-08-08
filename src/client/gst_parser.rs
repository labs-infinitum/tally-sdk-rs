use crate::models::{
    GstComputationEntry, GstComputationReport, Gstr1B2bInvoice, Gstr1B2cInvoice, Gstr1B2cSummary,
    Gstr1CdnrNote, Gstr1DocumentSummary, Gstr1HsnRow, Gstr1Report, Gstr1Source, Gstr1TaxBreakup,
    Voucher,
};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::BTreeMap;

/// Parse Tally's builtin `GST Computation` XML export.
///
/// The response is a flat sequence of sibling tags: each row starts with
/// `GSTCOMPPARTICULARS` followed by taxable/IGST/CGST/SGST/cess/total values.
pub fn parse_gst_computation_from_xml(xml: &str) -> GstComputationReport {
    let mut reader = Reader::from_reader(xml.as_bytes());
    reader.trim_text(true);

    let mut report = GstComputationReport::default();
    let mut path: Vec<Vec<u8>> = Vec::new();
    let mut current = GstComputationEntry::default();
    let mut have_row = false;

    let flush = |report: &mut GstComputationReport,
                 current: &mut GstComputationEntry,
                 have_row: &mut bool| {
        if *have_row && !current.particulars.trim().is_empty() {
            report.entries.push(std::mem::take(current));
        }
        *have_row = false;
    };

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                path.push(e.name().as_ref().to_vec());
            }
            Ok(Event::Text(ref e)) => {
                let text = e.unescape().unwrap_or_default().to_string();
                let trimmed = text.trim();
                match path.last().map(|tag| tag.as_slice()) {
                    Some(b"NAMEFIELD") => {
                        if !trimmed.is_empty() {
                            report.name_field = Some(trimmed.to_string());
                        }
                    }
                    Some(b"GSTCOMPPARTICULARS") => {
                        flush(&mut report, &mut current, &mut have_row);
                        current.particulars = trimmed.to_string();
                        have_row = true;
                    }
                    Some(b"GSTCOMPTAXABLEVAL") => {
                        current.taxable_value = parse_amount(trimmed);
                    }
                    Some(b"GSTCOMPIGSTVAL") => current.igst = parse_amount(trimmed),
                    Some(b"GSTCOMPCGSTVAL") => current.cgst = parse_amount(trimmed),
                    Some(b"GSTCOMPSGSTVAL") => current.sgst = parse_amount(trimmed),
                    Some(b"GSTCOMPCESSVAL") => current.cess = parse_amount(trimmed),
                    Some(b"GSTCOMPTOTTAXVAL") => current.total_tax = parse_amount(trimmed),
                    _ => {}
                }
            }
            Ok(Event::End(_)) => {
                path.pop();
            }
            Ok(Event::Eof) => {
                flush(&mut report, &mut current, &mut have_row);
                break;
            }
            Err(_) => break,
            _ => {}
        }
    }

    report
}

/// Build a GSTR-1 style report from vouchers in a period.
///
/// This is used because TallyPrime does not currently expose a builtin HTTP
/// report ID of `GSTR-1` (export returns "Could not find Report").
pub fn build_gstr1_from_vouchers(
    from_date: &str,
    to_date: &str,
    vouchers: &[Voucher],
) -> Gstr1Report {
    let mut report = Gstr1Report {
        from_date: from_date.to_string(),
        to_date: to_date.to_string(),
        company_gstin: None,
        source: Gstr1Source::VoucherDerived,
        b2b: Vec::new(),
        b2cl: Vec::new(),
        b2cs: Vec::new(),
        cdnr: Vec::new(),
        hsn: Vec::new(),
        documents: Vec::new(),
    };

    let mut b2cs_map: BTreeMap<(String, String), (Gstr1TaxBreakup, usize)> = BTreeMap::new();
    let mut hsn_map: BTreeMap<String, Gstr1HsnRow> = BTreeMap::new();
    let mut doc_map: BTreeMap<String, (usize, usize)> = BTreeMap::new();

    for voucher in vouchers {
        if voucher.is_optional {
            continue;
        }
        if report.company_gstin.is_none() {
            report.company_gstin = voucher.cmp_gstin.clone();
        }

        let kind = classify_voucher(voucher);
        if kind == VoucherKind::Other {
            continue;
        }

        let doc_key = voucher.voucher_type.clone();
        let entry = doc_map.entry(doc_key).or_insert((0, 0));
        entry.0 += 1;
        if voucher.is_cancelled {
            entry.1 += 1;
            continue;
        }

        let taxes = tax_breakup_from_voucher(voucher);
        accumulate_hsn(&mut hsn_map, voucher, &taxes);

        match kind {
            VoucherKind::Sales => {
                if let Some(gstin) = normalized_gstin(voucher.party_gstin.as_deref()) {
                    report.b2b.push(Gstr1B2bInvoice {
                        party_gstin: gstin,
                        party_name: voucher.party_ledger_name.clone(),
                        invoice_number: voucher.voucher_number.clone(),
                        invoice_date: voucher.date_yyyymmdd.clone(),
                        place_of_supply: voucher.place_of_supply.clone(),
                        voucher_type: voucher.voucher_type.clone(),
                        taxes,
                    });
                } else if is_b2cl(&taxes, voucher.place_of_supply.as_deref()) {
                    report.b2cl.push(Gstr1B2cInvoice {
                        party_name: voucher.party_ledger_name.clone(),
                        invoice_number: voucher.voucher_number.clone(),
                        invoice_date: voucher.date_yyyymmdd.clone(),
                        place_of_supply: voucher.place_of_supply.clone(),
                        voucher_type: voucher.voucher_type.clone(),
                        taxes,
                    });
                } else {
                    let pos = voucher
                        .place_of_supply
                        .clone()
                        .unwrap_or_else(|| "Unknown".into());
                    let rate_key = dominant_rate(voucher)
                        .map(|rate| format!("{rate:.2}"))
                        .unwrap_or_else(|| "0.00".into());
                    let slot = b2cs_map
                        .entry((pos, rate_key))
                        .or_insert((Gstr1TaxBreakup::default(), 0));
                    add_taxes(&mut slot.0, &taxes);
                    slot.1 += 1;
                }
            }
            VoucherKind::CreditNote | VoucherKind::DebitNote => {
                report.cdnr.push(Gstr1CdnrNote {
                    party_gstin: normalized_gstin(voucher.party_gstin.as_deref()),
                    party_name: voucher.party_ledger_name.clone(),
                    note_number: voucher.voucher_number.clone(),
                    note_date: voucher.date_yyyymmdd.clone(),
                    note_type: voucher.voucher_type.clone(),
                    place_of_supply: voucher.place_of_supply.clone(),
                    taxes,
                });
            }
            VoucherKind::Other => {}
        }
    }

    report.b2cs = b2cs_map
        .into_iter()
        .map(|((pos, rate), (taxes, count))| Gstr1B2cSummary {
            place_of_supply: Some(pos).filter(|value| value != "Unknown"),
            rate: rate.parse::<f64>().ok().filter(|value| *value > 0.0),
            taxes,
            invoice_count: count,
        })
        .collect();
    report.hsn = hsn_map.into_values().collect();
    report.documents = doc_map
        .into_iter()
        .map(|(document_type, (count, cancelled))| Gstr1DocumentSummary {
            document_type,
            count,
            cancelled,
        })
        .collect();

    report
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VoucherKind {
    Sales,
    CreditNote,
    DebitNote,
    Other,
}

fn classify_voucher(voucher: &Voucher) -> VoucherKind {
    let t = voucher.voucher_type.to_lowercase();
    if t.contains("credit note") || t == "credit note" || t.contains("creditnote") {
        VoucherKind::CreditNote
    } else if t.contains("debit note") || t == "debit note" || t.contains("debitnote") {
        VoucherKind::DebitNote
    } else if t.contains("sales") || voucher.is_invoice {
        VoucherKind::Sales
    } else {
        VoucherKind::Other
    }
}

fn normalized_gstin(value: Option<&str>) -> Option<String> {
    let trimmed = value.map(str::trim).filter(|value| !value.is_empty())?;
    Some(trimmed.to_uppercase())
}

fn is_b2cl(taxes: &Gstr1TaxBreakup, place_of_supply: Option<&str>) -> bool {
    // GSTR-1 B2CL: inter-state B2C invoices above 2.5 lakh.
    taxes.invoice_value() > 250_000.0 && (taxes.igst > 0.0 || looks_interstate(place_of_supply))
}

fn looks_interstate(place_of_supply: Option<&str>) -> bool {
    // Without company state comparison, treat non-empty foreign/other POS with IGST-only
    // as interstate; this helper is a weak signal used with invoice value.
    place_of_supply
        .map(|pos| {
            let p = pos.to_lowercase();
            !(p.contains("uttar pradesh") || p.contains("up-") || p == "up")
        })
        .unwrap_or(false)
}

fn tax_breakup_from_voucher(voucher: &Voucher) -> Gstr1TaxBreakup {
    let mut taxes = Gstr1TaxBreakup::default();
    let mut tax_total = 0.0_f64;
    let mut non_party_non_tax = 0.0_f64;

    for entry in &voucher.entries {
        let amount = entry.amount.abs() as f64;
        match tax_head(&entry.ledger_name) {
            Some(TaxHead::Igst) => {
                taxes.igst += amount;
                tax_total += amount;
            }
            Some(TaxHead::Cgst) => {
                taxes.cgst += amount;
                tax_total += amount;
            }
            Some(TaxHead::Sgst) => {
                taxes.sgst += amount;
                tax_total += amount;
            }
            Some(TaxHead::Cess) => {
                taxes.cess += amount;
                tax_total += amount;
            }
            None if !entry.is_party_ledger => {
                non_party_non_tax += amount;
            }
            None => {}
        }
    }

    if non_party_non_tax > 0.0 {
        taxes.taxable_value = non_party_non_tax;
    } else if let Some(amount) = voucher.amount {
        taxes.taxable_value = (amount.abs() as f64 - tax_total).max(0.0);
    } else {
        taxes.taxable_value = voucher
            .items
            .iter()
            .map(|item| item.amount.abs() as f64)
            .sum();
    }

    taxes
}

#[derive(Debug, Clone, Copy)]
enum TaxHead {
    Igst,
    Cgst,
    Sgst,
    Cess,
}

fn tax_head(ledger_name: &str) -> Option<TaxHead> {
    let n = ledger_name.to_lowercase();
    if n.contains("igst") {
        Some(TaxHead::Igst)
    } else if n.contains("cgst") {
        Some(TaxHead::Cgst)
    } else if n.contains("sgst") || n.contains("utgst") {
        Some(TaxHead::Sgst)
    } else if n.contains("cess") {
        Some(TaxHead::Cess)
    } else {
        None
    }
}

fn dominant_rate(voucher: &Voucher) -> Option<f64> {
    let mut rates: Vec<f32> = voucher
        .items
        .iter()
        .flat_map(|item| item.gst_rate_details.iter().map(|detail| detail.rate))
        .collect();
    if rates.is_empty() {
        return None;
    }
    rates.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    Some(rates[rates.len() / 2] as f64)
}

fn accumulate_hsn(
    hsn_map: &mut BTreeMap<String, Gstr1HsnRow>,
    voucher: &Voucher,
    voucher_taxes: &Gstr1TaxBreakup,
) {
    let item_count = voucher.items.len().max(1) as f64;
    for item in &voucher.items {
        let Some(hsn) = item
            .gst_hsn_code
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
        else {
            continue;
        };
        let row = hsn_map.entry(hsn.to_string()).or_insert(Gstr1HsnRow {
            hsn_code: hsn.to_string(),
            description: item.gst_hsn_description.clone(),
            uqc: None,
            total_quantity: None,
            taxes: Gstr1TaxBreakup::default(),
        });
        if row.description.is_none() {
            row.description = item.gst_hsn_description.clone();
        }
        let qty = item
            .billed_qty
            .or(item.actual_qty)
            .map(|value| value as f64);
        if let Some(qty) = qty {
            row.total_quantity = Some(row.total_quantity.unwrap_or(0.0) + qty);
        }
        // Spread voucher tax proportionally when item-level tax lines are absent.
        let share = if voucher.items.iter().any(|item| item.gst_hsn_code.is_some()) {
            let hsn_items = voucher
                .items
                .iter()
                .filter(|item| {
                    item.gst_hsn_code
                        .as_deref()
                        .is_some_and(|code| !code.trim().is_empty())
                })
                .count()
                .max(1) as f64;
            1.0 / hsn_items
        } else {
            1.0 / item_count
        };
        row.taxes.taxable_value += item.amount.abs() as f64;
        row.taxes.igst += voucher_taxes.igst * share;
        row.taxes.cgst += voucher_taxes.cgst * share;
        row.taxes.sgst += voucher_taxes.sgst * share;
        row.taxes.cess += voucher_taxes.cess * share;
    }
}

fn add_taxes(dst: &mut Gstr1TaxBreakup, src: &Gstr1TaxBreakup) {
    dst.taxable_value += src.taxable_value;
    dst.igst += src.igst;
    dst.cgst += src.cgst;
    dst.sgst += src.sgst;
    dst.cess += src.cess;
}

fn parse_amount(text: &str) -> Option<f64> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }
    trimmed.parse::<f64>().ok()
}

#[cfg(test)]
mod tests {
    use super::{build_gstr1_from_vouchers, parse_gst_computation_from_xml};
    use crate::models::{Gstr1Source, Item, Voucher, VoucherEntry};

    #[test]
    fn parses_gst_computation_rows() {
        let xml = r#"
<ENVELOPE>
 <NAMEFIELD>Liability</NAMEFIELD>
 <GSTCOMPPARTICULARS>Outward and Inward Supplies on Which Tax is Payable (Including Advances)</GSTCOMPPARTICULARS>
 <GSTCOMPTAXABLEVAL>1000.00</GSTCOMPTAXABLEVAL>
 <GSTCOMPIGSTVAL>180.00</GSTCOMPIGSTVAL>
 <GSTCOMPCGSTVAL></GSTCOMPCGSTVAL>
 <GSTCOMPSGSTVAL></GSTCOMPSGSTVAL>
 <GSTCOMPCESSVAL></GSTCOMPCESSVAL>
 <GSTCOMPTOTTAXVAL>180.00</GSTCOMPTOTTAXVAL>
 <GSTCOMPPARTICULARS>Input Tax Credit</GSTCOMPPARTICULARS>
 <GSTCOMPTAXABLEVAL></GSTCOMPTAXABLEVAL>
 <GSTCOMPIGSTVAL>50.00</GSTCOMPIGSTVAL>
 <GSTCOMPCGSTVAL>25.00</GSTCOMPCGSTVAL>
 <GSTCOMPSGSTVAL>25.00</GSTCOMPSGSTVAL>
 <GSTCOMPCESSVAL></GSTCOMPCESSVAL>
 <GSTCOMPTOTTAXVAL>100.00</GSTCOMPTOTTAXVAL>
</ENVELOPE>
"#;
        let report = parse_gst_computation_from_xml(xml);
        assert_eq!(report.name_field.as_deref(), Some("Liability"));
        assert_eq!(report.entries.len(), 2);
        assert!(report.entries[0].particulars.contains("Outward and Inward"));
        assert_eq!(report.entries[0].taxable_value, Some(1000.0));
        assert_eq!(report.entries[0].igst, Some(180.0));
        assert_eq!(report.entries[1].particulars, "Input Tax Credit");
        assert_eq!(report.entries[1].total_tax, Some(100.0));
    }

    #[test]
    fn builds_gstr1_b2b_and_b2cs_from_vouchers() {
        let vouchers = vec![
            Voucher {
                voucher_id: "1".into(),
                remote_id: None,
                vch_key: None,
                voucher_type: "Sales".into(),
                action: None,
                date_yyyymmdd: "20250410".into(),
                amount: Some(-1180.0),
                amount_forex: None,
                voucher_number: Some("INV-1".into()),
                reference: None,
                party_ledger_name: Some("Acme Traders".into()),
                cmp_gst_registration_type: Some("Regular".into()),
                party_gstin: Some("09ABCDE1234F1Z5".into()),
                cmp_gstin: Some("09AAECO1485A1ZN".into()),
                place_of_supply: Some("Uttar Pradesh".into()),
                entries: vec![
                    VoucherEntry {
                        ledger_name: "Acme Traders".into(),
                        amount: 1180.0,
                        forex: None,
                        is_debit: true,
                        is_party_ledger: true,
                        bill_allocations: vec![],
                    },
                    VoucherEntry {
                        ledger_name: "Sales".into(),
                        amount: 1000.0,
                        forex: None,
                        is_debit: false,
                        is_party_ledger: false,
                        bill_allocations: vec![],
                    },
                    VoucherEntry {
                        ledger_name: "CGST".into(),
                        amount: 90.0,
                        forex: None,
                        is_debit: false,
                        is_party_ledger: false,
                        bill_allocations: vec![],
                    },
                    VoucherEntry {
                        ledger_name: "SGST".into(),
                        amount: 90.0,
                        forex: None,
                        is_debit: false,
                        is_party_ledger: false,
                        bill_allocations: vec![],
                    },
                ],
                items: vec![],
                narration: None,
                reference_date: None,
                effective_date: None,
                is_invoice: true,
                is_cancelled: false,
                is_optional: false,
                entry_mode: None,
                alter_id: None,
                master_id: None,
            },
            Voucher {
                voucher_id: "2".into(),
                remote_id: None,
                vch_key: None,
                voucher_type: "Sales".into(),
                action: None,
                date_yyyymmdd: "20250411".into(),
                amount: Some(-500.0),
                amount_forex: None,
                voucher_number: Some("INV-2".into()),
                reference: None,
                party_ledger_name: Some("Walk-in Customer".into()),
                cmp_gst_registration_type: Some("Regular".into()),
                party_gstin: None,
                cmp_gstin: Some("09AAECO1485A1ZN".into()),
                place_of_supply: Some("Uttar Pradesh".into()),
                entries: vec![
                    VoucherEntry {
                        ledger_name: "Walk-in Customer".into(),
                        amount: 500.0,
                        forex: None,
                        is_debit: true,
                        is_party_ledger: true,
                        bill_allocations: vec![],
                    },
                    VoucherEntry {
                        ledger_name: "Sales".into(),
                        amount: 500.0,
                        forex: None,
                        is_debit: false,
                        is_party_ledger: false,
                        bill_allocations: vec![],
                    },
                ],
                items: vec![Item {
                    name: "Widget".into(),
                    amount: 500.0,
                    forex: None,
                    rate: None,
                    discount: None,
                    actual_qty: Some(1.0),
                    billed_qty: Some(1.0),
                    gst_hsn_code: Some("998314".into()),
                    gst_hsn_description: Some("IT services".into()),
                    gst_taxability: None,
                    gst_type_of_supply: None,
                    batch_allocations: vec![],
                    accounting_allocations: vec![],
                    gst_rate_details: vec![],
                }],
                narration: None,
                reference_date: None,
                effective_date: None,
                is_invoice: true,
                is_cancelled: false,
                is_optional: false,
                entry_mode: None,
                alter_id: None,
                master_id: None,
            },
        ];

        let report = build_gstr1_from_vouchers("20250401", "20250430", &vouchers);
        assert_eq!(report.source, Gstr1Source::VoucherDerived);
        assert_eq!(report.company_gstin.as_deref(), Some("09AAECO1485A1ZN"));
        assert_eq!(report.b2b.len(), 1);
        assert_eq!(report.b2b[0].party_gstin, "09ABCDE1234F1Z5");
        assert_eq!(report.b2b[0].taxes.taxable_value, 1000.0);
        assert_eq!(report.b2b[0].taxes.cgst, 90.0);
        assert_eq!(report.b2cs.len(), 1);
        assert_eq!(report.b2cs[0].invoice_count, 1);
        assert_eq!(report.hsn.len(), 1);
        assert_eq!(report.hsn[0].hsn_code, "998314");
        assert_eq!(report.documents.len(), 1);
    }
}
