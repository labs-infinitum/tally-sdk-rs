use regex::Regex;

pub fn parse_simple_response_public(xml: &str) -> serde_json::Value {
    // Handle both <RESPONSE> and ENVELOPE IMPORTRESULT blocks using regex extras like Python's fallback
    let cleaned = clean_xml(xml);
    // Try to pull common tally counters
    let keys = [
        "CREATED","ALTERED","DELETED","LASTVCHID","LASTMID","COMBINED","IGNORED","ERRORS","CANCELLED","EXCEPTIONS"
    ];
    let mut obj = serde_json::Map::new();
    for k in keys.iter() {
        if let Some(v) = capture_tag(&cleaned, k) { obj.insert((*k).into(), serde_json::Value::String(v)); }
    }
    if let Ok(re) = Regex::new(r"<LINEERROR>(.*?)</LINEERROR>") {
        let mut errors: Vec<serde_json::Value> = Vec::new();
        for cap in re.captures_iter(&cleaned) {
            if let Some(m) = cap.get(1) { errors.push(serde_json::Value::String(m.as_str().to_string())); }
        }
        if !errors.is_empty() { obj.insert("LINEERROR".into(), serde_json::Value::Array(errors)); }
    }
    serde_json::Value::Object(obj)
}

fn capture_tag(xml: &str, tag: &str) -> Option<String> {
    let re = Regex::new(&format!(r"<{0}>(.*?)</{0}>", regex::escape(tag))).ok()?;
    re.captures(xml).and_then(|c| c.get(1)).map(|m| m.as_str().to_string())
}

fn clean_xml(s: &str) -> String {
    // remove numeric char refs like &#4;
    let re = Regex::new(r"&#\d+;").unwrap();
    let cleaned = re.replace_all(s, "");
    cleaned.to_string()
}
