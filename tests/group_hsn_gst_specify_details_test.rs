use tally_sdk_rust::config::TallyConfig;
use tally_sdk_rust::models::Group;
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

fn build_group() -> Group {
    let ts = chrono::Utc::now().timestamp();
    Group {
        name: format!("TP Group HSN GST Test {}", ts),
        alias: Some(vec![format!("TP-GST-HSN-{}", ts), format!("TPGHG-{}", ts)]),
        parent: None,
        group_type: Some("Expenses".into()),
        affects_gross_profit: Some("Yes".into()),

        basic_group_is_calculable: Some("Yes".into()),
        is_addable: Some("Yes".into()),
        is_subledger: Some("Yes".into()),
        as_original: Some("Yes".into()),
        addl_alloc_type: Some("Appropriate by Qty".into()),

        // HSN (Specify Details Here)
        hsn_source_of_details: Some("Specify Details Here".into()),
        hsn_code: Some("8517".into()),
        hsn_description: Some("Mobile phones and smartphones".into()),
        hsn_applicable_from: None,
        hsn_classification_name: None,

        // GST (Specify Details Here)
        gst_source_of_details: Some("Specify Details Here".into()),
        gst_taxability: Some("Taxable".into()),
        gst_rate_duty_head: Some("IGST".into()),
        gst_rate_valuation_type: Some("Based on Value".into()),
        gst_rate: Some(25.0),
        gst_applicable_from: None,
        gst_classification_name: None,
        gst_state_name: None,
    }
}

fn get_counter(v: &serde_json::Value, key: &str) -> i64 {
    v.get(key)
        .and_then(|x| x.as_str())
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0)
}

#[test]
fn create_group_hsn_gst_specify_details() {
    let client = make_client();
    client.test_connection().expect("connection");
    if client
        .active_company_name()
        .expect("active company lookup")
        .is_none()
    {
        eprintln!("Skipping group creation test: no active Tally company loaded and TALLY_COMPANY is not set");
        return;
    }

    let group = build_group();

    // Before
    let before = client.get_groups().unwrap_or_default();
    let existed_before = before.iter().any(|entry| entry.name == group.name);

    // Create
    let resp = client.create_group_debug(&group).expect("create group");
    let created = get_counter(&resp, "CREATED");
    let altered = get_counter(&resp, "ALTERED");
    let exceptions = get_counter(&resp, "EXCEPTIONS");

    // After (with small polling for eventual consistency)
    let mut exists_after = false;
    for _ in 0..6 {
        let after = client.get_groups().unwrap_or_default();
        if after.iter().any(|entry| entry.name == group.name) {
            exists_after = true;
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    if !existed_before {
        assert_eq!(
            exceptions, 0,
            "Tally returned exceptions for group creation: {:?}",
            resp
        );
        assert!(
            created > 0 || altered > 0 || exists_after,
            "Expected CREATED/ALTERED or presence after; resp={:?}",
            resp
        );
    } else {
        assert!(exists_after, "Group should exist after creation call");
    }
}
