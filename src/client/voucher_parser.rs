use crate::models::{
    AccountingAllocation, BatchAllocation, ForexDetails, GstRateDetail, Item, Voucher,
    VoucherEntry,
};
use quick_xml::events::{BytesStart, Event};
use quick_xml::name::QName;
use quick_xml::Reader;
use regex::Regex;
use std::sync::OnceLock;

pub fn parse_vouchers_from_xml(xml: &str) -> Vec<Voucher> {
    let mut reader = Reader::from_reader(xml.as_bytes());
    reader.trim_text(true);

    let mut path: Vec<BytesStart> = vec![];

    // Voucher level fields
    let mut voucher_guid: Option<String> = None;
    let mut voucher_remote_id: Option<String> = None;
    let mut voucher_vch_key: Option<String> = None;
    let mut voucher_action: Option<String> = None;
    let mut voucher_date: Option<String> = None;
    let mut voucher_type: Option<String> = None;
    let mut voucher_amount: Option<f32> = None;
    let mut voucher_amount_forex: Option<ForexDetails> = None;
    let mut voucher_number: Option<String> = None;
    let mut reference: Option<String> = None;
    let mut reference_date: Option<String> = None;
    let mut effective_date: Option<String> = None;
    let mut narration: Option<String> = None;
    let mut party_ledger_name: Option<String> = None;
    let mut cmp_gst_registration_type: Option<String> = None;
    let mut party_gstin: Option<String> = None;
    let mut cmp_gstin: Option<String> = None;
    let mut place_of_supply: Option<String> = None;
    let mut is_invoice: Option<String> = None;
    let mut is_cancelled: Option<String> = None;
    let mut is_optional: Option<String> = None;
    let mut entry_mode: Option<String> = None;
    let mut alter_id: Option<i32> = None;
    let mut master_id: Option<i32> = None;

    // Item level fields
    let mut item_name: Option<String> = None;
    let mut item_amount: Option<f32> = None;
    let mut item_forex: Option<ForexDetails> = None;
    let mut item_rate: Option<f32> = None;
    let mut item_discount: Option<f32> = None;
    let mut item_actual_qty: Option<f32> = None;
    let mut item_billed_qty: Option<f32> = None;
    let mut item_gst_hsn_code: Option<String> = None;
    let mut item_gst_hsn_description: Option<String> = None;
    let mut item_gst_taxability: Option<String> = None;
    let mut item_gst_type_of_supply: Option<String> = None;

    // Batch allocation fields
    let mut batch_godown: Option<String> = None;
    let mut batch_name: Option<String> = None;
    let mut batch_amount: Option<f32> = None;
    let mut batch_forex: Option<ForexDetails> = None;
    let mut batch_actual_qty: Option<f32> = None;
    let mut batch_billed_qty: Option<f32> = None;

    // Accounting allocation fields
    let mut acct_ledger_name: Option<String> = None;
    let mut acct_amount: Option<f32> = None;
    let mut acct_forex: Option<ForexDetails> = None;
    let mut acct_is_deemed_positive: Option<String> = None;

    // GST Rate detail fields
    let mut gst_rate_duty_head: Option<String> = None;
    let mut gst_rate: Option<f32> = None;
    let mut gst_rate_valuation_type: Option<String> = None;

    // Ledger entry fields
    let mut ledger_entry_name: Option<String> = None;
    let mut ledger_entry_amount: Option<f32> = None;
    let mut ledger_entry_forex: Option<ForexDetails> = None;
    let mut ledger_entry_is_party: Option<String> = None;

    // Collections
    let mut vouchers: Vec<Voucher> = vec![];
    let mut items: Vec<Item> = vec![];
    let mut voucher_entries: Vec<VoucherEntry> = vec![];
    let mut batch_allocations: Vec<BatchAllocation> = vec![];
    let mut accounting_allocations: Vec<AccountingAllocation> = vec![];
    let mut gst_rate_details: Vec<GstRateDetail> = vec![];

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                path.push(e.clone());
                match e.name() {
                    QName(b"VOUCHER") => {
                        items = vec![];
                        voucher_entries = vec![];
                        // Reset voucher level fields
                        voucher_guid = None;
                        voucher_remote_id = None;
                        voucher_vch_key = None;
                        voucher_action = None;
                        voucher_date = None;
                        voucher_type = None;
                        voucher_amount = None;
                        voucher_amount_forex = None;
                        voucher_number = None;
                        reference = None;
                        reference_date = None;
                        effective_date = None;
                        narration = None;
                        party_ledger_name = None;
                        cmp_gst_registration_type = None;
                        party_gstin = None;
                        cmp_gstin = None;
                        place_of_supply = None;
                        is_invoice = None;
                        is_cancelled = None;
                        is_optional = None;
                        entry_mode = None;
                        alter_id = None;
                        master_id = None;

                        // Parse VOUCHER attributes
                        for attr in e.attributes().flatten() {
                            let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                            let value = attr.unescape_value().unwrap().to_string();
                            match key.as_str() {
                                "REMOTEID" => voucher_remote_id = Some(value),
                                "VCHKEY" => voucher_vch_key = Some(value),
                                "ACTION" => voucher_action = Some(value),
                                _ => {}
                            }
                        }
                    }
                    QName(b"ALLINVENTORYENTRIES.LIST") => {
                        // Reset item level fields
                        item_name = None;
                        item_amount = None;
                        item_forex = None;
                        item_rate = None;
                        item_discount = None;
                        item_actual_qty = None;
                        item_billed_qty = None;
                        item_gst_hsn_code = None;
                        item_gst_hsn_description = None;
                        item_gst_taxability = None;
                        item_gst_type_of_supply = None;
                        batch_allocations = vec![];
                        accounting_allocations = vec![];
                        gst_rate_details = vec![];
                    }
                    QName(b"BATCHALLOCATIONS.LIST") => {
                        batch_godown = None;
                        batch_name = None;
                        batch_amount = None;
                        batch_forex = None;
                        batch_actual_qty = None;
                        batch_billed_qty = None;
                    }
                    QName(b"ACCOUNTINGALLOCATIONS.LIST") => {
                        acct_ledger_name = None;
                        acct_amount = None;
                        acct_forex = None;
                        acct_is_deemed_positive = None;
                    }
                    QName(b"RATEDETAILS.LIST") => {
                        gst_rate_duty_head = None;
                        gst_rate = None;
                        gst_rate_valuation_type = None;
                    }
                    QName(b"LEDGERENTRIES.LIST") => {
                        ledger_entry_name = None;
                        ledger_entry_amount = None;
                        ledger_entry_forex = None;
                        ledger_entry_is_party = None;
                    }
                    QName(b"ALLLEDGERENTRIES.LIST") => {
                        ledger_entry_name = None;
                        ledger_entry_amount = None;
                        ledger_entry_forex = None;
                        ledger_entry_is_party = None;
                    }
                    _ => {}
                }
            }

            Ok(Event::End(ref e)) => {
                match e.name() {
                    QName(b"VOUCHER") => {
                        vouchers.push(Voucher {
                            voucher_id: voucher_guid.clone().unwrap_or_default(),
                            remote_id: voucher_remote_id.clone(),
                            vch_key: voucher_vch_key.clone(),
                            voucher_type: voucher_type.clone().unwrap_or_default(),
                            action: voucher_action.clone(),
                            date_yyyymmdd: voucher_date.clone().unwrap_or_default(),
                            amount: voucher_amount,
                            amount_forex: voucher_amount_forex.clone(),
                            voucher_number: voucher_number.clone(),
                            reference: reference.clone(),
                            party_ledger_name: party_ledger_name.clone(),
                            cmp_gst_registration_type: cmp_gst_registration_type.clone(),
                            party_gstin: party_gstin.clone(),
                            cmp_gstin: cmp_gstin.clone(),
                            place_of_supply: place_of_supply.clone(),
                            entries: voucher_entries.clone(),
                            items: items.clone(),
                            narration: narration.clone(),
                            reference_date: reference_date.clone(),
                            effective_date: effective_date.clone(),
                            is_invoice: is_invoice.as_ref().map_or(false, |v| v == "Yes"),
                            is_cancelled: is_cancelled.as_ref().map_or(false, |v| v == "Yes"),
                            is_optional: is_optional.as_ref().map_or(false, |v| v == "Yes"),
                            entry_mode: entry_mode.clone(),
                            alter_id,
                            master_id,
                        });
                    }

                    QName(b"ALLINVENTORYENTRIES.LIST") => {
                        if let Some(name) = item_name.clone() {
                            items.push(Item {
                                name,
                                amount: item_amount.unwrap_or(0.0),
                                forex: item_forex.clone(),
                                rate: item_rate,
                                discount: item_discount,
                                actual_qty: item_actual_qty,
                                billed_qty: item_billed_qty,
                                gst_hsn_code: item_gst_hsn_code.clone(),
                                gst_hsn_description: item_gst_hsn_description.clone(),
                                gst_taxability: item_gst_taxability.clone(),
                                gst_type_of_supply: item_gst_type_of_supply.clone(),
                                batch_allocations: batch_allocations.clone(),
                                accounting_allocations: accounting_allocations.clone(),
                                gst_rate_details: gst_rate_details.clone(),
                            });
                        }
                    }

                    QName(b"BATCHALLOCATIONS.LIST") => {
                        if let (Some(godown), Some(batch), Some(amount)) =
                            (batch_godown.clone(), batch_name.clone(), batch_amount)
                        {
                            batch_allocations.push(BatchAllocation {
                                godown_name: godown,
                                batch_name: batch,
                                amount,
                                forex: batch_forex.clone(),
                                actual_qty: batch_actual_qty,
                                billed_qty: batch_billed_qty,
                            });
                        }
                    }

                    QName(b"ACCOUNTINGALLOCATIONS.LIST") => {
                        if let (Some(ledger), Some(amount)) =
                            (acct_ledger_name.clone(), acct_amount)
                        {
                            accounting_allocations.push(AccountingAllocation {
                                ledger_name: ledger,
                                amount,
                                forex: acct_forex.clone(),
                                is_deemed_positive: acct_is_deemed_positive
                                    .as_ref()
                                    .map_or(false, |v| v == "Yes"),
                            });
                        }
                    }

                    QName(b"RATEDETAILS.LIST") => {
                        if let (Some(duty_head), Some(rate)) =
                            (gst_rate_duty_head.clone(), gst_rate)
                        {
                            gst_rate_details.push(GstRateDetail {
                                duty_head,
                                rate,
                                valuation_type: gst_rate_valuation_type.clone(),
                            });
                        }
                    }

                    QName(b"LEDGERENTRIES.LIST") | QName(b"ALLLEDGERENTRIES.LIST") => {
                        if let (Some(name), Some(amount)) =
                            (ledger_entry_name.clone(), ledger_entry_amount)
                        {
                            voucher_entries.push(VoucherEntry {
                                ledger_name: name,
                                amount: amount.abs(),
                                forex: ledger_entry_forex.clone().map(abs_forex),
                                is_debit: amount > 0.0,
                                is_party_ledger: ledger_entry_is_party
                                    .as_ref()
                                    .map_or(false, |v| v == "Yes"),
                            });
                        }
                    }
                    _ => {}
                }
                path.pop();
            }

            Ok(Event::Text(ref e)) => {
                if let Some(last) = path.last() {
                    let text = e.unescape().unwrap().to_string();

                    // Helper to check if we're in a specific parent context
                    let in_context = |parent_tag: &[u8]| -> bool {
                        path.iter().any(|p| p.name() == QName(parent_tag))
                    };
                    let in_ledger_entries = || {
                        in_context(b"LEDGERENTRIES.LIST") || in_context(b"ALLLEDGERENTRIES.LIST")
                    };

                    match last.name() {
                        // Voucher level fields
                        QName(b"GUID") if !in_context(b"ALLINVENTORYENTRIES.LIST") => {
                            voucher_guid = Some(text);
                        }
                        QName(b"DATE")
                            if !in_context(b"ALLINVENTORYENTRIES.LIST") && !in_ledger_entries() =>
                        {
                            voucher_date = Some(text);
                        }
                        QName(b"AMOUNT")
                            if !in_context(b"ALLINVENTORYENTRIES.LIST")
                                && !in_context(b"BATCHALLOCATIONS.LIST")
                                && !in_context(b"ACCOUNTINGALLOCATIONS.LIST")
                                && !in_ledger_entries() =>
                        {
                            if let Some(parsed) = parse_tally_amount_details(&text) {
                                voucher_amount = Some(parsed.amount);
                                voucher_amount_forex = parsed.forex;
                            }
                        }
                        QName(b"VOUCHERTYPENAME") => {
                            voucher_type = Some(text);
                        }
                        QName(b"VOUCHERNUMBER") => {
                            voucher_number = Some(text);
                        }
                        QName(b"REFERENCE") => {
                            reference = Some(text);
                        }
                        QName(b"REFERENCEDATE") => {
                            reference_date = Some(text);
                        }
                        QName(b"EFFECTIVEDATE") => {
                            effective_date = Some(text);
                        }
                        QName(b"NARRATION")
                            if !in_context(b"ALLINVENTORYENTRIES.LIST") && !in_ledger_entries() =>
                        {
                            narration = Some(text);
                        }
                        QName(b"PARTYLEDGERNAME") => {
                            party_ledger_name = Some(text);
                        }
                        QName(b"CMPGSTREGISTRATIONTYPE") => {
                            cmp_gst_registration_type = Some(text);
                        }
                        QName(b"PARTYGSTIN") => {
                            party_gstin = Some(text);
                        }
                        QName(b"CMPGSTIN") => {
                            cmp_gstin = Some(text);
                        }
                        QName(b"PLACEOFSUPPLY") => {
                            place_of_supply = Some(text);
                        }
                        QName(b"ISINVOICE") => {
                            is_invoice = Some(text);
                        }
                        QName(b"ISCANCELLED") => {
                            is_cancelled = Some(text);
                        }
                        QName(b"VCHENTRYMODE") => {
                            entry_mode = Some(text);
                        }
                        QName(b"VCHSTATUSISOPTIONAL") => {
                            is_optional = Some(text);
                        }
                        QName(b"ALTERID") => {
                            alter_id = text.trim().parse().ok();
                        }
                        QName(b"MASTERID") => {
                            master_id = text.trim().parse().ok();
                        }

                        // Item level fields
                        QName(b"STOCKITEMNAME") if in_context(b"ALLINVENTORYENTRIES.LIST") => {
                            item_name = Some(text);
                        }
                        QName(b"AMOUNT")
                            if in_context(b"ALLINVENTORYENTRIES.LIST")
                                && !in_context(b"BATCHALLOCATIONS.LIST")
                                && !in_context(b"ACCOUNTINGALLOCATIONS.LIST") =>
                        {
                            if let Some(parsed) = parse_tally_amount_details(&text) {
                                item_amount = Some(parsed.amount);
                                item_forex = parsed.forex;
                            }
                        }
                        QName(b"RATE")
                            if in_context(b"ALLINVENTORYENTRIES.LIST")
                                && !in_context(b"RATEDETAILS.LIST") =>
                        {
                            item_rate = text.parse().ok();
                        }
                        QName(b"DISCOUNT") if in_context(b"ALLINVENTORYENTRIES.LIST") => {
                            item_discount = text.parse().ok();
                        }
                        QName(b"ACTUALQTY")
                            if in_context(b"ALLINVENTORYENTRIES.LIST")
                                && !in_context(b"BATCHALLOCATIONS.LIST") =>
                        {
                            item_actual_qty = text.parse().ok();
                        }
                        QName(b"BILLEDQTY")
                            if in_context(b"ALLINVENTORYENTRIES.LIST")
                                && !in_context(b"BATCHALLOCATIONS.LIST") =>
                        {
                            item_billed_qty = text.parse().ok();
                        }
                        QName(b"GSTHSNNAME") => {
                            item_gst_hsn_code = Some(text);
                        }
                        QName(b"GSTHSNDESCRIPTION") => {
                            item_gst_hsn_description = Some(text);
                        }
                        QName(b"GSTOVRDNTAXABILITY") => {
                            item_gst_taxability = Some(text);
                        }
                        QName(b"GSTOVRDNTYPEOFSUPPLY") => {
                            item_gst_type_of_supply = Some(text);
                        }

                        // Batch allocation fields
                        QName(b"GODOWNNAME") if in_context(b"BATCHALLOCATIONS.LIST") => {
                            batch_godown = Some(text);
                        }
                        QName(b"BATCHNAME") if in_context(b"BATCHALLOCATIONS.LIST") => {
                            batch_name = Some(text);
                        }
                        QName(b"AMOUNT") if in_context(b"BATCHALLOCATIONS.LIST") => {
                            if let Some(parsed) = parse_tally_amount_details(&text) {
                                batch_amount = Some(parsed.amount);
                                batch_forex = parsed.forex;
                            }
                        }
                        QName(b"ACTUALQTY") if in_context(b"BATCHALLOCATIONS.LIST") => {
                            batch_actual_qty = text.parse().ok();
                        }
                        QName(b"BILLEDQTY") if in_context(b"BATCHALLOCATIONS.LIST") => {
                            batch_billed_qty = text.parse().ok();
                        }

                        // Accounting allocation fields
                        QName(b"LEDGERNAME") if in_context(b"ACCOUNTINGALLOCATIONS.LIST") => {
                            acct_ledger_name = Some(text);
                        }
                        QName(b"AMOUNT") if in_context(b"ACCOUNTINGALLOCATIONS.LIST") => {
                            if let Some(parsed) = parse_tally_amount_details(&text) {
                                acct_amount = Some(parsed.amount);
                                acct_forex = parsed.forex;
                            }
                        }
                        QName(b"ISDEEMEDPOSITIVE") if in_context(b"ACCOUNTINGALLOCATIONS.LIST") => {
                            acct_is_deemed_positive = Some(text);
                        }

                        // GST Rate details
                        QName(b"GSTRATEDUTYHEAD") if in_context(b"RATEDETAILS.LIST") => {
                            gst_rate_duty_head = Some(text);
                        }
                        QName(b"GSTRATE") if in_context(b"RATEDETAILS.LIST") => {
                            gst_rate = text.trim().parse().ok();
                        }
                        QName(b"GSTRATEVALUATIONTYPE") => {
                            gst_rate_valuation_type = Some(text);
                        }

                        // Ledger entry fields
                        QName(b"LEDGERNAME")
                            if in_ledger_entries()
                                && !in_context(b"ACCOUNTINGALLOCATIONS.LIST") =>
                        {
                            ledger_entry_name = Some(text);
                        }
                        QName(b"AMOUNT")
                            if in_ledger_entries()
                                && !in_context(b"ACCOUNTINGALLOCATIONS.LIST") =>
                        {
                            if let Some(parsed) = parse_tally_amount_details(&text) {
                                ledger_entry_amount = Some(parsed.amount);
                                ledger_entry_forex = parsed.forex;
                            }
                        }
                        QName(b"ISPARTYLEDGER") if in_ledger_entries() => {
                            ledger_entry_is_party = Some(text);
                        }

                        _ => {}
                    }
                }
            }

            Ok(Event::Eof) => break,
            Err(e) => {
                eprintln!(
                    "Error parsing XML at position {}: {:?}",
                    reader.buffer_position(),
                    e
                );
                break;
            }
            _ => {}
        }
    }

    vouchers
}

#[derive(Debug, Clone)]
struct ParsedTallyAmount {
    amount: f32,
    forex: Option<ForexDetails>,
}

fn parse_tally_amount(text: &str) -> Option<f32> {
    parse_tally_amount_details(text).map(|parsed| parsed.amount)
}

fn parse_tally_amount_details(text: &str) -> Option<ParsedTallyAmount> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }

    if let Ok(value) = trimmed.parse::<f32>() {
        return Some(ParsedTallyAmount {
            amount: value,
            forex: None,
        });
    }

    if let Some((left, rest)) = trimmed.split_once('@') {
        if let Some((rate_part, right)) = rest.split_once('=') {
            let (foreign_amount, foreign_currency_hint) = parse_currency_amount(left.trim())?;
            let (base_amount, base_currency_hint) = parse_currency_amount(right.trim())?;
            let (exchange_rate, rate_base, rate_foreign) = parse_exchange_rate(rate_part.trim())?;

            let foreign_currency = first_non_empty([rate_foreign, foreign_currency_hint])?;
            let base_currency = first_non_empty([rate_base, base_currency_hint])?;

            return Some(ParsedTallyAmount {
                amount: base_amount,
                forex: Some(ForexDetails {
                    foreign_amount,
                    foreign_currency,
                    foreign_currency_name: None,
                    base_currency,
                    base_currency_name: None,
                    exchange_rate,
                }),
            });
        }
    }

    let amount = parse_plain_signed_amount(trimmed)?;
    Some(ParsedTallyAmount {
        amount,
        forex: None,
    })
}

fn parse_currency_amount(text: &str) -> Option<(f32, Option<String>)> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        Regex::new(r"(?x)^\s*(-)?\s*([^\d.\-]*?)\s*([\d.]+)\s*([^\d.\-]*?)\s*$")
            .expect("currency amount regex")
    });

    let caps = re.captures(text)?;
    let negative = caps.get(1).is_some();
    let prefix = caps.get(2).map(|m| m.as_str().trim()).unwrap_or("");
    let number = caps.get(3)?.as_str();
    let suffix = caps.get(4).map(|m| m.as_str().trim()).unwrap_or("");

    let mut value = number.parse::<f32>().ok()?;
    if negative {
        value = -value;
    }

    let currency = if !prefix.is_empty() {
        Some(prefix.to_string())
    } else if !suffix.is_empty() {
        Some(suffix.to_string())
    } else {
        None
    };

    Some((value, currency))
}

fn parse_exchange_rate(text: &str) -> Option<(f32, Option<String>, Option<String>)> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        Regex::new(r"(?x)^\s*([^\d./]+?)\s*([\d.]+)\s*/\s*(.+?)\s*$")
            .expect("exchange rate regex")
    });

    let caps = re.captures(text)?;
    let base_currency = caps.get(1).map(|m| m.as_str().trim().to_string());
    let rate = caps.get(2)?.as_str().parse::<f32>().ok()?;
    let foreign_currency = caps.get(3).map(|m| m.as_str().trim().to_string());

    Some((
        rate,
        base_currency.filter(|value| !value.is_empty()),
        foreign_currency.filter(|value| !value.is_empty()),
    ))
}

fn parse_plain_signed_amount(text: &str) -> Option<f32> {
    let segment = text.rsplit('=').next().unwrap_or(text).trim();
    let negative = segment.contains('-') || (!text.contains('=') && text.starts_with('-'));
    let numeric: String = segment
        .chars()
        .filter(|ch| ch.is_ascii_digit() || *ch == '.')
        .collect();

    if numeric.is_empty() {
        return None;
    }

    let value = numeric.parse::<f32>().ok()?;
    Some(if negative { -value } else { value })
}

fn first_non_empty<const N: usize>(values: [Option<String>; N]) -> Option<String> {
    values.into_iter().flatten().find(|value| !value.is_empty())
}

fn abs_forex(mut forex: ForexDetails) -> ForexDetails {
    forex.foreign_amount = forex.foreign_amount.abs();
    forex
}

#[cfg(test)]
mod tests {
    use super::{parse_tally_amount, parse_tally_amount_details, parse_vouchers_from_xml};

    #[test]
    fn parses_voucher_amount_and_allledgerentries() {
        let xml = r#"
<ENVELOPE>
  <BODY>
    <DATA>
      <COLLECTION>
        <VOUCHER>
          <GUID>abc</GUID>
          <DATE>20250401</DATE>
          <VOUCHERTYPENAME>Payment</VOUCHERTYPENAME>
          <VOUCHERNUMBER>1</VOUCHERNUMBER>
          <PARTYLEDGERNAME>ICICI 2325</PARTYLEDGERNAME>
          <AMOUNT>-250000.00</AMOUNT>
          <ALLLEDGERENTRIES.LIST>
            <LEDGERNAME>ICICI 2325</LEDGERNAME>
            <ISPARTYLEDGER>Yes</ISPARTYLEDGER>
            <AMOUNT>-250000.00</AMOUNT>
          </ALLLEDGERENTRIES.LIST>
          <ALLLEDGERENTRIES.LIST>
            <LEDGERNAME>Bank Charges</LEDGERNAME>
            <ISPARTYLEDGER>No</ISPARTYLEDGER>
            <AMOUNT>250000.00</AMOUNT>
          </ALLLEDGERENTRIES.LIST>
        </VOUCHER>
      </COLLECTION>
    </DATA>
  </BODY>
</ENVELOPE>
"#;

        let vouchers = parse_vouchers_from_xml(xml);
        assert_eq!(vouchers.len(), 1);

        let voucher = &vouchers[0];
        assert_eq!(voucher.amount, Some(-250000.0));
        assert!(voucher.amount_forex.is_none());
        assert_eq!(voucher.entries.len(), 2);
        assert_eq!(voucher.entries[0].ledger_name, "ICICI 2325");
        assert!(voucher.entries[0].is_party_ledger);
        assert_eq!(voucher.entries[0].amount, 250000.0);
        assert!(voucher.entries[0].forex.is_none());
    }

    #[test]
    fn parses_multicurrency_amount_using_base_amount() {
        let amount = parse_tally_amount("-$5000.00 @ ? 84.8565/$ = -? 424282.50");
        assert_eq!(amount, Some(-424282.5));
    }

    #[test]
    fn parses_multicurrency_amount_details() {
        let parsed = parse_tally_amount_details("EUR29.37 @ D$1.1675/EUR = D$34.29")
            .expect("parsed forex amount");
        assert_eq!(parsed.amount, 34.29);
        let forex = parsed.forex.expect("forex details");
        assert_eq!(forex.foreign_amount, 29.37);
        assert_eq!(forex.foreign_currency, "EUR");
        assert_eq!(forex.base_currency, "D$");
        assert!((forex.exchange_rate - 1.1675).abs() < 0.00001);
    }

    #[test]
    fn parses_multicurrency_ledger_entries_on_voucher() {
        let xml = r#"
<ENVELOPE>
  <VOUCHER>
    <GUID>fx-1</GUID>
    <DATE>20251206</DATE>
    <VOUCHERTYPENAME>Journal</VOUCHERTYPENAME>
    <VOUCHERNUMBER>48</VOUCHERNUMBER>
    <PARTYLEDGERNAME>Wise Euro Account</PARTYLEDGERNAME>
    <AMOUNT>-EUR29.37 @ D$1.1675/EUR = -D$34.29</AMOUNT>
    <ALLLEDGERENTRIES.LIST>
      <LEDGERNAME>Wise Euro Account</LEDGERNAME>
      <ISPARTYLEDGER>Yes</ISPARTYLEDGER>
      <AMOUNT>-EUR29.37 @ D$1.1675/EUR = -D$34.29</AMOUNT>
    </ALLLEDGERENTRIES.LIST>
    <ALLLEDGERENTRIES.LIST>
      <LEDGERNAME>Banking Fees &amp; Charges</LEDGERNAME>
      <ISPARTYLEDGER>No</ISPARTYLEDGER>
      <AMOUNT>EUR29.37 @ D$1.1675/EUR = D$34.29</AMOUNT>
    </ALLLEDGERENTRIES.LIST>
  </VOUCHER>
</ENVELOPE>
"#;

        let vouchers = parse_vouchers_from_xml(xml);
        assert_eq!(vouchers.len(), 1);
        let voucher = &vouchers[0];
        assert_eq!(voucher.amount, Some(-34.29));
        let amount_forex = voucher.amount_forex.as_ref().expect("voucher forex");
        assert_eq!(amount_forex.foreign_amount, -29.37);
        assert_eq!(amount_forex.foreign_currency, "EUR");
        assert_eq!(amount_forex.base_currency, "D$");
        assert!((amount_forex.exchange_rate - 1.1675).abs() < 0.00001);

        assert_eq!(voucher.entries.len(), 2);
        let party = &voucher.entries[0];
        assert!(party.is_party_ledger);
        assert_eq!(party.amount, 34.29);
        let party_forex = party.forex.as_ref().expect("party forex");
        assert_eq!(party_forex.foreign_amount, 29.37);
        assert_eq!(party_forex.foreign_currency, "EUR");
        assert_eq!(party_forex.base_currency, "D$");
    }
}
