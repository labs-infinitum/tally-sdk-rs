use super::{gst_parser, report_parser, TallyClient};
use crate::errors::Result;
use crate::models::{
    BalanceSheetEntry, GstComputationReport, Gstr1Report, ProfitAndLossEntry, TrialBalanceEntry,
};
use crate::xml_builder::XmlBuilder;

impl TallyClient {
    pub fn get_trial_balance(
        &self,
        from_date: Option<&str>,
        to_date: Option<&str>,
        explode_flag: bool,
    ) -> Result<Vec<TrialBalanceEntry>> {
        let resp = self.export_builtin_report("Trial Balance", from_date, to_date, explode_flag)?;
        Ok(report_parser::parse_trial_balance_from_xml(&resp))
    }

    pub fn get_balance_sheet(
        &self,
        from_date: Option<&str>,
        to_date: Option<&str>,
        explode_flag: bool,
    ) -> Result<Vec<BalanceSheetEntry>> {
        let resp = self.export_builtin_report("Balance Sheet", from_date, to_date, explode_flag)?;
        Ok(report_parser::parse_balance_sheet_from_xml(&resp))
    }

    pub fn get_profit_and_loss(
        &self,
        from_date: Option<&str>,
        to_date: Option<&str>,
        explode_flag: bool,
    ) -> Result<Vec<ProfitAndLossEntry>> {
        let resp =
            self.export_builtin_report("Profit and Loss", from_date, to_date, explode_flag)?;
        Ok(report_parser::parse_profit_and_loss_from_xml(&resp))
    }

    /// Export Tally's builtin **GST Computation** report for a period.
    pub fn get_gst_computation(
        &self,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<GstComputationReport> {
        let resp = self.export_builtin_report("GST Computation", from_date, to_date, false)?;
        Ok(gst_parser::parse_gst_computation_from_xml(&resp))
    }

    /// Build a GSTR-1 style return summary for `[from_date, to_date]`.
    ///
    /// TallyPrime does not currently expose a builtin HTTP report named `GSTR-1`
    /// (export returns "Could not find Report"). This method therefore derives
    /// B2B / B2CL / B2CS / CDNR / HSN / document sections from vouchers in the
    /// range via [`Self::get_vouchers_in_range`].
    pub fn get_gstr1(&self, from_date: &str, to_date: &str) -> Result<Gstr1Report> {
        let vouchers = self.get_vouchers_in_range(from_date, to_date)?;
        Ok(gst_parser::build_gstr1_from_vouchers(
            from_date, to_date, &vouchers,
        ))
    }

    fn export_builtin_report(
        &self,
        report_name: &str,
        from_date: Option<&str>,
        to_date: Option<&str>,
        explode_flag: bool,
    ) -> Result<String> {
        let current_company = self.current_company_name()?;
        let xml = XmlBuilder::create_builtin_report_request(
            report_name,
            from_date,
            to_date,
            current_company.as_deref(),
            explode_flag,
        )?;
        self.post_xml(&xml)
    }
}
