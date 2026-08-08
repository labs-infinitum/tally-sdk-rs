use super::TallyClient;
use crate::errors::{Result, TallyError};

impl TallyClient {
    /// POST raw XML to Tally after injecting the current company when needed.
    ///
    /// Prefer typed client methods when available. Use this for custom TDL /
    /// report XML built with [`crate::xml_builder::XmlBuilder`].
    pub fn post_xml(&self, xml: &str) -> Result<String> {
        let prepared = self.prepare_request_xml(xml)?;
        self.post_raw_xml(&prepared)
    }

    pub(crate) fn post_raw_xml(&self, xml: &str) -> Result<String> {
        let body = encode_tally_xml(xml);
        let mut last_err: Option<TallyError> = None;
        for _ in 0..self.cfg.retry_attempts {
            match self.http.post(&self.base_url).body(body.clone()).send() {
                Ok(resp) => {
                    let status = resp.status();
                    match resp.bytes() {
                        Ok(bytes) => {
                            let text = decode_tally_xml(&bytes)?;
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

/// Tally returns currency symbols like `₹` only when the request is UTF-16.
fn encode_tally_xml(xml: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(xml.len() * 2);
    for unit in xml.encode_utf16() {
        out.extend_from_slice(&unit.to_le_bytes());
    }
    out
}

fn decode_tally_xml(bytes: &[u8]) -> Result<String> {
    if looks_like_utf16_le(bytes) {
        return decode_utf16_le(bytes);
    }
    if looks_like_utf16_be(bytes) {
        return decode_utf16_be(bytes);
    }

    String::from_utf8(bytes.to_vec()).map_err(|err| TallyError::Unexpected(err.to_string()))
}

fn looks_like_utf16_le(bytes: &[u8]) -> bool {
    bytes.starts_with(&[0xFF, 0xFE])
        || (bytes.len() >= 4 && bytes[1] == 0 && bytes[3] == 0 && bytes[0] != 0)
}

fn looks_like_utf16_be(bytes: &[u8]) -> bool {
    bytes.starts_with(&[0xFE, 0xFF])
        || (bytes.len() >= 4 && bytes[0] == 0 && bytes[2] == 0 && bytes[1] != 0)
}

fn decode_utf16_le(bytes: &[u8]) -> Result<String> {
    let data = if bytes.starts_with(&[0xFF, 0xFE]) {
        &bytes[2..]
    } else {
        bytes
    };
    if data.len() % 2 != 0 {
        return Err(TallyError::Unexpected(
            "Invalid UTF-16LE response from Tally".into(),
        ));
    }

    let units: Vec<u16> = data
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();
    String::from_utf16(&units).map_err(|err| TallyError::Unexpected(err.to_string()))
}

fn decode_utf16_be(bytes: &[u8]) -> Result<String> {
    let data = if bytes.starts_with(&[0xFE, 0xFF]) {
        &bytes[2..]
    } else {
        bytes
    };
    if data.len() % 2 != 0 {
        return Err(TallyError::Unexpected(
            "Invalid UTF-16BE response from Tally".into(),
        ));
    }

    let units: Vec<u16> = data
        .chunks_exact(2)
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
        .collect();
    String::from_utf16(&units).map_err(|err| TallyError::Unexpected(err.to_string()))
}

#[cfg(test)]
mod tests {
    use super::{decode_tally_xml, encode_tally_xml};

    #[test]
    fn round_trips_utf16_with_rupee_symbol() {
        let xml = r#"<CURRENCY NAME="₹"><MAILINGNAME>INR</MAILINGNAME></CURRENCY>"#;
        let encoded = encode_tally_xml(xml);
        assert_eq!(&encoded[..4], &[b'<', 0, b'C', 0]);

        let decoded = decode_tally_xml(&encoded).expect("decode");
        assert!(decoded.contains('₹'));
        assert!(decoded.contains("INR"));
    }
}
