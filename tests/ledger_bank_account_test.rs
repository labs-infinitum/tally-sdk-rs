use tally_sdk_rust::config::TallyConfig;
use tally_sdk_rust::models::Ledger;
use tally_sdk_rust::TallyClient;

fn make_client() -> TallyClient {
    let cfg = TallyConfig {
        host: std::env::var("TALLY_HOST").unwrap_or_else(|_| "localhost".into()),
        port: std::env::var("TALLY_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(9000),
        timeout_secs: 30,
        retry_attempts: 2,
        current_company: std::env::var("TALLY_COMPANY").ok(),
        tally_net_account: None,
        tally_net_password: None,
    };
    TallyClient::new(cfg).expect("client init")
}

fn build_bank_account_ledger() -> Ledger {
    let ts = chrono::Utc::now().timestamp();
    Ledger {
        // Basic
        name: format!("Fcdf Bank A/c {}", ts),
        parent: Some("Bank Accounts".into()),
        alias: Some(vec![
            format!("Fcdf A/c {}", ts),
            format!("Fcdf Main {}", ts),
        ]),

        // Opening balance
        opening_balance: Some(75000.00),

        // Mailing
        mailing_name: None,
        mailing_address: Some(vec![
            "Warno 15 Power Hosuee Road".into(),
            "Near Central Square".into(),
        ]),
        mailing_state: Some("Maharashtra".into()),
        mailing_country: Some("India".into()),
        mailing_pincode: Some("452001".into()),

        // Tax/PAN
        income_tax_number: None,

        // GST top-level
        gst_applicable: None,
        appropriate_for: None,
        gst_appropriate_to: None,
        excise_alloc_type: None,
        gst_type_of_supply: None,
        gst_duty_head: None,
        rate_of_tax_calculation: None,
        tax_type: None,
        bill_credit_period_days: None,
        is_billwise_on: None,
        is_credit_days_chk_on: None,

        // Bank
        account_number: Some("123456789012".into()),
        ifsc_code: Some("SBIN0002838".into()),
        bank_name: Some("State Bank of India".into()),
        bank_account_holder_name: Some("Fcdf Industries Pvt. Ltd.".into()),
        swift_code: Some("SBININBBXXX".into()),
        branch_name: Some("Main Branch, Indore".into()),
        bank_bsr_code: Some("1234567".into()),
        od_limit: None,

        // Payment
        default_transaction_type: None,
        payment_favouring: None,
        transaction_name: None,
        set_as_default: None,
        cheque_cross_comment: None,
        virtual_payment_address: None,
        beneficiary_code: None,

        // TDS
        is_tds_applicable: None,
        tds_deductee_type: None,
        deduct_tds_in_same_voucher: None,
        tds_applicable: None,
        tds_category_date: None,
        tds_category_name: None,

        // HSN
        hsn_applicable_from: None,
        hsn_code: None,
        hsn_description: None,
        hsn_classification_name: None,
        hsn_source_of_details: None,

        // GST nested
        gst_applicable_from: None,
        gst_taxability: None,
        gst_source_of_details: None,
        gst_classification_name: None,
        gst_state_name: None,
        gst_rate_duty_head: None,
        gst_rate_valuation_type: None,
        gst_rate: None,
    }
}

#[test]
fn create_bank_account_ledger_full_details() {
    let client = make_client();
    client.test_connection().expect("connection");
    if client
        .active_company_name()
        .expect("active company lookup")
        .is_none()
    {
        eprintln!("Skipping ledger creation test: no active Tally company loaded and TALLY_COMPANY is not set");
        return;
    }

    let ledger = build_bank_account_ledger();
    // verify before
    let before = client.get_ledgers().unwrap_or_default();
    let existed_before = before.iter().any(|entry| entry.name == ledger.name);
    let resp = client.create_ledger(&ledger).expect("create ledger");

    // If it didn't exist before, ensure counters or presence after polling
    let mut exists_after = false;
    for _ in 0..6 {
        let after = client.get_ledgers().unwrap_or_default();
        if after.iter().any(|entry| entry.name == ledger.name) {
            exists_after = true;
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    if !existed_before {
        // Ensure success (created/altered) or at least existence now, and no exceptions
        assert!(
            resp.exceptions == 0,
            "Tally returned exceptions for creation: {:?}",
            resp
        );
        assert!(
            resp.created > 0 || resp.altered > 0 || exists_after,
            "Expected CREATED/ALTERED counters or to find ledger after creation; resp={:?}",
            resp
        );
    }
}
