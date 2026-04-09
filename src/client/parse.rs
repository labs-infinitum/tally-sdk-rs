use crate::models::ImportResult;
use quick_xml::events::Event;
use quick_xml::Reader;

pub fn parse_simple_response_public(xml: &str) -> ImportResult {
    let mut reader = Reader::from_reader(xml.as_bytes());
    reader.trim_text(true);

    let mut current_tag: Option<Vec<u8>> = None;
    let mut result = ImportResult::default();

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                current_tag = Some(e.name().as_ref().to_vec());
            }
            Ok(Event::Text(ref e)) => {
                let Some(tag) = current_tag.as_deref() else {
                    continue;
                };
                let text = e.unescape().unwrap_or_default().trim().to_string();
                if text.is_empty() {
                    continue;
                }

                match tag {
                    b"CREATED" => result.created = parse_counter(&text),
                    b"ALTERED" => result.altered = parse_counter(&text),
                    b"DELETED" => result.deleted = parse_counter(&text),
                    b"COMBINED" => result.combined = parse_counter(&text),
                    b"IGNORED" => result.ignored = parse_counter(&text),
                    b"ERRORS" => result.errors = parse_counter(&text),
                    b"CANCELLED" => result.cancelled = parse_counter(&text),
                    b"EXCEPTIONS" => result.exceptions = parse_counter(&text),
                    b"LASTVCHID" => result.last_voucher_id = Some(text),
                    b"LASTMID" => result.last_master_id = Some(text),
                    b"LINEERROR" => result.line_errors.push(text),
                    _ => {}
                }
            }
            Ok(Event::End(_)) => {
                current_tag = None;
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
    }

    result
}

fn parse_counter(text: &str) -> i64 {
    text.parse::<i64>().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::parse_simple_response_public;

    #[test]
    fn parses_import_result_counters_and_line_errors() {
        let xml = r#"
<ENVELOPE>
  <BODY>
    <DATA>
      <IMPORTRESULT>
        <CREATED>1</CREATED>
        <ALTERED>0</ALTERED>
        <ERRORS>1</ERRORS>
        <LINEERROR>Could not find Company ''</LINEERROR>
        <LINEERROR>Voucher number missing</LINEERROR>
      </IMPORTRESULT>
    </DATA>
  </BODY>
</ENVELOPE>
"#;

        let value = parse_simple_response_public(xml);
        assert_eq!(value.created, 1);
        assert_eq!(value.altered, 0);
        assert_eq!(value.errors, 1);
        assert_eq!(value.line_errors[0], "Could not find Company ''");
        assert_eq!(value.line_errors[1], "Voucher number missing");
    }
}
