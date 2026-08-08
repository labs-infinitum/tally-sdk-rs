use crate::errors::Result;
use serde_json::Value;

use super::XmlBuilder;

impl XmlBuilder {
    /// Build a voucher import request using Tally's `Import Data` / `All Masters` envelope.
    ///
    /// This matches the path already used successfully by item-invoice imports in this crate.
    ///
    /// Expected map keys:
    /// - `VOUCHERTYPENAME` (also used as the `VCHTYPE` attribute)
    /// - `DATE`, optional `EFFECTIVEDATE`, `NARRATION`, `VOUCHERNUMBER`, `PARTYLEDGERNAME`
    /// - `LEDGERENTRIES.LIST`: array of ledger entry objects
    ///
    /// Amount convention used by Tally imports:
    /// - Debit: `ISDEEMEDPOSITIVE=Yes` and negative `AMOUNT`
    /// - Credit: `ISDEEMEDPOSITIVE=No` and positive `AMOUNT`
    pub fn create_voucher_request(voucher_map: &serde_json::Map<String, Value>) -> Result<String> {
        let vch_type = voucher_map
            .get("VOUCHERTYPENAME")
            .and_then(|v| v.as_str())
            .unwrap_or("Journal");
        let obj_view = voucher_map
            .get("OBJVIEW")
            .and_then(|v| v.as_str())
            .unwrap_or("Accounting Voucher View");

        // Use the Vouchers import report (not All Masters) — matches Tally sample
        // Payment/Receipt XML and reduces silent Import Exceptions.
        XmlBuilder::create_import_data_request("Vouchers", |writer| {
            XmlBuilder::write_start_tag_with_attrs(
                writer,
                "VOUCHER",
                &[
                    ("VCHTYPE", vch_type),
                    ("ACTION", "Create"),
                    ("OBJVIEW", obj_view),
                ],
            )?;

            for tag in [
                "DATE",
                "EFFECTIVEDATE",
                "VOUCHERTYPENAME",
                "VOUCHERNUMBER",
                "REFERENCE",
                "REFERENCEDATE",
                "NARRATION",
                "PARTYLEDGERNAME",
                "PERSISTEDVIEW",
                "ISINVOICE",
            ] {
                XmlBuilder::write_simple_if(writer, voucher_map, tag)?;
            }

            if let Some(entries) = voucher_map
                .get("LEDGERENTRIES.LIST")
                .and_then(|v| v.as_array())
                .or_else(|| {
                    voucher_map
                        .get("ALLLEDGERENTRIES.LIST")
                        .and_then(|v| v.as_array())
                })
            {
                let entry_tag = if voucher_map.get("LEDGERENTRIES.LIST").is_some() {
                    "LEDGERENTRIES.LIST"
                } else {
                    "ALLLEDGERENTRIES.LIST"
                };

                for entry in entries {
                    let Some(obj) = entry.as_object() else {
                        continue;
                    };
                    XmlBuilder::write_start_tag(writer, entry_tag)?;
                    for tag in [
                        "LEDGERNAME",
                        "ISDEEMEDPOSITIVE",
                        "ISPARTYLEDGER",
                        "AMOUNT",
                    ] {
                        XmlBuilder::write_simple_if(writer, obj, tag)?;
                    }
                    if let Some(bill) = obj.get("BILLALLOCATIONS.LIST") {
                        match bill {
                            Value::Object(bill_obj) => {
                                XmlBuilder::write_bill_allocation(writer, bill_obj)?;
                            }
                            Value::Array(bills) => {
                                for item in bills {
                                    if let Some(bill_obj) = item.as_object() {
                                        XmlBuilder::write_bill_allocation(writer, bill_obj)?;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    XmlBuilder::write_end_tag(writer, entry_tag)?;
                }
            }

            XmlBuilder::write_end_tag(writer, "VOUCHER")
        })
    }

    fn write_bill_allocation(
        writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
        bill_obj: &serde_json::Map<String, Value>,
    ) -> Result<()> {
        XmlBuilder::write_start_tag(writer, "BILLALLOCATIONS.LIST")?;
        for tag in ["NAME", "BILLTYPE", "AMOUNT"] {
            XmlBuilder::write_simple_if(writer, bill_obj, tag)?;
        }
        XmlBuilder::write_end_tag(writer, "BILLALLOCATIONS.LIST")
    }
}
