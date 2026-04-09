use crate::models::{BalanceSheetEntry, ProfitAndLossEntry, TrialBalanceEntry};
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;

pub fn parse_trial_balance_from_xml(xml: &str) -> Vec<TrialBalanceEntry> {
    let mut reader = Reader::from_reader(xml.as_bytes());
    reader.trim_text(true);

    let mut path: Vec<Vec<u8>> = Vec::new();
    let mut current_name: Option<String> = None;
    let mut current_debit: Option<f64> = None;
    let mut current_credit: Option<f64> = None;
    let mut entries = Vec::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                path.push(e.name().as_ref().to_vec());
                if e.name() == QName(b"DSPACCINFO") {
                    current_debit = None;
                    current_credit = None;
                }
            }
            Ok(Event::Text(ref e)) => {
                let text = e.unescape().unwrap_or_default().to_string();
                if text.trim().is_empty() {
                    continue;
                }
                match path.last().map(|tag| tag.as_slice()) {
                    Some(b"DSPDISPNAME") => current_name = Some(text),
                    Some(b"DSPCLDRAMTA") => current_debit = parse_amount(&text),
                    Some(b"DSPCLCRAMTA") => current_credit = parse_amount(&text),
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                if e.name() == QName(b"DSPACCINFO") {
                    if let Some(name) = current_name.take() {
                        entries.push(TrialBalanceEntry {
                            name,
                            debit: current_debit,
                            credit: current_credit,
                        });
                    }
                }
                path.pop();
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
    }

    entries
}

pub fn parse_balance_sheet_from_xml(xml: &str) -> Vec<BalanceSheetEntry> {
    let mut reader = Reader::from_reader(xml.as_bytes());
    reader.trim_text(true);

    let mut path: Vec<Vec<u8>> = Vec::new();
    let mut current_name: Option<String> = None;
    let mut current_main_amount: Option<f64> = None;
    let mut current_sub_amount: Option<f64> = None;
    let mut entries = Vec::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                path.push(e.name().as_ref().to_vec());
                if e.name() == QName(b"BSAMT") {
                    current_main_amount = None;
                    current_sub_amount = None;
                }
            }
            Ok(Event::Text(ref e)) => {
                let text = e.unescape().unwrap_or_default().to_string();
                if text.trim().is_empty() {
                    continue;
                }
                match path.last().map(|tag| tag.as_slice()) {
                    Some(b"DSPDISPNAME") => current_name = Some(text),
                    Some(b"BSMAINAMT") => current_main_amount = parse_amount(&text),
                    Some(b"BSSUBAMT") => current_sub_amount = parse_amount(&text),
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                if e.name() == QName(b"BSAMT") {
                    if let Some(name) = current_name.take() {
                        entries.push(BalanceSheetEntry {
                            name,
                            main_amount: current_main_amount,
                            sub_amount: current_sub_amount,
                        });
                    }
                }
                path.pop();
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
    }

    entries
}

pub fn parse_profit_and_loss_from_xml(xml: &str) -> Vec<ProfitAndLossEntry> {
    let mut reader = Reader::from_reader(xml.as_bytes());
    reader.trim_text(true);

    let mut path: Vec<Vec<u8>> = Vec::new();
    let mut current_name: Option<String> = None;
    let mut current_main_amount: Option<f64> = None;
    let mut current_sub_amount: Option<f64> = None;
    let mut entries = Vec::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                path.push(e.name().as_ref().to_vec());
                if matches!(e.name(), QName(b"PLAMT") | QName(b"BSAMT")) {
                    current_main_amount = None;
                    current_sub_amount = None;
                }
            }
            Ok(Event::Text(ref e)) => {
                let text = e.unescape().unwrap_or_default().to_string();
                if text.trim().is_empty() {
                    continue;
                }
                match path.last().map(|tag| tag.as_slice()) {
                    Some(b"DSPDISPNAME") => current_name = Some(text),
                    Some(b"BSMAINAMT") => current_main_amount = parse_amount(&text),
                    Some(b"PLSUBAMT") | Some(b"BSSUBAMT") => {
                        current_sub_amount = parse_amount(&text)
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                if matches!(e.name(), QName(b"PLAMT") | QName(b"BSAMT")) {
                    if let Some(name) = current_name.take() {
                        entries.push(ProfitAndLossEntry {
                            name,
                            main_amount: current_main_amount,
                            sub_amount: current_sub_amount,
                        });
                    }
                }
                path.pop();
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
    }

    entries
}

fn parse_amount(text: &str) -> Option<f64> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }
    trimmed.parse::<f64>().ok()
}

#[cfg(test)]
mod tests {
    use super::{
        parse_balance_sheet_from_xml, parse_profit_and_loss_from_xml, parse_trial_balance_from_xml,
    };

    #[test]
    fn parses_trial_balance_rows() {
        let xml = r#"
<ENVELOPE>
  <DSPACCNAME><DSPDISPNAME>Capital Account</DSPDISPNAME></DSPACCNAME>
  <DSPACCINFO>
    <DSPCLDRAMT><DSPCLDRAMTA></DSPCLDRAMTA></DSPCLDRAMT>
    <DSPCLCRAMT><DSPCLCRAMTA>50000.00</DSPCLCRAMTA></DSPCLCRAMT>
  </DSPACCINFO>
  <DSPACCNAME><DSPDISPNAME>Cash</DSPDISPNAME></DSPACCNAME>
  <DSPACCINFO>
    <DSPCLDRAMT><DSPCLDRAMTA>-1000.00</DSPCLDRAMTA></DSPCLDRAMT>
    <DSPCLCRAMT><DSPCLCRAMTA></DSPCLCRAMTA></DSPCLCRAMT>
  </DSPACCINFO>
</ENVELOPE>
"#;

        let rows = parse_trial_balance_from_xml(xml);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].name, "Capital Account");
        assert_eq!(rows[0].debit, None);
        assert_eq!(rows[0].credit, Some(50000.0));
        assert_eq!(rows[1].name, "Cash");
        assert_eq!(rows[1].debit, Some(-1000.0));
        assert_eq!(rows[1].credit, None);
    }

    #[test]
    fn parses_balance_sheet_rows() {
        let xml = r#"
<ENVELOPE>
  <BSNAME><DSPACCNAME><DSPDISPNAME>Capital Account</DSPDISPNAME></DSPACCNAME></BSNAME>
  <BSAMT><BSSUBAMT></BSSUBAMT><BSMAINAMT>50000.00</BSMAINAMT></BSAMT>
  <BSNAME><DSPACCNAME><DSPDISPNAME>Yash Goyal (Capital Account)</DSPDISPNAME></DSPACCNAME></BSNAME>
  <BSAMT><BSSUBAMT>45000.00</BSSUBAMT><BSMAINAMT></BSMAINAMT></BSAMT>
</ENVELOPE>
"#;

        let rows = parse_balance_sheet_from_xml(xml);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].name, "Capital Account");
        assert_eq!(rows[0].main_amount, Some(50000.0));
        assert_eq!(rows[0].sub_amount, None);
        assert_eq!(rows[1].name, "Yash Goyal (Capital Account)");
        assert_eq!(rows[1].main_amount, None);
        assert_eq!(rows[1].sub_amount, Some(45000.0));
    }

    #[test]
    fn parses_profit_and_loss_rows() {
        let xml = r#"
<ENVELOPE>
  <DSPACCNAME><DSPDISPNAME>Sales Accounts</DSPDISPNAME></DSPACCNAME>
  <PLAMT><PLSUBAMT></PLSUBAMT><BSMAINAMT>3095828.70</BSMAINAMT></PLAMT>
  <BSNAME><DSPACCNAME><DSPDISPNAME>Sales Register (International)</DSPDISPNAME></DSPACCNAME></BSNAME>
  <BSAMT><BSSUBAMT>3095828.70</BSSUBAMT><BSMAINAMT></BSMAINAMT></BSAMT>
</ENVELOPE>
"#;

        let rows = parse_profit_and_loss_from_xml(xml);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].name, "Sales Accounts");
        assert_eq!(rows[0].main_amount, Some(3095828.70));
        assert_eq!(rows[0].sub_amount, None);
        assert_eq!(rows[1].name, "Sales Register (International)");
        assert_eq!(rows[1].main_amount, None);
        assert_eq!(rows[1].sub_amount, Some(3095828.70));
    }
}
