use super::extract::extract_companies_from_xml;
use super::TallyClient;
use crate::errors::{Result, TallyError};
use crate::models::CompanyDetails;
use crate::xml_builder::XmlBuilder;
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;

impl TallyClient {
    /// Return the active/configured company name, if one can be resolved.
    pub fn active_company_name(&self) -> Result<Option<String>> {
        self.current_company_name()
    }

    /// Fetch detailed company master data for all companies currently available in Tally.
    pub fn get_companies(&self) -> Result<Vec<CompanyDetails>> {
        let current_company = self.current_company_name()?;
        let xml = XmlBuilder::create_company_export_request(current_company.as_deref())?;
        let resp = self.post_xml(&xml)?;
        Ok(extract_companies_from_xml(&resp))
    }

    /// Fetch detailed master data for the active/configured company.
    pub fn get_company_details(&self) -> Result<Option<CompanyDetails>> {
        let active = self.current_company_name()?;
        let companies = self.get_companies()?;
        if let Some(active_name) = active {
            return Ok(companies
                .into_iter()
                .find(|company| company.name.eq_ignore_ascii_case(&active_name)));
        }
        Ok(companies.into_iter().next())
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
        let xml = XmlBuilder::create_company_list_export_request()?;
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
