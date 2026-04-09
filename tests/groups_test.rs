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

fn get_counter(v: &serde_json::Value, key: &str) -> i64 {
    v.get(key)
        .and_then(|x| x.as_str())
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0)
}

#[test]
fn create_and_fetch_groups() {
    // This test hits a running TallyPrime at TALLY_HOST:TALLY_PORT
    let client = make_client();
    if client
        .active_company_name()
        .expect("active company lookup")
        .is_none()
    {
        eprintln!("Skipping group integration test: no active Tally company loaded and TALLY_COMPANY is not set");
        return;
    }

    // List groups before
    let before = client.get_groups().unwrap_or_default();

    // Pick a unique group name
    let unique_name = format!("SDK-GRP-{}", chrono::Utc::now().timestamp());

    // Create group under a common parent or empty parent
    let grp = Group {
        name: unique_name.clone(),
        parent: None,
        group_type: None,
        alias: None,
        basic_group_is_calculable: None,
        is_addable: None,
        is_subledger: None,
        addl_alloc_type: None,
        as_original: None,
        affects_gross_profit: None,
        hsn_applicable_from: None,
        hsn_code: None,
        hsn_description: None,
        hsn_classification_name: None,
        hsn_source_of_details: None,
        gst_applicable_from: None,
        gst_taxability: None,
        gst_source_of_details: None,
        gst_classification_name: None,
        gst_rate_duty_head: None,
        gst_rate_valuation_type: None,
        gst_rate: None,
        gst_state_name: None,
    };

    let resp = client.create_group(&grp).expect("create group");
    let created = get_counter(&resp, "CREATED");
    let altered = get_counter(&resp, "ALTERED");
    let exceptions = get_counter(&resp, "EXCEPTIONS");

    let mut has_after = false;
    for _ in 0..6 {
        let after = client.get_groups().expect("fetch groups after");
        has_after = after.iter().any(|(n, _)| n == &unique_name);
        if has_after {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    let had_before = before.iter().any(|(n, _)| n == &unique_name);

    if !had_before {
        assert_eq!(
            exceptions, 0,
            "Tally returned exceptions for group creation: {:?}",
            resp
        );
        assert!(
            created > 0 || altered > 0 || has_after,
            "Created group not found in groups list after creation; resp={:?}",
            resp
        );
    } else {
        assert!(
            has_after,
            "Group should still be present after creation call"
        );
    }
}

#[test]
fn create_group_under_another_group() {
    let client = make_client();
    if client
        .active_company_name()
        .expect("active company lookup")
        .is_none()
    {
        eprintln!("Skipping group hierarchy test: no active Tally company loaded and TALLY_COMPANY is not set");
        return;
    }

    // Create parent
    let parent_name = format!("SDK-GRP-PARENT-{}", chrono::Utc::now().timestamp());
    let parent = Group {
        name: parent_name.clone(),
        parent: None,
        group_type: None,
        alias: None,
        basic_group_is_calculable: None,
        is_addable: None,
        is_subledger: None,
        addl_alloc_type: None,
        as_original: None,
        affects_gross_profit: None,
        hsn_applicable_from: None,
        hsn_code: None,
        hsn_description: None,
        hsn_classification_name: None,
        hsn_source_of_details: None,
        gst_applicable_from: None,
        gst_taxability: None,
        gst_source_of_details: None,
        gst_classification_name: None,
        gst_rate_duty_head: None,
        gst_rate_valuation_type: None,
        gst_rate: None,
        gst_state_name: None,
    };
    let parent_resp = client.create_group(&parent).expect("create parent group");
    let parent_created = get_counter(&parent_resp, "CREATED");
    let parent_altered = get_counter(&parent_resp, "ALTERED");
    let parent_exceptions = get_counter(&parent_resp, "EXCEPTIONS");
    assert_eq!(
        parent_exceptions, 0,
        "Parent group creation returned exceptions: {:?}",
        parent_resp
    );
    assert!(
        parent_created > 0 || parent_altered > 0,
        "Parent group was not created or altered: {:?}",
        parent_resp
    );

    // Create child under parent
    let child_name = format!("SDK-GRP-CHILD-{}", chrono::Utc::now().timestamp());
    let child = Group {
        parent: Some(parent_name.clone()),
        name: child_name.clone(),
        ..parent
    };
    let child_resp = client.create_group(&child).expect("create child group");
    let child_created = get_counter(&child_resp, "CREATED");
    let child_altered = get_counter(&child_resp, "ALTERED");
    let child_exceptions = get_counter(&child_resp, "EXCEPTIONS");
    assert_eq!(
        child_exceptions, 0,
        "Child group creation returned exceptions: {:?}",
        child_resp
    );
    assert!(
        child_created > 0 || child_altered > 0,
        "Child group was not created or altered: {:?}",
        child_resp
    );

    let mut found: Option<(String, Option<String>)> = None;
    for _ in 0..6 {
        let groups = client.get_groups().expect("fetch groups");
        found = groups.iter().find(|(n, _)| n == &child_name).cloned();
        if found.is_some() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    assert!(
        found.is_some() || child_created > 0 || child_altered > 0,
        "child group not found and child create response did not indicate success: {:?}",
        child_resp
    );
    if let Some((_, p)) = found {
        if let Some(pp) = p {
            assert_eq!(pp, parent_name, "child group's parent mismatch");
        }
    }
}
