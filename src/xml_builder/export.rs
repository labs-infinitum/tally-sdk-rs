use crate::errors::{Result, TallyError};
use quick_xml::Writer;
use serde_json::Value;
use std::io::Cursor;

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
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        let from_date = format_tally_static_date(from_date.unwrap_or("19000101"))?;
        let to_date = format_tally_static_date(to_date.unwrap_or("99991231"))?;

        XmlBuilder::write_export_envelope_header(
            &mut writer,
            "EXPORT",
            "COLLECTION",
            "All Vouchers",
        )?;
        XmlBuilder::write_start_tag(&mut writer, "BODY")?;
        XmlBuilder::write_start_tag(&mut writer, "DESC")?;
        XmlBuilder::write_start_tag(&mut writer, "STATICVARIABLES")?;
        XmlBuilder::write_text_node(&mut writer, "SVEXPORTFORMAT", "$$SysName:XML")?;
        XmlBuilder::write_text_node_with_attrs(
            &mut writer,
            "SVFROMDATE",
            &from_date,
            &[("TYPE", "Date")],
        )?;
        XmlBuilder::write_text_node_with_attrs(
            &mut writer,
            "SVTODATE",
            &to_date,
            &[("TYPE", "Date")],
        )?;
        XmlBuilder::write_current_company_tag(&mut writer, current_company)?;
        XmlBuilder::write_end_tag(&mut writer, "STATICVARIABLES")?;
        XmlBuilder::write_start_tag(&mut writer, "TDL")?;
        XmlBuilder::write_start_tag(&mut writer, "TDLMESSAGE")?;
        XmlBuilder::write_start_tag_with_attrs(
            &mut writer,
            "COLLECTION",
            &[
                ("NAME", "AllVouchers"),
                ("ISFIXED", "Yes"),
                ("FETCHALLFIELDS", "Yes"),
            ],
        )?;
        XmlBuilder::write_text_node(&mut writer, "TYPE", "Voucher")?;
        XmlBuilder::write_text_node(&mut writer, "NATIVEMETHOD", "*, *.*")?;
        for field in [
            "Date",
            "VoucherNumber",
            "VoucherTypeName",
            "Amount",
            "MasterID",
            "PartyLedgerName",
        ] {
            XmlBuilder::write_text_node(&mut writer, "FETCH", field)?;
        }
        XmlBuilder::write_end_tag(&mut writer, "COLLECTION")?;
        XmlBuilder::write_end_tag(&mut writer, "TDLMESSAGE")?;
        XmlBuilder::write_end_tag(&mut writer, "TDL")?;
        XmlBuilder::write_end_tag(&mut writer, "DESC")?;
        XmlBuilder::write_end_tag(&mut writer, "BODY")?;
        XmlBuilder::write_end_tag(&mut writer, "ENVELOPE")?;

        XmlBuilder::finish_writer(writer)
    }

    pub fn create_currency_export_request(current_company: Option<&str>) -> Result<String> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        XmlBuilder::write_export_envelope_header(
            &mut writer,
            "EXPORT",
            "COLLECTION",
            "CustomCurrencyCollection",
        )?;
        XmlBuilder::write_start_tag(&mut writer, "BODY")?;
        XmlBuilder::write_start_tag(&mut writer, "DESC")?;
        XmlBuilder::write_start_tag(&mut writer, "STATICVARIABLES")?;
        XmlBuilder::write_text_node(&mut writer, "SVEXPORTFORMAT", "$$SysName:XML")?;
        XmlBuilder::write_current_company_tag(&mut writer, current_company)?;
        XmlBuilder::write_end_tag(&mut writer, "STATICVARIABLES")?;
        XmlBuilder::write_start_tag(&mut writer, "TDL")?;
        XmlBuilder::write_start_tag(&mut writer, "TDLMESSAGE")?;
        XmlBuilder::write_start_tag_with_attrs(
            &mut writer,
            "COLLECTION",
            &[
                ("NAME", "CustomCurrencyCollection"),
                ("ISMODIFY", "No"),
                ("ISFIXED", "No"),
                ("ISINITIALIZE", "No"),
                ("ISOPTION", "No"),
                ("ISINTERNAL", "No"),
            ],
        )?;
        XmlBuilder::write_text_node(&mut writer, "TYPE", "CURRENCY")?;
        XmlBuilder::write_end_tag(&mut writer, "COLLECTION")?;
        XmlBuilder::write_end_tag(&mut writer, "TDLMESSAGE")?;
        XmlBuilder::write_end_tag(&mut writer, "TDL")?;
        XmlBuilder::write_end_tag(&mut writer, "DESC")?;
        XmlBuilder::write_end_tag(&mut writer, "BODY")?;
        XmlBuilder::write_end_tag(&mut writer, "ENVELOPE")?;

        XmlBuilder::finish_writer(writer)
    }

    pub fn create_builtin_report_request(
        report_name: &str,
        from_date: Option<&str>,
        to_date: Option<&str>,
        current_company: Option<&str>,
        explode_flag: bool,
    ) -> Result<String> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        XmlBuilder::write_export_envelope_header(&mut writer, "Export", "Data", report_name)?;
        XmlBuilder::write_start_tag(&mut writer, "BODY")?;
        XmlBuilder::write_start_tag(&mut writer, "DESC")?;
        XmlBuilder::write_start_tag(&mut writer, "STATICVARIABLES")?;
        XmlBuilder::write_text_node(&mut writer, "SVEXPORTFORMAT", "$$SysName:XML")?;

        if explode_flag {
            XmlBuilder::write_text_node(&mut writer, "EXPLODEFLAG", "Yes")?;
        }
        if let Some(from) = from_date {
            XmlBuilder::write_text_node_with_attrs(
                &mut writer,
                "SVFROMDATE",
                &format_tally_static_date(from)?,
                &[("TYPE", "Date")],
            )?;
        }
        if let Some(to) = to_date {
            XmlBuilder::write_text_node_with_attrs(
                &mut writer,
                "SVTODATE",
                &format_tally_static_date(to)?,
                &[("TYPE", "Date")],
            )?;
        }
        XmlBuilder::write_current_company_tag(&mut writer, current_company)?;
        XmlBuilder::write_end_tag(&mut writer, "STATICVARIABLES")?;
        XmlBuilder::write_end_tag(&mut writer, "DESC")?;
        XmlBuilder::write_end_tag(&mut writer, "BODY")?;
        XmlBuilder::write_end_tag(&mut writer, "ENVELOPE")?;

        XmlBuilder::finish_writer(writer)
    }
}

pub(crate) fn format_tally_static_date(date: &str) -> Result<String> {
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

    #[test]
    fn builtin_report_request_uses_dates_and_company() {
        let xml = XmlBuilder::create_builtin_report_request(
            "Trial Balance",
            Some("20250401"),
            Some("20260331"),
            Some("Example Company"),
            true,
        )
        .expect("report request");

        assert!(xml.contains("<ID>Trial Balance</ID>"));
        assert!(xml.contains("<EXPLODEFLAG>Yes</EXPLODEFLAG>"));
        assert!(xml.contains("<SVFROMDATE TYPE=\"Date\">01-Apr-2025</SVFROMDATE>"));
        assert!(xml.contains("<SVTODATE TYPE=\"Date\">31-Mar-2026</SVTODATE>"));
        assert!(xml.contains("<SVCURRENTCOMPANY>Example Company</SVCURRENTCOMPANY>"));
    }
}
