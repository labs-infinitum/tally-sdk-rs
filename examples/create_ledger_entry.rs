mod common;

use common::{active_company_label, arg_value, create_client_from_env, has_flag};
use serde_json::{json, Map, Value};
use tally_sdk_rust::client::parse_simple_response;
use tally_sdk_rust::{Ledger, xml_builder::XmlBuilder};

fn main() {
    let client = create_client_from_env();
    let company = active_company_label(&client);

    // Party ledger = the ledger you created (e.g. Acme Traders under Sundry Debtors).
    let party_ledger = arg_value("--party")
        .or_else(|| arg_value("--credit"))
        .unwrap_or_else(|| {
            eprintln!("Missing required --party <ledger-name> (the ledger you created).");
            eprintln!(
                "Example: cargo run --example create_ledger_entry -- --party \"Acme Traders\" --amount 1000 --date 20260701"
            );
            std::process::exit(1);
        });

    let amount = arg_value("--amount")
        .unwrap_or_else(|| "1000".into())
        .parse::<f64>()
        .unwrap_or_else(|_| {
            eprintln!("Invalid --amount. Expected a number.");
            std::process::exit(1);
        });
    if amount <= 0.0 {
        eprintln!("--amount must be greater than 0.");
        std::process::exit(1);
    }

    // Default to Sales accounting voucher: Dr Party / Cr Sales.
    // That is the standard way to put an entry into a Sundry Debtors ledger.
    let voucher_type = arg_value("--voucher-type").unwrap_or_else(|| "Sales".into());
    let sales_ledger = arg_value("--account").unwrap_or_else(|| "SDK Sales".into());
    let date = arg_value("--date").unwrap_or_else(default_voucher_date);
    let narration = arg_value("--narration").unwrap_or_else(|| {
        format!("SDK ledger entry against {party_ledger}")
    });
    let bill_ref = arg_value("--bill-ref").unwrap_or_else(|| {
        format!("SDK-{}", chrono::Local::now().format("%Y%m%d%H%M%S"))
    });
    let voucher_number = arg_value("--voucher-number").unwrap_or_else(|| {
        format!("SDK-{}", chrono::Local::now().format("%H%M%S"))
    });
    let debug = has_flag("--debug");

    ensure_ledger_exists(&client, &party_ledger);
    ensure_sales_ledger(&client, &sales_ledger);

    let voucher_map = build_sales_accounting_voucher_map(
        &voucher_type,
        &date,
        &narration,
        &party_ledger,
        &sales_ledger,
        amount,
        &bill_ref,
        &voucher_number,
    );

    let xml = XmlBuilder::create_voucher_request(&voucher_map).unwrap_or_else(|err| {
        eprintln!("Failed to build voucher XML: {err}");
        std::process::exit(1);
    });

    println!("Creating {voucher_type} voucher in company: {company}");
    println!("date={date} | amount={amount:.2} | bill_ref={bill_ref} | voucher_number={voucher_number}");
    println!("Dr {party_ledger} | Cr {sales_ledger}");
    if debug {
        println!("\n==== XML Voucher Request (before company injection) ===\n{xml}\n============================\n");
    }

    let resp = client.post_xml(&xml).unwrap_or_else(|err| {
        eprintln!("Failed to post voucher: {err}");
        std::process::exit(1);
    });
    if debug {
        println!("\n==== Raw Response ===\n{resp}\n====================\n");
    }

    let result = parse_simple_response(&resp);
    println!(
        "created={} | altered={} | ignored={} | errors={} | exceptions={}",
        result.created, result.altered, result.ignored, result.errors, result.exceptions
    );
    if let Some(ref voucher_id) = result.last_voucher_id {
        println!("last_voucher_id={voucher_id}");
    }
    for line_error in &result.line_errors {
        eprintln!("line_error: {line_error}");
    }

    if result.has_errors() || !(result.created > 0 || result.altered > 0) {
        eprintln!("Voucher was not created successfully.");
        eprintln!("Hints:");
        eprintln!("  - This Tally company rejects many mid-month dates over XML");
        eprintln!("  - Try --date 20260701 or --date 20260731 (first/last day of month)");
        eprintln!("  - In Tally, check Company Alter > period / lock dates");
        eprintln!("  - Re-run with --debug to inspect the XML request/response");
        if !debug {
            eprintln!("\n==== Raw Response ===\n{resp}\n====================");
        }
        std::process::exit(1);
    }
}

fn default_voucher_date() -> String {
    // This company currently rejects many mid-month voucher dates over XML
    // (e.g. 20260715 fails, while 20260701 / 20260731 succeed). Default to the
    // first day of the current month in YYYYMMDD.
    chrono::Local::now().format("%Y%m01").to_string()
}

fn ensure_ledger_exists(client: &tally_sdk_rust::TallyClient, name: &str) {
    let ledgers = client.get_ledgers().unwrap_or_else(|err| {
        eprintln!("Failed to fetch ledgers: {err}");
        std::process::exit(1);
    });
    if !ledgers.iter().any(|ledger| ledger.name == name) {
        eprintln!("Ledger `{name}` was not found in the active company.");
        eprintln!("Create it first with:");
        eprintln!("  cargo run --example create_ledger -- --name \"{name}\"");
        std::process::exit(1);
    }
}

fn ensure_sales_ledger(client: &tally_sdk_rust::TallyClient, name: &str) {
    let ledgers = client.get_ledgers().unwrap_or_else(|err| {
        eprintln!("Failed to fetch ledgers: {err}");
        std::process::exit(1);
    });
    if ledgers.iter().any(|ledger| ledger.name == name) {
        return;
    }

    println!("Sales ledger `{name}` not found; creating under Sales Accounts...");
    let ledger = Ledger {
        name: name.to_string(),
        parent: Some("Sales Accounts".into()),
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

    let result = client.create_ledger(&ledger).unwrap_or_else(|err| {
        eprintln!("Failed to create sales ledger `{name}`: {err}");
        std::process::exit(1);
    });
    if result.has_errors() {
        eprintln!("Failed to create sales ledger `{name}`: {:?}", result);
        std::process::exit(1);
    }
}

fn build_sales_accounting_voucher_map(
    voucher_type: &str,
    date: &str,
    narration: &str,
    party_ledger: &str,
    sales_ledger: &str,
    amount: f64,
    bill_ref: &str,
    voucher_number: &str,
) -> Map<String, Value> {
    // Sales accounting voucher (TallyHelp sample shape):
    // Dr Party  => ISDEEMEDPOSITIVE=Yes, AMOUNT=-X
    // Cr Sales  => ISDEEMEDPOSITIVE=No,  AMOUNT=+X
    let party_amount = format!("-{amount:.2}");
    let sales_amount = format!("{amount:.2}");

    let mut map = Map::new();
    map.insert("VOUCHERTYPENAME".into(), json!(voucher_type));
    map.insert("OBJVIEW".into(), json!("Accounting Voucher View"));
    map.insert("PERSISTEDVIEW".into(), json!("Accounting Voucher View"));
    map.insert("ISINVOICE".into(), json!("No"));
    map.insert("DATE".into(), json!(date));
    map.insert("EFFECTIVEDATE".into(), json!(date));
    map.insert("VOUCHERNUMBER".into(), json!(voucher_number));
    map.insert("NARRATION".into(), json!(narration));
    map.insert("PARTYLEDGERNAME".into(), json!(party_ledger));

    map.insert(
        "LEDGERENTRIES.LIST".into(),
        json!([
            {
                "LEDGERNAME": party_ledger,
                "ISDEEMEDPOSITIVE": "Yes",
                "ISPARTYLEDGER": "Yes",
                "AMOUNT": party_amount,
                "BILLALLOCATIONS.LIST": {
                    "NAME": bill_ref,
                    "BILLTYPE": "New Ref",
                    "AMOUNT": party_amount,
                }
            },
            {
                "LEDGERNAME": sales_ledger,
                "ISDEEMEDPOSITIVE": "No",
                "ISPARTYLEDGER": "No",
                "AMOUNT": sales_amount,
            }
        ]),
    );

    map
}
