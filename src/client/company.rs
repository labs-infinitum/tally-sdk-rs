use super::TallyClient;
use crate::errors::{Result, TallyError};
use crate::xml_builder::XmlBuilder;
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;

impl TallyClient {
    pub fn active_company_name(&self) -> Result<Option<String>> {
        self.current_company_name()
    }

    pub(crate) fn prepare_request_xml(&self, xml: &str) -> Result<String> {
        if xml.contains("<SVCURRENTCOMPANY>") {
            return Ok(xml.to_string());
        }

        if !xml.contains("<TALLYREQUEST>Import Data</TALLYREQUEST>") {
            return Ok(xml.to_string());
        }

        let company = self.current_company_name()?.ok_or_else(|| {
            TallyError::Validation(
                "No active Tally company is available. Load a company in Tally or set `current_company`/`TALLY_COMPANY`."
                    .into(),
            )
        })?;

        if xml.contains("<REQUESTDESC>") && xml.contains("</REQUESTDESC>") {
            let static_vars = format!(
                "<STATICVARIABLES><SVCURRENTCOMPANY>{}</SVCURRENTCOMPANY></STATICVARIABLES>",
                XmlBuilder::escape_simple(&company)
            );
            return Ok(xml.replacen("</REQUESTDESC>", &(static_vars + "</REQUESTDESC>"), 1));
        }

        Ok(xml.to_string())
    }

    pub(crate) fn current_company_name(&self) -> Result<Option<String>> {
        if let Ok(cache) = self.current_company.lock() {
            if let Some(company) = cache.clone() {
                return Ok(Some(company));
            }
        }

        let company = self.discover_current_company()?;
        if let Some(ref discovered) = company {
            if let Ok(mut cache) = self.current_company.lock() {
                *cache = Some(discovered.clone());
            }
        }
        Ok(company)
    }

    fn discover_current_company(&self) -> Result<Option<String>> {
        let header = serde_json::json!({
            "VERSION": "1",
            "TALLYREQUEST": "EXPORT",
            "TYPE": "COLLECTION",
            "ID": "Company",
        })
        .as_object()
        .unwrap()
        .clone();
        let mut body = serde_json::Map::new();
        let mut desc = serde_json::Map::new();
        let mut stat = serde_json::Map::new();
        stat.insert(
            "SVEXPORTFORMAT".into(),
            serde_json::Value::String("$$SysName:XML".into()),
        );
        desc.insert("STATICVARIABLES".into(), serde_json::Value::Object(stat));
        body.insert("DESC".into(), serde_json::Value::Object(desc));
        let xml = XmlBuilder::create_envelope(&header, Some(&body))?;
        let resp = self.post_raw_xml(&xml)?;
        Ok(parse_current_company_name(&resp))
    }
}

fn parse_current_company_name(xml: &str) -> Option<String> {
    let mut reader = Reader::from_reader(xml.as_bytes());
    reader.trim_text(true);

    let mut in_company = false;
    let mut current_tag: Option<Vec<u8>> = None;

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                let tag = e.name();
                if tag == QName(b"COMPANY") {
                    in_company = true;
                    if let Some(name) = e
                        .attributes()
                        .flatten()
                        .find(|attr| attr.key == QName(b"NAME"))
                        .and_then(|attr| attr.unescape_value().ok())
                        .map(|value| value.trim().to_string())
                        .filter(|value| !value.is_empty())
                    {
                        return Some(name);
                    }
                }
                current_tag = Some(tag.as_ref().to_vec());
            }
            Ok(Event::Empty(ref e)) => {
                if e.name() == QName(b"COMPANY") {
                    if let Some(name) = e
                        .attributes()
                        .flatten()
                        .find(|attr| attr.key == QName(b"NAME"))
                        .and_then(|attr| attr.unescape_value().ok())
                        .map(|value| value.trim().to_string())
                        .filter(|value| !value.is_empty())
                    {
                        return Some(name);
                    }
                }
            }
            Ok(Event::Text(ref e)) => {
                if in_company && current_tag.as_deref() == Some(b"NAME") {
                    let text = e.unescape().unwrap_or_default().trim().to_string();
                    if !text.is_empty() {
                        return Some(text);
                    }
                }
            }
            Ok(Event::End(ref e)) => {
                if e.name() == QName(b"COMPANY") {
                    in_company = false;
                }
                current_tag = None;
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::parse_current_company_name;

    #[test]
    fn parses_company_name_from_attribute_or_nested_name() {
        let from_attr =
            r#"<ENVELOPE><BODY><DATA><COMPANY NAME="ACME LLP" /></DATA></BODY></ENVELOPE>"#;
        assert_eq!(
            parse_current_company_name(from_attr).as_deref(),
            Some("ACME LLP")
        );

        let from_name = r#"
<ENVELOPE>
  <BODY>
    <DATA>
      <COMPANY>
        <NAME>Okeanos Software Solutions Private Limited</NAME>
      </COMPANY>
    </DATA>
  </BODY>
</ENVELOPE>
"#;
        assert_eq!(
            parse_current_company_name(from_name).as_deref(),
            Some("Okeanos Software Solutions Private Limited")
        );
    }
}
