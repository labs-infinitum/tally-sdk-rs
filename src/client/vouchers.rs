use super::{voucher_parser, TallyClient};
use crate::errors::Result;
use crate::models::{CurrencySummary, ForexDetails, ImportResult, Voucher};
use crate::xml_builder::XmlBuilder;

impl TallyClient {
    /// Create an accounting voucher (Payment, Receipt, Journal, etc.) in Tally.
    pub fn create_voucher(&self, voucher: &Voucher) -> Result<ImportResult> {
        Ok(self.create_voucher_with_response(voucher)?.0)
    }

    /// Create a voucher and return the raw Tally XML response (for diagnostics).
    pub fn create_voucher_with_response(
        &self,
        voucher: &Voucher,
    ) -> Result<(ImportResult, String)> {
        voucher.validate()?;
        let map = voucher.to_map();
        let xml = XmlBuilder::create_voucher_request(&map)?;
        let resp = self.post_xml(&xml)?;
        Ok((super::parse::parse_simple_response_public(&resp), resp))
    }

    /// Same as [`Self::create_voucher`], but prints the XML request/response.
    pub fn create_voucher_debug(&self, voucher: &Voucher) -> Result<ImportResult> {
        voucher.validate()?;
        let map = voucher.to_map();
        let xml = XmlBuilder::create_voucher_request(&map)?;
        self.execute_debug_create_request(&xml)
    }

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
        let mut vouchers = voucher_parser::parse_vouchers_from_xml(&resp);
        if vouchers_have_forex(&vouchers) {
            if let Ok(currencies) = self.get_currencies() {
                enrich_vouchers_with_currency_names(&mut vouchers, &currencies);
            }
        }
        Ok(vouchers)
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

fn vouchers_have_forex(vouchers: &[Voucher]) -> bool {
    vouchers.iter().any(|voucher| {
        voucher.amount_forex.is_some()
            || voucher.entries.iter().any(|entry| entry.forex.is_some())
            || voucher.items.iter().any(|item| {
                item.forex.is_some()
                    || item
                        .batch_allocations
                        .iter()
                        .any(|batch| batch.forex.is_some())
                    || item
                        .accounting_allocations
                        .iter()
                        .any(|allocation| allocation.forex.is_some())
            })
    })
}

fn enrich_vouchers_with_currency_names(vouchers: &mut [Voucher], currencies: &[CurrencySummary]) {
    for voucher in vouchers {
        if let Some(forex) = voucher.amount_forex.as_mut() {
            resolve_forex_names(forex, currencies);
        }
        for entry in &mut voucher.entries {
            if let Some(forex) = entry.forex.as_mut() {
                resolve_forex_names(forex, currencies);
            }
        }
        for item in &mut voucher.items {
            if let Some(forex) = item.forex.as_mut() {
                resolve_forex_names(forex, currencies);
            }
            for batch in &mut item.batch_allocations {
                if let Some(forex) = batch.forex.as_mut() {
                    resolve_forex_names(forex, currencies);
                }
            }
            for allocation in &mut item.accounting_allocations {
                if let Some(forex) = allocation.forex.as_mut() {
                    resolve_forex_names(forex, currencies);
                }
            }
        }
    }
}

fn resolve_forex_names(forex: &mut ForexDetails, currencies: &[CurrencySummary]) {
    forex.foreign_currency_name = currency_name_for_symbol(&forex.foreign_currency, currencies);
    forex.base_currency_name = currency_name_for_symbol(&forex.base_currency, currencies);
}

fn currency_name_for_symbol(symbol: &str, currencies: &[CurrencySummary]) -> Option<String> {
    currencies
        .iter()
        .find(|currency| currency.matches_symbol(symbol))
        .map(|currency| currency.display_name().to_string())
        .filter(|name| !name.is_empty() && !name.eq_ignore_ascii_case(symbol))
}

#[cfg(test)]
mod tests {
    use super::{currency_name_for_symbol, resolve_forex_names};
    use crate::models::{CurrencySummary, ForexDetails};

    #[test]
    fn resolves_currency_names_from_symbols() {
        let currencies = vec![
            CurrencySummary {
                name: "D$".into(),
                original_name: Some("$".into()),
                mailing_name: Some("Dollar".into()),
                expanded_symbol: Some("Dollar".into()),
                decimal_symbol: Some("Cent".into()),
                decimal_places: Some(2),
                decimal_places_for_printing: Some(2),
                is_suffix: Some(false),
                has_space: Some(false),
                in_millions: Some(false),
                guid: None,
            },
            CurrencySummary {
                name: "EUR".into(),
                original_name: Some("EUR".into()),
                mailing_name: Some("EURO".into()),
                expanded_symbol: Some("EURO".into()),
                decimal_symbol: None,
                decimal_places: Some(2),
                decimal_places_for_printing: Some(2),
                is_suffix: Some(false),
                has_space: Some(false),
                in_millions: Some(false),
                guid: None,
            },
        ];

        assert_eq!(
            currency_name_for_symbol("EUR", &currencies).as_deref(),
            Some("EURO")
        );
        assert_eq!(
            currency_name_for_symbol("D$", &currencies).as_deref(),
            Some("Dollar")
        );
        assert_eq!(
            currency_name_for_symbol("$", &currencies).as_deref(),
            Some("Dollar")
        );

        let mut forex = ForexDetails {
            foreign_amount: 29.37,
            foreign_currency: "EUR".into(),
            foreign_currency_name: None,
            base_currency: "D$".into(),
            base_currency_name: None,
            exchange_rate: 1.1675,
        };
        resolve_forex_names(&mut forex, &currencies);
        assert_eq!(forex.foreign_currency_label(), "EURO");
        assert_eq!(forex.base_currency_label(), "Dollar");
    }
}
