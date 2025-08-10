use tally_sdk_rust::config::TallyConfig;
use tally_sdk_rust::models::{Group, Ledger, StockItem, ItemInvoice};
use tally_sdk_rust::client::{TallyClient, parse_simple_response};

fn make_client() -> TallyClient {
    let cfg = TallyConfig {
        host: std::env::var("TALLY_HOST").unwrap_or_else(|_| "localhost".into()),
        port: std::env::var("TALLY_PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(9000),
        timeout_secs: 30,
        retry_attempts: 2,
        tally_net_account: None,
        tally_net_password: None,
    };
    TallyClient::new(cfg).expect("client")
}

fn ensure_party_ledger(client: &TallyClient, name: &str) {
    // Create a simple party ledger under 'Sundry Creditors' if not exists
    let exists = client.get_ledgers().unwrap_or_default().iter().any(|(n, _)| n == name);
    if !exists {
        let ledger = Ledger {
            name: name.to_string(),
            parent: Some("Sundry Creditors".into()),
            alias: None,
            opening_balance: None,
            mailing_name: None,
            mailing_address: None,
            mailing_state: None,
            mailing_country: None,
            mailing_pincode: None,
            income_tax_number: None,
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
            account_number: None,
            ifsc_code: None,
            bank_name: None,
            bank_account_holder_name: None,
            swift_code: None,
            branch_name: None,
            bank_bsr_code: None,
            od_limit: None,
            default_transaction_type: None,
            payment_favouring: None,
            transaction_name: None,
            set_as_default: None,
            cheque_cross_comment: None,
            virtual_payment_address: None,
            beneficiary_code: None,
            is_tds_applicable: None,
            tds_deductee_type: None,
            deduct_tds_in_same_voucher: None,
            tds_applicable: None,
            tds_category_date: None,
            tds_category_name: None,
            hsn_applicable_from: None,
            hsn_code: None,
            hsn_description: None,
            hsn_classification_name: None,
            hsn_source_of_details: None,
            gst_applicable_from: None,
            gst_taxability: None,
            gst_source_of_details: None,
            gst_classification_name: None,
            gst_state_name: None,
            gst_rate_duty_head: None,
            gst_rate_valuation_type: None,
            gst_rate: None,
        };
        let _ = client.create_ledger_debug(&ledger).expect("create party ledger");
    }
}

fn ensure_stock_item(client: &TallyClient, name: &str) {
    let exists = client.get_stock_items().unwrap_or_default().iter().any(|(n, _)| n == name);
    if !exists {
        let item = StockItem {
            name: name.to_string(),
            parent: None,
            alias: None,
            base_units: None,
            additional_units: None,
            gst_applicable: Some("Applicable".into()),
            gst_type_of_supply: Some("Goods".into()),
            basic_rate_of_excise: None,
            opening_balance: None,
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
        };
        let _ = client.create_stock_item_debug(&item).expect("create stock item");
    }
}

fn counters(v: &serde_json::Value) -> (i64, i64, i64) {
    let get = |k: &str| v.get(k).and_then(|x| x.as_str()).and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
    (get("CREATED"), get("ALTERED"), get("EXCEPTIONS"))
}

#[test]
fn purchase_voucher_with_new_party_and_item() {
    let client = make_client();
    client.test_connection().expect("connection");

    // Fudged unique names
    let ts = chrono::Utc::now().timestamp();
    let party_name = format!("SDK Supplier {}", ts);
    let item_name = format!("SDK Item {}", ts);

    ensure_party_ledger(&client, &party_name);
    ensure_stock_item(&client, &item_name);

    // Build ItemInvoice for Purchase on 2025-04-02 with 10 units at 50 each
    let vch = ItemInvoice {
        voucher_type: "Purchase".into(),
        date_yyyymmdd: "20250402".into(),
        party_ledger_name: party_name.clone(),
        line_ledger_name: "Purchase A/c".into(),
        item_name: item_name.clone(),
        quantity: 10.0,
        unit: "NOS".into(),
        rate: 50.0,
        narration: Some("SDK test purchase".into()),
        voucher_number: None,
        supplier_invoice_no: Some(format!("INV-{}", ts)),
        supplier_invoice_date_yyyymmdd: Some("20250402".into()),
        receipt_note_no: None,
        receipt_note_date_yyyymmdd: None,
        receipt_doc_no: None,
        dispatched_through: None,
        destination: None,
        carrier_name_agent: None,
        bill_of_lading_no: None,
        bill_of_lading_date_yyyymmdd: None,
        motor_vehicle_no: None,
        inventory_is_deemed_positive: None,
        party_is_deemed_positive: None,
    };

    // Create voucher and verify
    let xml = tally_sdk_rust::xml_builder::XmlBuilder::create_item_invoice_request(&vch.to_map()).expect("build xml");
    println!("\n==== XML Voucher Request ===\n{}\n============================\n", xml);

    let resp = client.post_xml(&xml).expect("post voucher");
    println!("\n==== Raw Response ===\n{}\n====================\n", resp);
    let parsed = parse_simple_response(&resp);
    let (created, altered, exceptions) = counters(&parsed);
    assert_eq!(exceptions, 0, "voucher creation exceptions: {:?}", parsed);
    assert!(created > 0 || altered > 0, "voucher not created/altered: {:?}", parsed);
}


