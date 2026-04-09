use crate::errors::Result;
use serde_json::Value;

use super::XmlBuilder;

impl XmlBuilder {
    pub fn create_ledger_request(ledger_map: &serde_json::Map<String, Value>) -> Result<String> {
        XmlBuilder::create_all_masters_import_request(|writer| {
            XmlBuilder::write_start_tag_with_attrs(writer, "LEDGER", &[("Action", "Create")])?;
            XmlBuilder::write_simple_if(writer, ledger_map, "NAME")?;
            XmlBuilder::write_parent_tag(
                writer,
                ledger_map.get("PARENT").and_then(|v| v.as_str()),
                false,
            )?;
            if let Some(v) = ledger_map.get("OPENINGBALANCE") {
                XmlBuilder::write_simple(writer, "OPENINGBALANCE", v)?;
            }
            for key in [
                "INCOMETAXNUMBER",
                "GSTAPPLICABLE",
                "APPROPRIATEFOR",
                "GSTAPPROPRIATETO",
                "EXCISEALLOCTYPE",
                "GSTTYPEOFSUPPLY",
                "GSTDUTYHEAD",
                "RATEOFTAXCALCULATION",
                "TAXTYPE",
                "BILLCREDITPERIOD",
                "ISBILLWISEON",
                "ISCREDITDAYSCHKON",
            ] {
                XmlBuilder::write_simple_if(writer, ledger_map, key)?;
            }

            if let Some(v) = ledger_map
                .get("LEDMAILINGDETAILS.LIST")
                .and_then(|v| v.as_object())
            {
                XmlBuilder::write_start_tag(writer, "LEDMAILINGDETAILS.LIST")?;
                for key in [
                    "APPLICABLEFROM",
                    "MAILINGNAME",
                    "COUNTRY",
                    "STATE",
                    "PINCODE",
                ] {
                    if key == "COUNTRY" || key == "STATE" || key == "PINCODE" {
                        continue;
                    }
                    XmlBuilder::write_simple_if(writer, v, key)?;
                }
                if let Some(addr_list) = v.get("ADDRESS.LIST").and_then(|x| x.as_array()) {
                    XmlBuilder::write_start_tag_with_attrs(
                        writer,
                        "ADDRESS.LIST",
                        &[("TYPE", "String")],
                    )?;
                    for item in addr_list {
                        if let Some(obj) = item.as_object() {
                            XmlBuilder::write_simple_if(writer, obj, "ADDRESS")?;
                        }
                    }
                    XmlBuilder::write_end_tag(writer, "ADDRESS.LIST")?;
                }
                for key in ["COUNTRY", "STATE", "PINCODE"] {
                    XmlBuilder::write_simple_if(writer, v, key)?;
                }
                XmlBuilder::write_end_tag(writer, "LEDMAILINGDETAILS.LIST")?;
            }

            for key in [
                "BANKDETAILS",
                "IFSCODE",
                "BANKACCHOLDERNAME",
                "SWIFTCODE",
                "BRANCHNAME",
                "BANKBSRCODE",
                "ODLIMIT",
            ] {
                XmlBuilder::write_simple_if(writer, ledger_map, key)?;
            }

            if let Some(v) = ledger_map
                .get("LEDGSTREGDETAILS.LIST")
                .and_then(|v| v.as_object())
            {
                XmlBuilder::write_start_tag(writer, "LEDGSTREGDETAILS.LIST")?;
                for key in [
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
                    XmlBuilder::write_simple_if(writer, v, key)?;
                }
                XmlBuilder::write_end_tag(writer, "LEDGSTREGDETAILS.LIST")?;
            }

            if let Some(v) = ledger_map.get("PAYMENTDETAILS").and_then(|v| v.as_object()) {
                XmlBuilder::write_start_tag(writer, "PAYMENTDETAILS.LIST")?;
                for key in [
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
                    XmlBuilder::write_simple_if(writer, v, key)?;
                }
                if let Some(ben) = v.get("BENEFICIARYCODEDETAILS").and_then(|x| x.as_object()) {
                    XmlBuilder::write_start_tag(writer, "BENEFICIARYCODEDETAILS.LIST")?;
                    XmlBuilder::write_simple_if(writer, ben, "BENEFICIARYCODE")?;
                    XmlBuilder::write_end_tag(writer, "BENEFICIARYCODEDETAILS.LIST")?;
                }
                XmlBuilder::write_end_tag(writer, "PAYMENTDETAILS.LIST")?;
            }

            for key in ["ISTDSAPPLICABLE", "TDSDEDUCTEETYPE", "DEDUCTINSAMEVCH"] {
                XmlBuilder::write_simple_if(writer, ledger_map, key)?;
            }
            if let Some(v) = ledger_map.get("TDSAPPLICABLE") {
                XmlBuilder::write_simple(writer, "TDSAPPLICABLE", v)?;
            }
            XmlBuilder::write_tds_category_details_block(
                writer,
                ledger_map
                    .get("TDSCATEGORYDETAILS.LIST")
                    .and_then(|v| v.as_object()),
            )?;

            for key in ["VATDEALERNATURE", "ROUNDINGMETHOD", "ROUNDINGLIMIT"] {
                XmlBuilder::write_simple_if(writer, ledger_map, key)?;
            }

            XmlBuilder::write_hsn_details_block(
                writer,
                ledger_map
                    .get("HSNDETAILS.LIST")
                    .and_then(|v| v.as_object()),
                &[
                    "APPLICABLEFROM",
                    "SRCOFHSNDETAILS",
                    "HSNCODE",
                    "HSN",
                    "HSNCLASSIFICATIONNAME",
                ],
            )?;
            XmlBuilder::write_gst_details_block(
                writer,
                ledger_map
                    .get("GSTDETAILS.LIST")
                    .and_then(|v| v.as_object()),
                &[
                    "APPLICABLEFROM",
                    "HSNMASTERNAME",
                    "TAXABILITY",
                    "SRCOFGSTDETAILS",
                ],
                true,
                true,
            )?;

            if let Some(Value::String(name)) = ledger_map.get("NAME") {
                XmlBuilder::write_language_name_list(
                    writer,
                    name,
                    ledger_map.get("ALIAS"),
                    false,
                    false,
                )?;
            }
            XmlBuilder::write_end_tag(writer, "LEDGER")
        })
    }
}
