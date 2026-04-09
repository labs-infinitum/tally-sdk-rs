use crate::errors::Result;
use serde_json::Value;

use super::XmlBuilder;

impl XmlBuilder {
    pub fn create_stock_item_request(item_map: &serde_json::Map<String, Value>) -> Result<String> {
        let name = item_map.get("NAME").and_then(|v| v.as_str()).unwrap_or("");
        let mut s = String::new();
        XmlBuilder::append_all_masters_import_start(&mut s);
        s.push_str(&format!(
            "<STOCKITEM NAME=\"{}\" RESERVEDNAME=\"\">\n",
            XmlBuilder::escape_simple(name)
        ));
        XmlBuilder::append_parent_tag(
            &mut s,
            item_map.get("PARENT").and_then(|v| v.as_str()),
            true,
        );
        for k in [
            "BASEUNITS",
            "ADDITIONALUNITS",
            "GSTAPPLICABLE",
            "GSTTYPEOFSUPPLY",
            "BASICRATEOFEXCISE",
            "OPENINGBALANCE",
        ] {
            XmlBuilder::append_simple_if(item_map, k, &mut s);
        }
        XmlBuilder::append_hsn_details_block(
            &mut s,
            item_map.get("HSNDETAILS.LIST").and_then(|v| v.as_object()),
            &[
                "APPLICABLEFROM",
                "HSNCODE",
                "HSN",
                "SRCOFHSNDETAILS",
                "HSNCLASSIFICATIONNAME",
            ],
        );
        XmlBuilder::append_gst_details_block(
            &mut s,
            item_map.get("GSTDETAILS.LIST").and_then(|v| v.as_object()),
            &[
                "APPLICABLEFROM",
                "TAXABILITY",
                "SRCOFGSTDETAILS",
                "HSNMASTERNAME",
            ],
            true,
            false,
        );
        XmlBuilder::append_language_name_list(&mut s, name, item_map.get("ALIAS"), true, true);
        s.push_str("</STOCKITEM>\n");
        XmlBuilder::append_import_end(&mut s);
        Ok(s)
    }
}
