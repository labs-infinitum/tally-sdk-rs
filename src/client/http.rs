use super::TallyClient;
use crate::errors::{Result, TallyError};

impl TallyClient {
    pub fn post_xml(&self, xml: &str) -> Result<String> {
        let prepared = self.prepare_request_xml(xml)?;
        self.post_raw_xml(&prepared)
    }

    pub(crate) fn post_raw_xml(&self, xml: &str) -> Result<String> {
        let mut last_err: Option<TallyError> = None;
        for _ in 0..self.cfg.retry_attempts {
            match self.http.post(&self.base_url).body(xml.to_string()).send() {
                Ok(resp) => {
                    let status = resp.status();
                    match resp.text() {
                        Ok(text) => {
                            if status.is_success() {
                                return Ok(text);
                            } else {
                                return Err(TallyError::Http(format!(
                                    "HTTP {}: {}",
                                    status.as_u16(),
                                    text
                                )));
                            }
                        }
                        Err(e) => return Err(TallyError::Http(e.to_string())),
                    }
                }
                Err(e) => {
                    last_err = Some(TallyError::Connection(e.to_string()));
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
            }
        }
        Err(last_err.unwrap_or_else(|| TallyError::Connection("Request failed".into())))
    }
}
