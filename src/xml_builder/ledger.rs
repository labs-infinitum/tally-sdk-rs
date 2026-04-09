use crate::errors::Result;
use serde_json::Value;

use super::XmlBuilder;

impl XmlBuilder {
    pub fn create_ledger_request(ledger_map: &serde_json::Map<String, Value>) -> Result<String> {
        let mut s = String::new();
        s.push_str("<ENVELOPE>\n<HEADER>\n<TALLYREQUEST>Import Data</TALLYREQUEST>\n</HEADER>\n<BODY>\n<IMPORTDATA>\n<REQUESTDESC>\n<REPORTNAME>All Masters</REPORTNAME>\n</REQUESTDESC>\n<REQUESTDATA>\n<TALLYMESSAGE xmlns:UDF=\"TallyUDF\">\n<LEDGER Action=\"Create\">\n");
        XmlBuilder::append_simple_if(ledger_map, "NAME", &mut s);
        XmlBuilder::append_simple_if(ledger_map, "PARENT", &mut s);
        if let Some(v) = ledger_map.get("OPENINGBALANCE") {
            s.push_str(&format!(
                "<OPENINGBALANCE>{}</OPENINGBALANCE>\n",
                XmlBuilder::escape_text(v)
            ));
        }
        XmlBuilder::append_simple_if(ledger_map, "INCOMETAXNUMBER", &mut s);
        XmlBuilder::append_simple_if(ledger_map, "GSTAPPLICABLE", &mut s);
        XmlBuilder::append_simple_if(ledger_map, "APPROPRIATEFOR", &mut s);
        XmlBuilder::append_simple_if(ledger_map, "GSTAPPROPRIATETO", &mut s);
        XmlBuilder::append_simple_if(ledger_map, "EXCISEALLOCTYPE", &mut s);
        XmlBuilder::append_simple_if(ledger_map, "GSTTYPEOFSUPPLY", &mut s);
        XmlBuilder::append_simple_if(ledger_map, "GSTDUTYHEAD", &mut s);
        XmlBuilder::append_simple_if(ledger_map, "RATEOFTAXCALCULATION", &mut s);
        XmlBuilder::append_simple_if(ledger_map, "TAXTYPE", &mut s);
        XmlBuilder::append_simple_if(ledger_map, "BILLCREDITPERIOD", &mut s);
        XmlBuilder::append_simple_if(ledger_map, "ISBILLWISEON", &mut s);
        XmlBuilder::append_simple_if(ledger_map, "ISCREDITDAYSCHKON", &mut s);

        if let Some(v) = ledger_map
            .get("LEDMAILINGDETAILS.LIST")
            .and_then(|v| v.as_object())
        {
            s.push_str("<LEDMAILINGDETAILS.LIST>\n");
            XmlBuilder::append_simple_if(v, "APPLICABLEFROM", &mut s);
            XmlBuilder::append_simple_if(v, "MAILINGNAME", &mut s);
            if let Some(addr_list) = v.get("ADDRESS.LIST").and_then(|x| x.as_array()) {
                s.push_str("<ADDRESS.LIST TYPE=\"String\">\n");
                for item in addr_list {
                    if let Some(obj) = item.as_object() {
                        XmlBuilder::append_simple_if(obj, "ADDRESS", &mut s);
                    }
                }
                s.push_str("</ADDRESS.LIST>\n");
            }
            XmlBuilder::append_simple_if(v, "COUNTRY", &mut s);
            XmlBuilder::append_simple_if(v, "STATE", &mut s);
            XmlBuilder::append_simple_if(v, "PINCODE", &mut s);
            s.push_str("</LEDMAILINGDETAILS.LIST>\n");
        }

        XmlBuilder::append_simple_if(ledger_map, "BANKDETAILS", &mut s);
        XmlBuilder::append_simple_if(ledger_map, "IFSCODE", &mut s);
        XmlBuilder::append_simple_if(ledger_map, "BANKACCHOLDERNAME", &mut s);
        XmlBuilder::append_simple_if(ledger_map, "SWIFTCODE", &mut s);
        XmlBuilder::append_simple_if(ledger_map, "BRANCHNAME", &mut s);
        XmlBuilder::append_simple_if(ledger_map, "BANKBSRCODE", &mut s);
        XmlBuilder::append_simple_if(ledger_map, "ODLIMIT", &mut s);

        if let Some(v) = ledger_map
            .get("LEDGSTREGDETAILS.LIST")
            .and_then(|v| v.as_object())
        {
            s.push_str("<LEDGSTREGDETAILS.LIST>\n");
            for k in [
                "APPLICABLEFROM",
                "GSTREGISTRATIONTYPE",
                "GSTIN",
                "PLACEOFSUPPLY",
                "TRANSPORTERID",
                "ISOTHTERRITORYASSESSEE",
                "CONSIDERPURCHASEFOREXPORT",
                "ISTRANSPORTER",
                "ISCOMMONPARTY",
            ] {
                XmlBuilder::append_simple_if(v, k, &mut s);
            }
            s.push_str("</LEDGSTREGDETAILS.LIST>\n");
        }

        if let Some(v) = ledger_map.get("PAYMENTDETAILS").and_then(|v| v.as_object()) {
            s.push_str("<PAYMENTDETAILS.LIST>\n");
            for k in [
                "PAYMENTFAVOURING",
                "TRANSACTIONNAME",
                "SETASDEFAULT",
                "DEFAULTTRANSACTIONTYPE",
                "CHEQUECROSSCOMMENT",
                "VIRTUALPAYMENTADDRESS",
                "IFSCODE",
                "BANKNAME",
                "ACCOUNTNUMBER",
            ] {
                XmlBuilder::append_simple_if(v, k, &mut s);
            }
            if let Some(ben) = v.get("BENEFICIARYCODEDETAILS").and_then(|x| x.as_object()) {
                s.push_str("<BENEFICIARYCODEDETAILS.LIST>\n");
                XmlBuilder::append_simple_if(ben, "BENEFICIARYCODE", &mut s);
                s.push_str("</BENEFICIARYCODEDETAILS.LIST>\n");
            }
            s.push_str("</PAYMENTDETAILS.LIST>\n");
        }

        XmlBuilder::append_simple_if(ledger_map, "ISTDSAPPLICABLE", &mut s);
        XmlBuilder::append_simple_if(ledger_map, "TDSDEDUCTEETYPE", &mut s);
        XmlBuilder::append_simple_if(ledger_map, "DEDUCTINSAMEVCH", &mut s);
        if let Some(v) = ledger_map.get("TDSAPPLICABLE") {
            s.push_str(&format!(
                "<TDSAPPLICABLE>{}</TDSAPPLICABLE>\n",
                XmlBuilder::escape_text(v)
            ));
        }
        if let Some(v) = ledger_map
            .get("TDSCATEGORYDETAILS.LIST")
            .and_then(|v| v.as_object())
        {
            s.push_str("<TDSCATEGORYDETAILS.LIST>\n");
            XmlBuilder::append_simple_if(v, "CATEGORYDATE", &mut s);
            XmlBuilder::append_simple_if(v, "CATEGORYNAME", &mut s);
            s.push_str("</TDSCATEGORYDETAILS.LIST>\n");
        }

        XmlBuilder::append_simple_if(ledger_map, "VATDEALERNATURE", &mut s);
        XmlBuilder::append_simple_if(ledger_map, "ROUNDINGMETHOD", &mut s);
        XmlBuilder::append_simple_if(ledger_map, "ROUNDINGLIMIT", &mut s);

        if let Some(v) = ledger_map
            .get("HSNDETAILS.LIST")
            .and_then(|v| v.as_object())
        {
            s.push_str("<HSNDETAILS.LIST>\n");
            for k in [
                "APPLICABLEFROM",
                "SRCOFHSNDETAILS",
                "HSNCODE",
                "HSN",
                "HSNCLASSIFICATIONNAME",
            ] {
                XmlBuilder::append_simple_if(v, k, &mut s);
            }
            s.push_str("</HSNDETAILS.LIST>\n");
        }

        if let Some(v) = ledger_map
            .get("GSTDETAILS.LIST")
            .and_then(|v| v.as_object())
        {
            s.push_str("<GSTDETAILS.LIST>\n");
            for k in [
                "APPLICABLEFROM",
                "HSNMASTERNAME",
                "TAXABILITY",
                "SRCOFGSTDETAILS",
            ] {
                XmlBuilder::append_simple_if(v, k, &mut s);
            }
            if let Some(state) = v.get("STATEWISEDETAILS.LIST").and_then(|x| x.as_object()) {
                s.push_str("<STATEWISEDETAILS.LIST>\n");
                if let Some(name) = state.get("STATENAME") {
                    if name.is_string() && name.as_str().unwrap_or("").starts_with("&#") {
                        if let Some(sv) = name.as_str() {
                            s.push_str(&format!("<STATENAME>{}</STATENAME>\n", sv));
                        }
                    } else {
                        s.push_str(&format!(
                            "<STATENAME>{}</STATENAME>\n",
                            XmlBuilder::escape_text(name)
                        ));
                    }
                } else {
                    s.push_str("<STATENAME>&#4; Any</STATENAME>\n");
                }
                if let Some(rate) = state.get("RATEDETAILS.LIST").and_then(|x| x.as_object()) {
                    s.push_str("<RATEDETAILS.LIST>\n");
                    for k in ["GSTRATEDUTYHEAD", "GSTRATEVALUATIONTYPE", "GSTRATE"] {
                        XmlBuilder::append_simple_if(rate, k, &mut s);
                    }
                    s.push_str("</RATEDETAILS.LIST>\n");
                }
                s.push_str("</STATEWISEDETAILS.LIST>\n");
            }
            s.push_str("</GSTDETAILS.LIST>\n");
        }

        s.push_str("<LANGUAGENAME.LIST>\n<NAME.LIST TYPE=\"String\">\n");
        if let Some(Value::String(name)) = ledger_map.get("NAME").cloned() {
            s.push_str(&format!("<NAME>{}</NAME>\n", name));
        }
        if let Some(Value::Array(alias_arr)) = ledger_map.get("ALIAS") {
            for alias in alias_arr {
                if let Value::String(a) = alias {
                    s.push_str(&format!("<NAME>{}</NAME>\n", a));
                }
            }
        }
        s.push_str("</NAME.LIST>\n<LANGUAGEID>1033</LANGUAGEID>\n</LANGUAGENAME.LIST>\n");
        s.push_str(
            "</LEDGER>\n</TALLYMESSAGE>\n</REQUESTDATA>\n</IMPORTDATA>\n</BODY>\n</ENVELOPE>",
        );
        Ok(s)
    }
}
