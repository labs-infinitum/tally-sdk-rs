use super::{voucher_parser, TallyClient};
use crate::errors::Result;
use crate::models::Voucher;
use crate::xml_builder::XmlBuilder;

impl TallyClient {
    /// Fetch vouchers from Tally server
    ///
    /// # Arguments
    /// * `from_date` - Optional start date in YYYYMMDD format (e.g., "20250101")
    /// * `to_date` - Optional end date in YYYYMMDD format (e.g., "20251231")
    ///
    /// If no dates are provided, fetches all vouchers (uses "19000101" to current date)
    pub fn get_vouchers(
        &self,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<Vec<Voucher>> {
        let current_company = self.current_company_name()?;
        let xml = XmlBuilder::create_voucher_export_request(
            from_date,
            to_date,
            current_company.as_deref(),
        )?;
        let resp = self.post_xml(&xml)?;
        Ok(voucher_parser::parse_vouchers_from_xml(&resp))
    }

    /// Fetch vouchers and enforce the date window client-side.
    ///
    /// Tally's Day Book export does not always honor date filters consistently
    /// across environments, so this method applies an exact YYYYMMDD filter on
    /// the parsed vouchers before returning them.
    pub fn get_vouchers_in_range(&self, from_date: &str, to_date: &str) -> Result<Vec<Voucher>> {
        let vouchers = self.get_vouchers(Some(from_date), Some(to_date))?;
        Ok(vouchers
            .into_iter()
            .filter(|voucher| is_yyyymmdd_in_range(&voucher.date_yyyymmdd, from_date, to_date))
            .collect())
    }
}

fn is_yyyymmdd_in_range(date: &str, from_date: &str, to_date: &str) -> bool {
    if date.len() != 8 || from_date.len() != 8 || to_date.len() != 8 {
        return false;
    }
    date >= from_date && date <= to_date
}
