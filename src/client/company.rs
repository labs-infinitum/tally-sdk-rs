use super::TallyClient;
use crate::errors::{Result, TallyError};
use crate::xml_builder::XmlBuilder;
use regex::Regex;

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

        let company_attr = Regex::new(r#"<COMPANY[^>]*\bNAME="([^"]+)""#)
            .ok()
            .and_then(|re| re.captures(&resp))
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().trim().to_string())
            .filter(|name| !name.is_empty());
        if company_attr.is_some() {
            return Ok(company_attr);
        }

        let company_tag = Regex::new(r"(?s)<COMPANY(?:\s[^>]*)?>.*?<NAME>(.*?)</NAME>")
            .ok()
            .and_then(|re| re.captures(&resp))
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().trim().to_string())
            .filter(|name| !name.is_empty());
        Ok(company_tag)
    }
}
