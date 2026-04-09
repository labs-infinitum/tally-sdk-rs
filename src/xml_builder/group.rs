use crate::errors::{Result, TallyError};
use serde_json::Value;

use super::XmlBuilder;

impl XmlBuilder {
    pub fn create_group_request(group_map: &serde_json::Map<String, Value>) -> Result<String> {
        let name = group_map
            .get("NAME")
            .and_then(|v| v.as_str())
            .ok_or_else(|| TallyError::Validation("Group NAME is required".into()))?;
        let mut s = String::new();
        XmlBuilder::append_all_masters_import_start(&mut s);
        s.push_str(&format!(
            "<GROUP NAME=\"{}\" RESERVEDNAME=\"\">\n",
            XmlBuilder::escape_simple(name)
        ));
        XmlBuilder::append_parent_tag(
            &mut s,
            group_map.get("PARENT").and_then(|v| v.as_str()),
            true,
        );
        for k in [
            "ISADDABLE",
            "BASICGROUPISCALCULABLE",
            "ASORIGINAL",
            "ISSUBLEDGER",
            "ADDLALLOCTYPE",
            "AFFECTSGROSSPROFIT",
        ] {
            XmlBuilder::append_simple_if(group_map, k, &mut s);
        }

        if let Some(Value::String(gt)) = group_map.get("GROUP_TYPE") {
            match gt.as_str() {
                "Assets" => {
                    s.push_str("<ISREVENUE>No</ISREVENUE>\n<AFFECTSGROSSPROFIT>No</AFFECTSGROSSPROFIT>\n<ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE>\n<AFFECTSSTOCK>No</AFFECTSSTOCK>\n");
                }
                "Liabilities" => {
                    s.push_str("<ISREVENUE>No</ISREVENUE>\n<AFFECTSGROSSPROFIT>No</AFFECTSGROSSPROFIT>\n<ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE>\n<AFFECTSSTOCK>No</AFFECTSSTOCK>\n");
                }
                "Income" => {
                    s.push_str("<ISREVENUE>Yes</ISREVENUE>\n<AFFECTSGROSSPROFIT>Yes</AFFECTSGROSSPROFIT>\n<ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE>\n<AFFECTSSTOCK>No</AFFECTSSTOCK>\n");
                }
                "Expenses" => {
                    s.push_str("<ISREVENUE>Yes</ISREVENUE>\n<AFFECTSGROSSPROFIT>No</AFFECTSGROSSPROFIT>\n<ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE>\n<AFFECTSSTOCK>No</AFFECTSSTOCK>\n");
                }
                _ => {}
            }
        }

        XmlBuilder::append_hsn_details_block(
            &mut s,
            group_map.get("HSNDETAILS.LIST").and_then(|v| v.as_object()),
            &[
                "APPLICABLEFROM",
                "SRCOFHSNDETAILS",
                "HSNCODE",
                "HSN",
                "HSNCLASSIFICATIONNAME",
            ],
        );
        XmlBuilder::append_gst_details_block(
            &mut s,
            group_map.get("GSTDETAILS.LIST").and_then(|v| v.as_object()),
            &[
                "APPLICABLEFROM",
                "HSNMASTERNAME",
                "TAXABILITY",
                "SRCOFGSTDETAILS",
            ],
            true,
            false,
        );
        XmlBuilder::append_language_name_list(&mut s, name, group_map.get("ALIAS"), true, true);

        XmlBuilder::append_simple_if(group_map, "TDSAPPLICABLE", &mut s);
        XmlBuilder::append_tds_category_details_block(
            &mut s,
            group_map
                .get("TDSCATEGORYDETAILS.LIST")
                .and_then(|v| v.as_object()),
        );

        s.push_str("</GROUP>\n");
        XmlBuilder::append_import_end(&mut s);
        Ok(s)
    }
}
