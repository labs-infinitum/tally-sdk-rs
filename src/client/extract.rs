use crate::models::{CurrencySummary, GroupSummary, LedgerSummary, StockItemSummary};
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;
use std::collections::BTreeSet;

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
    let mut seen = BTreeSet::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) if e.name() == QName(b"CURRENCY") => {
                if let Some(name) = extract_name_attr(e) {
                    if seen.insert(name.clone()) {
                        currencies.push(CurrencySummary { name });
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
    }

    currencies
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

#[cfg(test)]
mod tests {
    use super::{
        extract_currencies_from_xml, extract_groups_from_xml, extract_ledgers_from_xml,
        extract_stock_items_from_xml,
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
