use crate::models::{CurrencySummary, GroupSummary, LedgerSummary, StockItemSummary};
use regex::Regex;

pub(crate) fn extract_groups_from_xml(xml: &str) -> Vec<GroupSummary> {
    extract_name_parent_nodes(xml, "GROUP", |name, parent| GroupSummary { name, parent })
}

pub(crate) fn extract_ledgers_from_xml(xml: &str) -> Vec<LedgerSummary> {
    extract_name_parent_nodes(xml, "LEDGER", |name, parent| LedgerSummary { name, parent })
}

pub(crate) fn extract_stock_items_from_xml(xml: &str) -> Vec<StockItemSummary> {
    extract_name_parent_nodes(xml, "STOCKITEM", |name, parent| StockItemSummary {
        name,
        parent,
    })
}

pub(crate) fn extract_currencies_from_xml(xml: &str) -> Vec<CurrencySummary> {
    let currency_re = Regex::new(r#"<CURRENCY\b[^>]*\bNAME="([^"]+)""#).ok();
    let mut rows = Vec::new();

    if let Some(currency_re) = currency_re {
        for caps in currency_re.captures_iter(xml) {
            if let Some(name) = caps.get(1).map(|m| m.as_str().trim().to_string()) {
                if !name.is_empty()
                    && !rows
                        .iter()
                        .any(|currency: &CurrencySummary| currency.name == name)
                {
                    rows.push(CurrencySummary { name });
                }
            }
        }
    }

    rows
}

fn extract_name_parent_nodes<T>(
    xml: &str,
    tag: &str,
    build: impl Fn(String, Option<String>) -> T,
) -> Vec<T> {
    let node_re = Regex::new(&format!(
        r#"<{tag}[^>]*\bNAME=\"([^\"]+)\"[^>]*>(?s)(.*?)</{tag}>"#
    ))
    .ok();
    let parent_re = Regex::new(r"<PARENT(?: [^>]*)?>([^<]*)</PARENT>").ok();

    let mut rows = Vec::new();
    if let Some(node_re) = node_re {
        for cap in node_re.captures_iter(xml) {
            let name = cap
                .get(1)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let inner = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let parent = parent_re
                .as_ref()
                .and_then(|re| re.captures(inner))
                .and_then(|captures| captures.get(1))
                .map(|m| m.as_str().trim().to_string())
                .filter(|parent| !parent.is_empty());
            rows.push(build(name, parent));
        }
    }

    rows
}

#[cfg(test)]
mod tests {
    use super::extract_currencies_from_xml;

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
}
