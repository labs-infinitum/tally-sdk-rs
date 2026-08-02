mod common;

use common::{active_company_label, arg_value, create_client_from_env, has_flag};
use tally_sdk_rust::Ledger;

fn main() {
    let client = create_client_from_env();
    let company = active_company_label(&client);

    let name = arg_value("--name").unwrap_or_else(|| {
        format!(
            "SDK Demo Ledger {}",
            chrono::Local::now().format("%Y%m%d%H%M%S")
        )
    });
    let parent = arg_value("--parent").unwrap_or_else(|| "Sundry Debtors".into());
    let opening_balance = arg_value("--opening-balance").and_then(|value| {
        value.parse::<f64>().map(Some).unwrap_or_else(|_| {
            eprintln!("Invalid --opening-balance value `{value}`. Expected a number.");
            std::process::exit(1);
        })
    });
    let debug = has_flag("--debug");

    let ledger = simple_ledger(name.clone(), parent.clone(), opening_balance);

    println!("Creating ledger in company: {company}");
    println!("name={name} | parent={parent}");
    if let Some(balance) = opening_balance {
        println!("opening_balance={balance:.2}");
    }

    let result = if debug {
        client.create_ledger_debug(&ledger)
    } else {
        client.create_ledger(&ledger)
    }
    .unwrap_or_else(|err| {
        eprintln!("Failed to create ledger: {err}");
        std::process::exit(1);
    });

    println!(
        "created={} | altered={} | ignored={} | errors={} | exceptions={}",
        result.created, result.altered, result.ignored, result.errors, result.exceptions
    );
    if let Some(ref master_id) = result.last_master_id {
        println!("last_master_id={master_id}");
    }
    for line_error in &result.line_errors {
        eprintln!("line_error: {line_error}");
    }

    if result.has_errors() {
        std::process::exit(1);
    }
}

fn simple_ledger(name: String, parent: String, opening_balance: Option<f64>) -> Ledger {
    Ledger {
        name,
        parent: Some(parent),
        alias: None,
        opening_balance,
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
    }
}
