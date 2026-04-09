use crate::errors::Result;
use serde_json::Value;

use super::XmlBuilder;

impl XmlBuilder {
    pub fn create_item_invoice_request(vch: &serde_json::Map<String, Value>) -> Result<String> {
        let name = vch
            .get("VOUCHERTYPENAME")
            .and_then(|s| s.as_str())
            .unwrap_or("Sales");
        let mut s = String::new();
        XmlBuilder::append_all_masters_import_start(&mut s);
        s.push_str(&format!(
            "<VOUCHER VCHTYPE=\"{}\" ACTION=\"Create\" OBJVIEW=\"Invoice Voucher View\">\n",
            XmlBuilder::escape_simple(name)
        ));
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
            XmlBuilder::append_simple_if(vch, tag, &mut s);
        }
        if let Some(inv) = vch
            .get("ALLINVENTORYENTRIES.LIST")
            .and_then(|x| x.as_object())
        {
            s.push_str("<ALLINVENTORYENTRIES.LIST>\n");
            for tag in [
                "STOCKITEMNAME",
                "ISDEEMEDPOSITIVE",
                "RATE",
                "AMOUNT",
                "ACTUALQTY",
                "BILLEDQTY",
            ] {
                XmlBuilder::append_simple_if(inv, tag, &mut s);
            }
            if let Some(acc) = inv
                .get("ACCOUNTINGALLOCATIONS.LIST")
                .and_then(|x| x.as_object())
            {
                s.push_str("<ACCOUNTINGALLOCATIONS.LIST>\n");
                for tag in ["LEDGERNAME", "ISDEEMEDPOSITIVE", "AMOUNT"] {
                    XmlBuilder::append_simple_if(acc, tag, &mut s);
                }
                s.push_str("</ACCOUNTINGALLOCATIONS.LIST>\n");
            }
            s.push_str("</ALLINVENTORYENTRIES.LIST>\n");
        }
        if let Some(party) = vch.get("LEDGERENTRIES.LIST").and_then(|x| x.as_object()) {
            s.push_str("<LEDGERENTRIES.LIST>\n");
            for tag in ["LEDGERNAME", "ISDEEMEDPOSITIVE", "AMOUNT", "ISPARTYLEDGER"] {
                XmlBuilder::append_simple_if(party, tag, &mut s);
            }
            s.push_str("</LEDGERENTRIES.LIST>\n");
        }
        if let Some(del) = vch.get("INVOICEDELNOTES.LIST").and_then(|x| x.as_object()) {
            s.push_str("<INVOICEDELNOTES.LIST>\n");
            for tag in ["BASICSHIPPINGDATE", "BASICSHIPDELIVERYNOTE"] {
                XmlBuilder::append_simple_if(del, tag, &mut s);
            }
            s.push_str("</INVOICEDELNOTES.LIST>\n");
        }
        s.push_str("</VOUCHER>\n");
        XmlBuilder::append_import_end(&mut s);
        Ok(s)
    }
}
