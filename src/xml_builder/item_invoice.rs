use crate::errors::Result;
use serde_json::Value;

use super::XmlBuilder;

impl XmlBuilder {
    pub fn create_item_invoice_request(vch: &serde_json::Map<String, Value>) -> Result<String> {
        let name = vch
            .get("VOUCHERTYPENAME")
            .and_then(|s| s.as_str())
            .unwrap_or("Sales");
        XmlBuilder::create_all_masters_import_request(|writer| {
            XmlBuilder::write_start_tag_with_attrs(
                writer,
                "VOUCHER",
                &[
                    ("VCHTYPE", name),
                    ("ACTION", "Create"),
                    ("OBJVIEW", "Invoice Voucher View"),
                ],
            )?;
            for tag in [
                "DATE",
                "VCHENTRYMODE",
                "VOUCHERNUMBER",
                "NARRATION",
                "REFERENCE",
                "REFERENCEDATE",
                "BASICSHIPDOCUMENTNO",
                "BASICSHIPPEDBY",
                "BASICFINALDESTINATION",
                "EICHECKPOST",
                "BILLOFLADINGNO",
                "BILLOFLADINGDATE",
                "BASICSHIPVESSELNO",
            ] {
                XmlBuilder::write_simple_if(writer, vch, tag)?;
            }
            if let Some(inv) = vch
                .get("ALLINVENTORYENTRIES.LIST")
                .and_then(|x| x.as_object())
            {
                XmlBuilder::write_start_tag(writer, "ALLINVENTORYENTRIES.LIST")?;
                for tag in [
                    "STOCKITEMNAME",
                    "ISDEEMEDPOSITIVE",
                    "RATE",
                    "AMOUNT",
                    "ACTUALQTY",
                    "BILLEDQTY",
                ] {
                    XmlBuilder::write_simple_if(writer, inv, tag)?;
                }
                if let Some(acc) = inv
                    .get("ACCOUNTINGALLOCATIONS.LIST")
                    .and_then(|x| x.as_object())
                {
                    XmlBuilder::write_start_tag(writer, "ACCOUNTINGALLOCATIONS.LIST")?;
                    for tag in ["LEDGERNAME", "ISDEEMEDPOSITIVE", "AMOUNT"] {
                        XmlBuilder::write_simple_if(writer, acc, tag)?;
                    }
                    XmlBuilder::write_end_tag(writer, "ACCOUNTINGALLOCATIONS.LIST")?;
                }
                XmlBuilder::write_end_tag(writer, "ALLINVENTORYENTRIES.LIST")?;
            }
            if let Some(party) = vch.get("LEDGERENTRIES.LIST").and_then(|x| x.as_object()) {
                XmlBuilder::write_start_tag(writer, "LEDGERENTRIES.LIST")?;
                for tag in ["LEDGERNAME", "ISDEEMEDPOSITIVE", "AMOUNT", "ISPARTYLEDGER"] {
                    XmlBuilder::write_simple_if(writer, party, tag)?;
                }
                XmlBuilder::write_end_tag(writer, "LEDGERENTRIES.LIST")?;
            }
            if let Some(del) = vch.get("INVOICEDELNOTES.LIST").and_then(|x| x.as_object()) {
                XmlBuilder::write_start_tag(writer, "INVOICEDELNOTES.LIST")?;
                for tag in ["BASICSHIPPINGDATE", "BASICSHIPDELIVERYNOTE"] {
                    XmlBuilder::write_simple_if(writer, del, tag)?;
                }
                XmlBuilder::write_end_tag(writer, "INVOICEDELNOTES.LIST")?;
            }
            XmlBuilder::write_end_tag(writer, "VOUCHER")
        })
    }
}
