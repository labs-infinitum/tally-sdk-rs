use crate::errors::{Result, TallyError};
use serde_json::Value;

use super::XmlBuilder;

impl XmlBuilder {
    pub fn create_export_request(
        request_type: &str,
        collection_type: &str,
        static_variables: Option<&serde_json::Map<String, Value>>,
    ) -> Result<String> {
        let id_value = match collection_type.to_lowercase().as_str() {
            "voucher" => "DayBook", // No space, as per TallyPrime API
            "company" | "group" | "ledger" => collection_type,
            _ => &format!("List of {}s", collection_type),
        };

        let mut header = serde_json::Map::new();
        header.insert("VERSION".into(), Value::String("1".into()));
        header.insert("TALLYREQUEST".into(), Value::String("EXPORT".into()));
        header.insert("TYPE".into(), Value::String(request_type.to_string()));
        header.insert("ID".into(), Value::String(id_value.to_string()));

        let mut desc = serde_json::Map::new();
        let mut stat = serde_json::Map::new();
        stat.insert(
            "SVEXPORTFORMAT".into(),
            Value::String("$$SysName:XML".into()),
        );
        if let Some(sv) = static_variables {
            for (k, v) in sv {
                if k != "FETCHLIST" {
                    stat.insert(k.clone(), v.clone());
                }
            }
        }
        desc.insert("STATICVARIABLES".into(), Value::Object(stat));
        // Only include FETCHLIST for true collections, not for Day Book export
        if collection_type.to_lowercase().as_str() != "voucher" {
            if let Some(sv) = static_variables {
                if let Some(fetch) = sv.get("FETCHLIST") {
                    desc.insert("FETCHLIST".into(), fetch.clone());
                }
            }
        }

        let mut body = serde_json::Map::new();
        body.insert("DESC".into(), Value::Object(desc));
        XmlBuilder::create_envelope(&header, Some(&body))
    }

    pub fn create_voucher_export_request(
        from_date: Option<&str>,
        to_date: Option<&str>,
        current_company: Option<&str>,
    ) -> Result<String> {
        let from_date = from_date.unwrap_or("19000101");
        let to_date = to_date.unwrap_or("99991231");
        let from_date = format_tally_static_date(from_date)?;
        let to_date = format_tally_static_date(to_date)?;

        let mut static_variables = String::from(
            "<SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>\n\
             <SVFROMDATE TYPE=\"Date\">",
        );
        static_variables.push_str(&from_date);
        static_variables.push_str("</SVFROMDATE>\n<SVTODATE TYPE=\"Date\">");
        static_variables.push_str(&to_date);
        static_variables.push_str("</SVTODATE>");

        if let Some(company) = current_company.filter(|name| !name.trim().is_empty()) {
            static_variables.push_str("\n<SVCURRENTCOMPANY>");
            static_variables.push_str(&XmlBuilder::escape_simple(company.trim()));
            static_variables.push_str("</SVCURRENTCOMPANY>");
        }

        Ok(format!(
            "<ENVELOPE>\
<HEADER>\
<VERSION>1</VERSION>\
<TALLYREQUEST>EXPORT</TALLYREQUEST>\
<TYPE>COLLECTION</TYPE>\
<ID>All Vouchers</ID>\
</HEADER>\
<BODY>\
<DESC>\
<STATICVARIABLES>{static_variables}</STATICVARIABLES>\
<TDL>\
<TDLMESSAGE>\
<COLLECTION NAME=\"AllVouchers\" ISFIXED=\"Yes\" FETCHALLFIELDS=\"Yes\">\
<TYPE>Voucher</TYPE>\
<NATIVEMETHOD>*, *.*</NATIVEMETHOD>\
<FETCH>Date</FETCH>\
<FETCH>VoucherNumber</FETCH>\
<FETCH>VoucherTypeName</FETCH>\
<FETCH>Amount</FETCH>\
<FETCH>MasterID</FETCH>\
<FETCH>PartyLedgerName</FETCH>\
</COLLECTION>\
</TDLMESSAGE>\
</TDL>\
</DESC>\
</BODY>\
</ENVELOPE>"
        ))
    }
}

fn format_tally_static_date(date: &str) -> Result<String> {
    if date.len() == 11
        && date.as_bytes().get(2) == Some(&b'-')
        && date.as_bytes().get(6) == Some(&b'-')
    {
        return Ok(date.to_string());
    }

    if date.len() != 8 || !date.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(TallyError::Validation(format!(
            "Invalid voucher date `{date}`. Expected YYYYMMDD or DD-MMM-YYYY."
        )));
    }

    let month = match &date[4..6] {
        "01" => "Jan",
        "02" => "Feb",
        "03" => "Mar",
        "04" => "Apr",
        "05" => "May",
        "06" => "Jun",
        "07" => "Jul",
        "08" => "Aug",
        "09" => "Sep",
        "10" => "Oct",
        "11" => "Nov",
        "12" => "Dec",
        _ => {
            return Err(TallyError::Validation(format!(
                "Invalid voucher date `{date}`. Month must be between 01 and 12."
            )))
        }
    };

    Ok(format!("{}-{}-{}", &date[6..8], month, &date[0..4]))
}

#[cfg(test)]
mod tests {
    use super::XmlBuilder;

    #[test]
    fn voucher_export_uses_tdl_collection_and_date_static_vars() {
        let xml = XmlBuilder::create_voucher_export_request(
            Some("20250401"),
            Some("20250731"),
            Some("Example Company"),
        )
        .expect("voucher export request");

        assert!(xml.contains("<TYPE>COLLECTION</TYPE>"));
        assert!(xml.contains("<ID>All Vouchers</ID>"));
        assert!(xml.contains("<SVFROMDATE TYPE=\"Date\">01-Apr-2025</SVFROMDATE>"));
        assert!(xml.contains("<SVTODATE TYPE=\"Date\">31-Jul-2025</SVTODATE>"));
        assert!(xml.contains("<SVCURRENTCOMPANY>Example Company</SVCURRENTCOMPANY>"));
        assert!(xml
            .contains("<COLLECTION NAME=\"AllVouchers\" ISFIXED=\"Yes\" FETCHALLFIELDS=\"Yes\">"));
        assert!(xml.contains("<TYPE>Voucher</TYPE>"));
        assert!(xml.contains("<NATIVEMETHOD>*, *.*</NATIVEMETHOD>"));
        assert!(xml.contains("<FETCH>Amount</FETCH>"));
        assert!(xml.contains("<FETCH>PartyLedgerName</FETCH>"));
    }
}
