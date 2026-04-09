use super::{report_parser, TallyClient};
use crate::errors::Result;
use crate::models::{BalanceSheetEntry, ProfitAndLossEntry, TrialBalanceEntry};
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
