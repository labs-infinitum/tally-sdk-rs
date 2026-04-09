use crate::errors::Result;
use serde_json::Value;

use super::XmlBuilder;

impl XmlBuilder {
    pub fn create_stock_item_request(item_map: &serde_json::Map<String, Value>) -> Result<String> {
        let name = item_map.get("NAME").and_then(|v| v.as_str()).unwrap_or("");
        XmlBuilder::create_all_masters_import_request(|writer| {
            XmlBuilder::write_start_tag_with_attrs(
                writer,
                "STOCKITEM",
                &[("NAME", name), ("RESERVEDNAME", "")],
            )?;
            XmlBuilder::write_parent_tag(
                writer,
                item_map.get("PARENT").and_then(|v| v.as_str()),
                true,
            )?;
            for key in [
                "BASEUNITS",
                "ADDITIONALUNITS",
                "GSTAPPLICABLE",
                "GSTTYPEOFSUPPLY",
                "BASICRATEOFEXCISE",
                "OPENINGBALANCE",
            ] {
                XmlBuilder::write_simple_if(writer, item_map, key)?;
            }
            XmlBuilder::write_hsn_details_block(
                writer,
                item_map.get("HSNDETAILS.LIST").and_then(|v| v.as_object()),
                &[
                    "APPLICABLEFROM",
                    "HSNCODE",
                    "HSN",
                    "SRCOFHSNDETAILS",
                    "HSNCLASSIFICATIONNAME",
                ],
            )?;
            XmlBuilder::write_gst_details_block(
                writer,
                item_map.get("GSTDETAILS.LIST").and_then(|v| v.as_object()),
                &[
                    "APPLICABLEFROM",
                    "TAXABILITY",
                    "SRCOFGSTDETAILS",
                    "HSNMASTERNAME",
                ],
                true,
                false,
            )?;
            XmlBuilder::write_language_name_list(writer, name, item_map.get("ALIAS"), true, true)?;
            XmlBuilder::write_end_tag(writer, "STOCKITEM")
        })
    }
}
