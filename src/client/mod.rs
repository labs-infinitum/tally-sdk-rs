use crate::config::TallyConfig;
use crate::errors::{Result, TallyError};
use crate::xml_builder::XmlBuilder;
use std::sync::Mutex;

mod company;
mod extract;
mod http;
mod masters;
pub mod parse;
mod report_parser;
mod reports;
pub mod voucher_parser;
mod vouchers;

pub struct TallyClient {
    cfg: TallyConfig,
    http: reqwest::blocking::Client,
    base_url: String,
    current_company: Mutex<Option<String>>,
}

impl TallyClient {
    pub fn new(cfg: TallyConfig) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("text/xml"),
        );
        headers.insert(
            reqwest::header::CACHE_CONTROL,
            reqwest::header::HeaderValue::from_static("no-cache"),
        );
        if let (Some(acc), Some(pw)) = (&cfg.tally_net_account, &cfg.tally_net_password) {
            headers.insert(
                "X-Tally-Account",
                reqwest::header::HeaderValue::from_str(acc)
                    .unwrap_or(reqwest::header::HeaderValue::from_static("")),
            );
            headers.insert(
                "X-Tally-Password",
                reqwest::header::HeaderValue::from_str(pw)
                    .unwrap_or(reqwest::header::HeaderValue::from_static("")),
            );
        }

        let http = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(cfg.timeout_secs))
            .build()
            .map_err(|e| TallyError::Unexpected(e.to_string()))?;
        let base_url = format!("http://{}:{}", cfg.host, cfg.port);
        Ok(Self {
            current_company: Mutex::new(cfg.current_company.clone()),
            cfg,
            http,
            base_url,
        })
    }

    pub fn test_connection(&self) -> Result<bool> {
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
        let _resp = self.post_xml(&xml)?;
        Ok(true)
    }
}

pub use parse::parse_simple_response_public as parse_simple_response;
