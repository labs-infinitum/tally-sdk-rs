use crate::errors::{Result, TallyError};
use serde_json::Value;

use super::XmlBuilder;

impl XmlBuilder {
    pub fn create_group_request(group_map: &serde_json::Map<String, Value>) -> Result<String> {
        let name = group_map
            .get("NAME")
            .and_then(|v| v.as_str())
            .ok_or_else(|| TallyError::Validation("Group NAME is required".into()))?;
        XmlBuilder::create_all_masters_import_request(|writer| {
            XmlBuilder::write_start_tag_with_attrs(
                writer,
                "GROUP",
                &[("NAME", name), ("RESERVEDNAME", "")],
            )?;
            XmlBuilder::write_parent_tag(
                writer,
                group_map.get("PARENT").and_then(|v| v.as_str()),
                true,
            )?;
            for key in [
                "ISADDABLE",
                "BASICGROUPISCALCULABLE",
                "ASORIGINAL",
                "ISSUBLEDGER",
                "ADDLALLOCTYPE",
                "AFFECTSGROSSPROFIT",
            ] {
                XmlBuilder::write_simple_if(writer, group_map, key)?;
            }

            if let Some(Value::String(group_type)) = group_map.get("GROUP_TYPE") {
                match group_type.as_str() {
                    "Assets" => {
                        XmlBuilder::write_text_node(writer, "ISREVENUE", "No")?;
                        XmlBuilder::write_text_node(writer, "AFFECTSGROSSPROFIT", "No")?;
                        XmlBuilder::write_text_node(writer, "ISDEEMEDPOSITIVE", "Yes")?;
                        XmlBuilder::write_text_node(writer, "AFFECTSSTOCK", "No")?;
                    }
                    "Liabilities" => {
                        XmlBuilder::write_text_node(writer, "ISREVENUE", "No")?;
                        XmlBuilder::write_text_node(writer, "AFFECTSGROSSPROFIT", "No")?;
                        XmlBuilder::write_text_node(writer, "ISDEEMEDPOSITIVE", "No")?;
                        XmlBuilder::write_text_node(writer, "AFFECTSSTOCK", "No")?;
                    }
                    "Income" => {
                        XmlBuilder::write_text_node(writer, "ISREVENUE", "Yes")?;
                        XmlBuilder::write_text_node(writer, "AFFECTSGROSSPROFIT", "Yes")?;
                        XmlBuilder::write_text_node(writer, "ISDEEMEDPOSITIVE", "No")?;
                        XmlBuilder::write_text_node(writer, "AFFECTSSTOCK", "No")?;
                    }
                    "Expenses" => {
                        XmlBuilder::write_text_node(writer, "ISREVENUE", "Yes")?;
                        XmlBuilder::write_text_node(writer, "AFFECTSGROSSPROFIT", "No")?;
                        XmlBuilder::write_text_node(writer, "ISDEEMEDPOSITIVE", "Yes")?;
                        XmlBuilder::write_text_node(writer, "AFFECTSSTOCK", "No")?;
                    }
                    _ => {}
                }
            }

            XmlBuilder::write_hsn_details_block(
                writer,
                group_map.get("HSNDETAILS.LIST").and_then(|v| v.as_object()),
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
                group_map.get("GSTDETAILS.LIST").and_then(|v| v.as_object()),
                &[
                    "APPLICABLEFROM",
                    "HSNMASTERNAME",
                    "TAXABILITY",
                    "SRCOFGSTDETAILS",
                ],
                true,
                false,
            )?;
            XmlBuilder::write_language_name_list(writer, name, group_map.get("ALIAS"), true, true)?;
            XmlBuilder::write_simple_if(writer, group_map, "TDSAPPLICABLE")?;
            XmlBuilder::write_tds_category_details_block(
                writer,
                group_map
                    .get("TDSCATEGORYDETAILS.LIST")
                    .and_then(|v| v.as_object()),
            )?;
            XmlBuilder::write_end_tag(writer, "GROUP")
        })
    }
}
