use regex::Regex;

pub(crate) fn extract_groups_from_xml(xml: &str) -> Vec<(String, Option<String>)> {
    let mut out = Vec::new();
    // Capture GROUP with NAME and inner block
    let re_group = Regex::new(r#"<GROUP[^>]*\bNAME=\"([^\"]+)\"[^>]*>(?s)(.*?)</GROUP>"#).ok();
    // Parent can be empty/self-closed or contain text
    let re_parent_text = Regex::new(r"<PARENT(?: [^>]*)?>([^<]*)</PARENT>").ok();
    if let Some(rg) = re_group {
        for cap in rg.captures_iter(xml) {
            let name = cap
                .get(1)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let inner = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let parent = if let Some(rpt) = &re_parent_text {
                rpt.captures(inner)
                    .and_then(|c| c.get(1))
                    .map(|m| m.as_str().to_string())
            } else {
                None
            };
            let parent = match parent {
                Some(ref p) if p.trim().is_empty() => None,
                other => other,
            };
            out.push((name, parent));
        }
    }
    out
}

pub(crate) fn extract_ledgers_from_xml(xml: &str) -> Vec<(String, Option<String>)> {
    let mut out = Vec::new();
    let re_ledger = Regex::new(r#"<LEDGER[^>]*\bNAME=\"([^\"]+)\"[^>]*>(?s)(.*?)</LEDGER>"#).ok();
    let re_parent_text = Regex::new(r"<PARENT(?: [^>]*)?>([^<]*)</PARENT>").ok();
    if let Some(rl) = re_ledger {
        for cap in rl.captures_iter(xml) {
            let name = cap
                .get(1)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let inner = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let parent = if let Some(rpt) = &re_parent_text {
                rpt.captures(inner)
                    .and_then(|c| c.get(1))
                    .map(|m| m.as_str().to_string())
            } else {
                None
            };
            let parent = parent.and_then(|p| if p.trim().is_empty() { None } else { Some(p) });
            out.push((name, parent));
        }
    }
    out
}

pub(crate) fn extract_stock_items_from_xml(xml: &str) -> Vec<(String, Option<String>)> {
    let mut out = Vec::new();
    let re_item =
        Regex::new(r#"<STOCKITEM[^>]*\bNAME=\"([^\"]+)\"[^>]*>(?s)(.*?)</STOCKITEM>"#).ok();
    let re_parent_text = Regex::new(r"<PARENT(?: [^>]*)?>([^<]*)</PARENT>").ok();
    if let Some(ri) = re_item {
        for cap in ri.captures_iter(xml) {
            let name = cap
                .get(1)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let inner = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let parent = if let Some(rpt) = &re_parent_text {
                rpt.captures(inner)
                    .and_then(|c| c.get(1))
                    .map(|m| m.as_str().to_string())
            } else {
                None
            };
            let parent = parent.and_then(|p| if p.trim().is_empty() { None } else { Some(p) });
            out.push((name, parent));
        }
    }
    out
}
