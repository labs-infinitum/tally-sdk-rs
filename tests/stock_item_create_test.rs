use tally_sdk_rust::config::TallyConfig;
use tally_sdk_rust::models::StockItem;
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
    TallyClient::new(cfg).expect("client")
}

fn build_stock_item() -> StockItem {
    let ts = chrono::Utc::now().timestamp();
    StockItem {
        name: format!("SDK Item {}", ts),
        parent: None,
        alias: None,
        base_units: None,
        additional_units: None,
        gst_applicable: Some("Applicable".into()),
        gst_type_of_supply: Some("Goods".into()),
        basic_rate_of_excise: None,
        opening_balance: None,
        // HSN/GST nested
        hsn_applicable_from: None,
        hsn_code: Some("8517".into()),
        hsn_description: Some("Mobile phones and smartphones".into()),
        hsn_classification_name: None,
        hsn_source_of_details: Some("Specify Details Here".into()),
        gst_applicable_from: None,
        gst_taxability: Some("Taxable".into()),
        gst_source_of_details: Some("Specify Details Here".into()),
        gst_classification_name: None,
        gst_state_name: None,
        gst_rate_duty_head: Some("IGST".into()),
        gst_rate_valuation_type: Some("Based on Value".into()),
        gst_rate: Some(18.0),
    }
}

#[test]
fn create_stock_item_and_verify() {
    let client = make_client();
    client.test_connection().expect("connection");
    if client
        .active_company_name()
        .expect("active company lookup")
        .is_none()
    {
        eprintln!("Skipping stock item creation test: no active Tally company loaded and TALLY_COMPANY is not set");
        return;
    }

    let item = build_stock_item();

    // Before
    let before = client.get_stock_items().unwrap_or_default();
    let existed_before = before.iter().any(|entry| entry.name == item.name);

    // Create with debug to print XML when needed
    let resp = client
        .create_stock_item_debug(&item)
        .expect("create stock item");
    if !resp.line_errors.is_empty() {
        eprintln!("LINEERROR: {:?}", resp.line_errors);
    }

    // Poll verify
    let mut exists_after = false;
    for _ in 0..6 {
        let after = client.get_stock_items().unwrap_or_default();
        if after.iter().any(|entry| entry.name == item.name) {
            exists_after = true;
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    if !existed_before {
        assert_eq!(
            resp.exceptions, 0,
            "Tally returned exceptions for stock item creation: {:?}",
            resp
        );
        assert!(
            resp.created > 0 || resp.altered > 0 || exists_after,
            "Expected CREATED/ALTERED or to find stock item after creation; resp={:?}",
            resp
        );
    } else {
        assert!(exists_after, "Stock item should exist after creation call");
    }
}
