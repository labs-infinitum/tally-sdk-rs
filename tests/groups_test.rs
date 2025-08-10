use tally_sdk_rust::{TallyClient};
use tally_sdk_rust::config::TallyConfig;
use tally_sdk_rust::models::Group;

fn make_client() -> TallyClient {
    let cfg = TallyConfig { host: std::env::var("TALLY_HOST").unwrap_or_else(|_| "localhost".into()), port: std::env::var("TALLY_PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(9000), timeout_secs: 30, retry_attempts: 2, tally_net_account: None, tally_net_password: None };
    TallyClient::new(cfg).expect("client")
}

#[test]
fn create_and_fetch_groups() {
    // This test hits a running TallyPrime at TALLY_HOST:TALLY_PORT
    let client = make_client();

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

    let _ = client.create_group(&grp).expect("create group");

    // List groups after
    let after = client.get_groups().expect("fetch groups after");

    // Verify that the new group appears in 'after' but not necessarily in 'before'
    let had_before = before.iter().any(|(n, _)| n == &unique_name);
    let has_after = after.iter().any(|(n, _)| n == &unique_name);

    assert!(!had_before || has_after, "Group should be present after creation");
    assert!(has_after, "Created group not found in groups list after creation");
}

#[test]
fn create_group_under_another_group() {
    let client = make_client();

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
    let _ = client.create_group(&parent).expect("create parent group");

    // Create child under parent
    let child_name = format!("SDK-GRP-CHILD-{}", chrono::Utc::now().timestamp());
    let child = Group { parent: Some(parent_name.clone()), name: child_name.clone(), ..parent };
    let _ = client.create_group(&child).expect("create child group");

    // Verify child lists with parent
    let groups = client.get_groups().expect("fetch groups");
    let found = groups.iter().find(|(n, _)| n == &child_name);
    assert!(found.is_some(), "child group not found");
    if let Some((_, p)) = found {
        if let Some(pp) = p {
            assert_eq!(pp, &parent_name, "child group's parent mismatch");
        }
    }
}


