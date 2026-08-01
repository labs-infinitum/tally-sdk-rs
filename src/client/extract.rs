use crate::models::{
    CompanyDetails, CurrencySummary, GroupSummary, LedgerSummary, StockItemSummary,
};
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;

pub(crate) fn extract_groups_from_xml(xml: &str) -> Vec<GroupSummary> {
    extract_name_parent_nodes(xml, b"GROUP", |name, parent| GroupSummary { name, parent })
}

pub(crate) fn extract_ledgers_from_xml(xml: &str) -> Vec<LedgerSummary> {
    extract_name_parent_nodes(xml, b"LEDGER", |name, parent| LedgerSummary {
        name,
        parent,
    })
}

pub(crate) fn extract_stock_items_from_xml(xml: &str) -> Vec<StockItemSummary> {
    extract_name_parent_nodes(xml, b"STOCKITEM", |name, parent| StockItemSummary {
        name,
        parent,
    })
}

pub(crate) fn extract_currencies_from_xml(xml: &str) -> Vec<CurrencySummary> {
    let mut reader = Reader::from_reader(xml.as_bytes());
    reader.trim_text(true);

    let mut currencies = Vec::new();
    let mut path: Vec<Vec<u8>> = Vec::new();

    let mut name: Option<String> = None;
    let mut original_name: Option<String> = None;
    let mut mailing_name: Option<String> = None;
    let mut expanded_symbol: Option<String> = None;
    let mut decimal_symbol: Option<String> = None;
    let mut decimal_places: Option<i32> = None;
    let mut decimal_places_for_printing: Option<i32> = None;
    let mut is_suffix: Option<bool> = None;
    let mut has_space: Option<bool> = None;
    let mut in_millions: Option<bool> = None;
    let mut guid: Option<String> = None;

    let reset = |name: &mut Option<String>,
                 original_name: &mut Option<String>,
                 mailing_name: &mut Option<String>,
                 expanded_symbol: &mut Option<String>,
                 decimal_symbol: &mut Option<String>,
                 decimal_places: &mut Option<i32>,
                 decimal_places_for_printing: &mut Option<i32>,
                 is_suffix: &mut Option<bool>,
                 has_space: &mut Option<bool>,
                 in_millions: &mut Option<bool>,
                 guid: &mut Option<String>| {
        *name = None;
        *original_name = None;
        *mailing_name = None;
        *expanded_symbol = None;
        *decimal_symbol = None;
        *decimal_places = None;
        *decimal_places_for_printing = None;
        *is_suffix = None;
        *has_space = None;
        *in_millions = None;
        *guid = None;
    };

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                path.push(e.name().as_ref().to_vec());
                if e.name() == QName(b"CURRENCY") {
                    reset(
                        &mut name,
                        &mut original_name,
                        &mut mailing_name,
                        &mut expanded_symbol,
                        &mut decimal_symbol,
                        &mut decimal_places,
                        &mut decimal_places_for_printing,
                        &mut is_suffix,
                        &mut has_space,
                        &mut in_millions,
                        &mut guid,
                    );
                    name = extract_name_attr(e);
                }
            }
            Ok(Event::Empty(ref e)) if e.name() == QName(b"CURRENCY") => {
                if let Some(currency_name) = extract_name_attr(e) {
                    push_or_upgrade_currency(
                        &mut currencies,
                        CurrencySummary {
                            name: currency_name,
                            original_name: None,
                            mailing_name: None,
                            expanded_symbol: None,
                            decimal_symbol: None,
                            decimal_places: None,
                            decimal_places_for_printing: None,
                            is_suffix: None,
                            has_space: None,
                            in_millions: None,
                            guid: None,
                        },
                    );
                }
            }
            Ok(Event::Text(ref e)) => {
                if !in_currency(&path) {
                    continue;
                }
                let text = e.unescape().unwrap_or_default().trim().to_string();
                if text.is_empty() {
                    continue;
                }
                match path.last().map(|tag| tag.as_slice()) {
                    Some(b"ORIGINALNAME") => original_name = Some(text),
                    Some(b"MAILINGNAME") => mailing_name = Some(text),
                    Some(b"EXPANDEDSYMBOL") => expanded_symbol = Some(text),
                    Some(b"DECIMALSYMBOL") => decimal_symbol = Some(text),
                    Some(b"DECIMALPLACES") => decimal_places = parse_i32(&text),
                    Some(b"DECIMALPLACESFORPRINTING") => {
                        decimal_places_for_printing = parse_i32(&text)
                    }
                    Some(b"ISSUFFIX") => is_suffix = parse_yes_no(&text),
                    Some(b"HASSPACE") => has_space = parse_yes_no(&text),
                    Some(b"INMILLIONS") => in_millions = parse_yes_no(&text),
                    Some(b"GUID") => guid = Some(text),
                    Some(b"NAME") if name.is_none() => name = Some(text),
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                if e.name() == QName(b"CURRENCY") {
                    if let Some(currency_name) = name.take() {
                        push_or_upgrade_currency(
                            &mut currencies,
                            CurrencySummary {
                                name: currency_name,
                                original_name: original_name.take(),
                                mailing_name: mailing_name.take(),
                                expanded_symbol: expanded_symbol.take(),
                                decimal_symbol: decimal_symbol.take(),
                                decimal_places,
                                decimal_places_for_printing,
                                is_suffix,
                                has_space,
                                in_millions,
                                guid: guid.take(),
                            },
                        );
                    }
                    reset(
                        &mut name,
                        &mut original_name,
                        &mut mailing_name,
                        &mut expanded_symbol,
                        &mut decimal_symbol,
                        &mut decimal_places,
                        &mut decimal_places_for_printing,
                        &mut is_suffix,
                        &mut has_space,
                        &mut in_millions,
                        &mut guid,
                    );
                }
                path.pop();
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
    }

    currencies
}

pub(crate) fn extract_companies_from_xml(xml: &str) -> Vec<CompanyDetails> {
    let mut reader = Reader::from_reader(xml.as_bytes());
    reader.trim_text(true);

    let mut companies = Vec::new();
    let mut path: Vec<Vec<u8>> = Vec::new();
    let mut current = CompanyDraft::default();

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                path.push(e.name().as_ref().to_vec());
                if e.name() == QName(b"COMPANY") {
                    current = CompanyDraft::default();
                    current.name = extract_name_attr(e);
                }
            }
            Ok(Event::Empty(ref e)) if e.name() == QName(b"COMPANY") => {
                if let Some(name) = extract_name_attr(e) {
                    companies.push(CompanyDetails {
                        name,
                        ..CompanyDetails::empty()
                    });
                }
            }
            Ok(Event::Text(ref e)) => {
                if !in_company(&path) {
                    continue;
                }
                let text = e.unescape().unwrap_or_default().trim().to_string();
                if text.is_empty() {
                    continue;
                }
                match path.last().map(|tag| tag.as_slice()) {
                    Some(b"NAME") if current.name.is_none() => current.name = Some(text),
                    Some(b"BASICCOMPANYFORMALNAME") => current.formal_name = Some(text),
                    Some(b"GUID") => current.guid = Some(text),
                    Some(b"COMPANYNUMBER") => current.company_number = Some(text),
                    Some(b"STARTINGFROM") => current.starting_from = Some(text),
                    Some(b"BOOKSFROM") => current.books_from = Some(text),
                    Some(b"AUDITEDUPTO") => current.audited_upto = Some(text),
                    Some(b"CURRENCYNAME") => current.currency_name = Some(text),
                    Some(b"EMAIL") => current.email = Some(text),
                    Some(b"WEBSITE") => current.website = Some(text),
                    Some(b"PHONENUMBER") => current.phone_number = Some(text),
                    Some(b"FAXNUMBER") | Some(b"CMPFAXNUMBER") => current.fax_number = Some(text),
                    Some(b"ADDRESS") => current.address.push(text),
                    Some(b"STATENAME") => current.state_name = Some(text),
                    Some(b"PRIORSTATENAME") if current.state_name.is_none() => {
                        current.state_name = Some(text)
                    }
                    Some(b"COUNTRYNAME") => current.country_name = Some(text),
                    Some(b"COUNTRYISDCODE") => current.country_isd_code = Some(text),
                    Some(b"PINCODE") => current.pincode = Some(text),
                    Some(b"GSTREGISTRATIONNUMBER") => {
                        current.gst_registration_number = Some(text)
                    }
                    Some(b"GSTREGISTRATIONTYPE") => current.gst_registration_type = Some(text),
                    Some(b"INCOMETAXNUMBER") => current.income_tax_number = Some(text),
                    Some(b"ISGSTON") => current.is_gst_on = parse_yes_no(&text),
                    Some(b"ISACCOUNTINGON") => current.is_accounting_on = parse_yes_no(&text),
                    Some(b"ISINVENTORYON") => current.is_inventory_on = parse_yes_no(&text),
                    Some(b"ISBILLWISEON") => current.is_bill_wise_on = parse_yes_no(&text),
                    Some(b"ISPAYROLLON") => current.is_payroll_on = parse_yes_no(&text),
                    Some(b"ISSECURITYON") => current.is_security_on = parse_yes_no(&text),
                    Some(b"PREVISINVOICINGON") => current.is_invoicing_on = parse_yes_no(&text),
                    Some(b"PREVISMULTICURRENCYON") => {
                        current.is_multi_currency_on = parse_yes_no(&text)
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                if e.name() == QName(b"COMPANY") {
                    if let Some(name) = current.name.take() {
                        companies.push(CompanyDetails {
                            name,
                            formal_name: current.formal_name.take(),
                            guid: current.guid.take(),
                            company_number: current.company_number.take(),
                            starting_from: current.starting_from.take(),
                            books_from: current.books_from.take(),
                            audited_upto: current.audited_upto.take(),
                            currency_name: current.currency_name.take(),
                            email: current.email.take(),
                            website: current.website.take(),
                            phone_number: current.phone_number.take(),
                            fax_number: current.fax_number.take(),
                            address: std::mem::take(&mut current.address),
                            state_name: current.state_name.take(),
                            country_name: current.country_name.take(),
                            country_isd_code: current.country_isd_code.take(),
                            pincode: current.pincode.take(),
                            gst_registration_number: current.gst_registration_number.take(),
                            gst_registration_type: current.gst_registration_type.take(),
                            income_tax_number: current.income_tax_number.take(),
                            is_gst_on: current.is_gst_on,
                            is_accounting_on: current.is_accounting_on,
                            is_inventory_on: current.is_inventory_on,
                            is_bill_wise_on: current.is_bill_wise_on,
                            is_payroll_on: current.is_payroll_on,
                            is_security_on: current.is_security_on,
                            is_invoicing_on: current.is_invoicing_on,
                            is_multi_currency_on: current.is_multi_currency_on,
                        });
                    }
                    current = CompanyDraft::default();
                }
                path.pop();
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
    }

    companies
}

#[derive(Default)]
struct CompanyDraft {
    name: Option<String>,
    formal_name: Option<String>,
    guid: Option<String>,
    company_number: Option<String>,
    starting_from: Option<String>,
    books_from: Option<String>,
    audited_upto: Option<String>,
    currency_name: Option<String>,
    email: Option<String>,
    website: Option<String>,
    phone_number: Option<String>,
    fax_number: Option<String>,
    address: Vec<String>,
    state_name: Option<String>,
    country_name: Option<String>,
    country_isd_code: Option<String>,
    pincode: Option<String>,
    gst_registration_number: Option<String>,
    gst_registration_type: Option<String>,
    income_tax_number: Option<String>,
    is_gst_on: Option<bool>,
    is_accounting_on: Option<bool>,
    is_inventory_on: Option<bool>,
    is_bill_wise_on: Option<bool>,
    is_payroll_on: Option<bool>,
    is_security_on: Option<bool>,
    is_invoicing_on: Option<bool>,
    is_multi_currency_on: Option<bool>,
}

impl CompanyDetails {
    fn empty() -> Self {
        Self {
            name: String::new(),
            formal_name: None,
            guid: None,
            company_number: None,
            starting_from: None,
            books_from: None,
            audited_upto: None,
            currency_name: None,
            email: None,
            website: None,
            phone_number: None,
            fax_number: None,
            address: Vec::new(),
            state_name: None,
            country_name: None,
            country_isd_code: None,
            pincode: None,
            gst_registration_number: None,
            gst_registration_type: None,
            income_tax_number: None,
            is_gst_on: None,
            is_accounting_on: None,
            is_inventory_on: None,
            is_bill_wise_on: None,
            is_payroll_on: None,
            is_security_on: None,
            is_invoicing_on: None,
            is_multi_currency_on: None,
        }
    }
}

fn extract_name_parent_nodes<T>(
    xml: &str,
    tag: &[u8],
    build: impl Fn(String, Option<String>) -> T,
) -> Vec<T> {
    let mut reader = Reader::from_reader(xml.as_bytes());
    reader.trim_text(true);

    let mut rows = Vec::new();
    let mut current_name: Option<String> = None;
    let mut current_parent: Option<String> = None;
    let mut path: Vec<Vec<u8>> = Vec::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                path.push(e.name().as_ref().to_vec());
                if e.name() == QName(tag) {
                    current_name = extract_name_attr(e);
                    current_parent = None;
                }
            }
            Ok(Event::Text(ref e)) => {
                if current_name.is_some()
                    && matches!(path.last().map(|tag| tag.as_slice()), Some(b"PARENT"))
                {
                    let parent = e.unescape().unwrap_or_default().trim().to_string();
                    if !parent.is_empty() {
                        current_parent = Some(parent);
                    }
                }
            }
            Ok(Event::End(ref e)) => {
                if e.name() == QName(tag) {
                    if let Some(name) = current_name.take() {
                        rows.push(build(name, current_parent.take()));
                    }
                }
                path.pop();
            }
            Ok(Event::Empty(ref e)) if e.name() == QName(tag) => {
                if let Some(name) = extract_name_attr(e) {
                    rows.push(build(name, None));
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
    }

    rows
}

fn extract_name_attr(start: &quick_xml::events::BytesStart<'_>) -> Option<String> {
    start
        .attributes()
        .flatten()
        .find(|attr| attr.key == QName(b"NAME"))
        .and_then(|attr| attr.unescape_value().ok())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn push_or_upgrade_currency(currencies: &mut Vec<CurrencySummary>, next: CurrencySummary) {
    if let Some(existing) = currencies.iter_mut().find(|row| row.name == next.name) {
        if currency_detail_score(&next) > currency_detail_score(existing) {
            *existing = next;
        }
        return;
    }
    currencies.push(next);
}

fn currency_detail_score(currency: &CurrencySummary) -> u8 {
    let mut score = 0;
    if currency.original_name.is_some() {
        score += 1;
    }
    if currency.mailing_name.is_some() {
        score += 1;
    }
    if currency.expanded_symbol.is_some() {
        score += 1;
    }
    if currency.decimal_places.is_some() {
        score += 1;
    }
    if currency.guid.is_some() {
        score += 1;
    }
    score
}

fn in_currency(path: &[Vec<u8>]) -> bool {
    path.iter().any(|tag| tag.as_slice() == b"CURRENCY")
}

fn in_company(path: &[Vec<u8>]) -> bool {
    path.iter().any(|tag| tag.as_slice() == b"COMPANY")
}

fn parse_yes_no(text: &str) -> Option<bool> {
    match text.trim() {
        "Yes" | "yes" | "YES" => Some(true),
        "No" | "no" | "NO" => Some(false),
        _ => None,
    }
}

fn parse_i32(text: &str) -> Option<i32> {
    text.trim().parse().ok()
}

#[cfg(test)]
mod tests {
    use super::{
        extract_companies_from_xml, extract_currencies_from_xml, extract_groups_from_xml,
        extract_ledgers_from_xml, extract_stock_items_from_xml,
    };

    #[test]
    fn extracts_unique_currencies() {
        let xml = r#"
<ENVELOPE>
  <CURRENCY NAME="$"></CURRENCY>
  <CURRENCY NAME="INR"></CURRENCY>
  <CURRENCY NAME="$"></CURRENCY>
</ENVELOPE>
"#;

        let rows = extract_currencies_from_xml(xml);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].name, "$");
        assert_eq!(rows[1].name, "INR");
    }

    #[test]
    fn extracts_currency_details() {
        let xml = r#"
<ENVELOPE>
  <CURRENCY NAME="D$">
    <ORIGINALNAME>$</ORIGINALNAME>
    <MAILINGNAME>Dollar</MAILINGNAME>
    <EXPANDEDSYMBOL>Dollar</EXPANDEDSYMBOL>
    <DECIMALSYMBOL>Cent</DECIMALSYMBOL>
    <DECIMALPLACES>2</DECIMALPLACES>
    <ISSUFFIX>No</ISSUFFIX>
    <HASSPACE>No</HASSPACE>
    <INMILLIONS>No</INMILLIONS>
  </CURRENCY>
</ENVELOPE>
"#;

        let rows = extract_currencies_from_xml(xml);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].name, "D$");
        assert_eq!(rows[0].original_name.as_deref(), Some("$"));
        assert_eq!(rows[0].mailing_name.as_deref(), Some("Dollar"));
        assert_eq!(rows[0].expanded_symbol.as_deref(), Some("Dollar"));
        assert_eq!(rows[0].decimal_symbol.as_deref(), Some("Cent"));
        assert_eq!(rows[0].decimal_places, Some(2));
        assert_eq!(rows[0].is_suffix, Some(false));
    }

    #[test]
    fn extracts_company_details_including_base_currency() {
        let xml = r#"
<ENVELOPE>
  <COMPANY NAME="Northwind Traders LLC">
    <BASICCOMPANYFORMALNAME>Northwind Traders LLC</BASICCOMPANYFORMALNAME>
    <CURRENCYNAME>$</CURRENCYNAME>
    <EMAIL>ops@northwind.test</EMAIL>
    <WEBSITE>https://northwind.test</WEBSITE>
    <STATENAME>California</STATENAME>
    <COUNTRYNAME>United States of America</COUNTRYNAME>
    <PINCODE>94105</PINCODE>
    <STARTINGFROM>20240101</STARTINGFROM>
    <BOOKSFROM>20240101</BOOKSFROM>
    <ADDRESS.LIST>
      <ADDRESS>100 Market Street</ADDRESS>
      <ADDRESS>Suite 400</ADDRESS>
    </ADDRESS.LIST>
    <ISGSTON>No</ISGSTON>
    <ISACCOUNTINGON>Yes</ISACCOUNTINGON>
    <PREVISMULTICURRENCYON>No</PREVISMULTICURRENCYON>
  </COMPANY>
</ENVELOPE>
"#;

        let rows = extract_companies_from_xml(xml);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].name, "Northwind Traders LLC");
        assert_eq!(rows[0].currency_name.as_deref(), Some("$"));
        assert_eq!(rows[0].email.as_deref(), Some("ops@northwind.test"));
        assert_eq!(
            rows[0].address,
            vec!["100 Market Street".to_string(), "Suite 400".to_string()]
        );
        assert_eq!(rows[0].is_accounting_on, Some(true));
        assert_eq!(rows[0].is_multi_currency_on, Some(false));
    }

    #[test]
    fn extracts_groups_with_parent() {
        let xml = r#"
<ENVELOPE>
  <GROUP NAME="Child">
    <PARENT>Parent</PARENT>
  </GROUP>
  <GROUP NAME="Root">
    <PARENT></PARENT>
  </GROUP>
</ENVELOPE>
"#;

        let rows = extract_groups_from_xml(xml);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].name, "Child");
        assert_eq!(rows[0].parent.as_deref(), Some("Parent"));
        assert_eq!(rows[1].name, "Root");
        assert_eq!(rows[1].parent, None);
    }

    #[test]
    fn extracts_ledgers_and_stock_items() {
        let ledgers_xml = r#"<LEDGER NAME="Cash"><PARENT>Current Assets</PARENT></LEDGER>"#;
        let items_xml = r#"<STOCKITEM NAME="Keyboard"><PARENT>Peripherals</PARENT></STOCKITEM>"#;

        let ledgers = extract_ledgers_from_xml(ledgers_xml);
        let items = extract_stock_items_from_xml(items_xml);

        assert_eq!(ledgers[0].name, "Cash");
        assert_eq!(ledgers[0].parent.as_deref(), Some("Current Assets"));
        assert_eq!(items[0].name, "Keyboard");
        assert_eq!(items[0].parent.as_deref(), Some("Peripherals"));
    }
}
