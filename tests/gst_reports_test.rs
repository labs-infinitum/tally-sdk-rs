use tallyprime_sdk::config::TallyConfig;
use tallyprime_sdk::TallyClient;

fn make_client() -> TallyClient {
    let cfg = TallyConfig {
        host: std::env::var("TALLY_HOST").unwrap_or_else(|_| "localhost".into()),
        port: std::env::var("TALLY_PORT")
            .ok()
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(9000),
        current_company: std::env::var("TALLY_COMPANY").ok(),
        ..Default::default()
    };
    TallyClient::new(cfg).expect("client")
}

#[test]
#[ignore = "requires live TallyPrime"]
fn fetch_gst_computation_and_gstr1() {
    let client = make_client();
    if client
        .active_company_name()
        .expect("active company lookup")
        .is_none()
    {
        eprintln!(
            "Skipping GST reports test: no active Tally company loaded and TALLY_COMPANY is not set"
        );
        return;
    }

    let computation = client
        .get_gst_computation(Some("20250401"), Some("20250430"))
        .expect("gst computation");
    assert!(
        !computation.entries.is_empty(),
        "expected GST Computation rows"
    );

    let gstr1 = client
        .get_gstr1("20250401", "20250430")
        .expect("gstr1 summary");
    assert_eq!(gstr1.from_date, "20250401");
    assert_eq!(gstr1.to_date, "20250430");
    // Document summary may be empty when the period has no sales vouchers.
    let _ = (
        gstr1.b2b.len(),
        gstr1.b2cl.len(),
        gstr1.b2cs.len(),
        gstr1.cdnr.len(),
        gstr1.hsn.len(),
        gstr1.documents.len(),
    );
}
