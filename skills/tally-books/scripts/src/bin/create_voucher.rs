use serde_json::{json, Map, Value};
use tally_books_skill_cli::{arg_value, client_from_env, has_flag};
use tally_sdk_rust::client::parse_simple_response;
use tally_sdk_rust::xml_builder::XmlBuilder;

fn main() {
    let voucher_type = arg_value("--type").unwrap_or_else(|| {
        eprintln!("Missing required --type Payment|Receipt|Contra|Journal");
        std::process::exit(1);
    });
    let voucher_type_norm = normalize_type(&voucher_type);

    let bank = arg_value("--bank").unwrap_or_else(|| {
        eprintln!("Missing required --bank <bank-ledger-name>");
        std::process::exit(1);
    });
    let account = arg_value("--account").unwrap_or_else(|| {
        eprintln!("Missing required --account <contra-ledger-name>");
        std::process::exit(1);
    });
    let amount = arg_value("--amount")
        .unwrap_or_else(|| {
            eprintln!("Missing required --amount <number>");
            std::process::exit(1);
        })
        .parse::<f64>()
        .unwrap_or_else(|_| {
            eprintln!("Invalid --amount");
            std::process::exit(1);
        });
    if amount <= 0.0 {
        eprintln!("--amount must be > 0");
        std::process::exit(1);
    }

    let date = arg_value("--date").unwrap_or_else(|| {
        eprintln!("Missing required --date YYYYMMDD");
        std::process::exit(1);
    });
    if date.len() != 8 || !date.chars().all(|c| c.is_ascii_digit()) {
        eprintln!("--date must be YYYYMMDD");
        std::process::exit(1);
    }

    let narration = arg_value("--narration").unwrap_or_else(|| {
        format!("{voucher_type_norm} via tally-books skill")
    });
    let voucher_number = arg_value("--voucher-number").unwrap_or_else(|| {
        format!("TB-{}", chrono::Local::now().format("%H%M%S"))
    });
    let dry_run = has_flag("--dry-run");
    let debug = has_flag("--debug");

    let (debit_ledger, credit_ledger) = match voucher_type_norm.as_str() {
        "Payment" | "Journal" | "Contra" => (account.clone(), bank.clone()),
        "Receipt" => (bank.clone(), account.clone()),
        other => {
            eprintln!("Unsupported --type `{other}`. Use Payment, Receipt, Contra, or Journal.");
            std::process::exit(1);
        }
    };

    let plan = json!({
        "voucher_type": voucher_type_norm,
        "date": date,
        "amount": amount,
        "debit": debit_ledger,
        "credit": credit_ledger,
        "narration": narration,
        "voucher_number": voucher_number,
        "dry_run": dry_run,
    });
    println!("{}", serde_json::to_string_pretty(&plan).unwrap());

    if dry_run {
        return;
    }

    let client = client_from_env();
    ensure_ledger_exists(&client, &bank);
    ensure_ledger_exists(&client, &account);

    let voucher_map = build_two_line_voucher(
        &voucher_type_norm,
        &date,
        &narration,
        &voucher_number,
        &debit_ledger,
        &credit_ledger,
        amount,
    );

    let xml = XmlBuilder::create_voucher_request(&voucher_map).unwrap_or_else(|err| {
        eprintln!("Failed to build voucher XML: {err}");
        std::process::exit(1);
    });
    if debug {
        eprintln!("==== XML ====\n{xml}\n=============");
    }

    let resp = client.post_xml(&xml).unwrap_or_else(|err| {
        eprintln!("Failed to post voucher: {err}");
        std::process::exit(1);
    });
    if debug {
        eprintln!("==== RESP ====\n{resp}\n==============");
    }

    let result = parse_simple_response(&resp);
    let out = json!({
        "created": result.created,
        "altered": result.altered,
        "ignored": result.ignored,
        "errors": result.errors,
        "exceptions": result.exceptions,
        "last_voucher_id": result.last_voucher_id,
        "line_errors": result.line_errors,
    });
    println!("{}", serde_json::to_string_pretty(&out).unwrap());

    if result.has_errors() || !result.created_or_altered() {
        eprintln!("Voucher was not created successfully.");
        std::process::exit(1);
    }
}

fn normalize_type(value: &str) -> String {
    let lower = value.to_ascii_lowercase();
    match lower.as_str() {
        "payment" | "pay" => "Payment".into(),
        "receipt" | "rec" => "Receipt".into(),
        "contra" => "Contra".into(),
        "journal" | "jrnl" => "Journal".into(),
        _ => value.to_string(),
    }
}

fn ensure_ledger_exists(client: &tally_sdk_rust::TallyClient, name: &str) {
    let ledgers = client.get_ledgers().unwrap_or_else(|err| {
        eprintln!("Failed to fetch ledgers: {err}");
        std::process::exit(1);
    });
    if !ledgers.iter().any(|ledger| ledger.name == name) {
        eprintln!("Ledger `{name}` not found. Create it in Tally or pick an existing name.");
        std::process::exit(1);
    }
}

fn build_two_line_voucher(
    voucher_type: &str,
    date: &str,
    narration: &str,
    voucher_number: &str,
    debit_ledger: &str,
    credit_ledger: &str,
    amount: f64,
) -> Map<String, Value> {
    let debit_amount = format!("-{amount:.2}");
    let credit_amount = format!("{amount:.2}");

    let mut map = Map::new();
    map.insert("VOUCHERTYPENAME".into(), json!(voucher_type));
    map.insert("OBJVIEW".into(), json!("Accounting Voucher View"));
    map.insert("PERSISTEDVIEW".into(), json!("Accounting Voucher View"));
    map.insert("ISINVOICE".into(), json!("No"));
    map.insert("DATE".into(), json!(date));
    map.insert("EFFECTIVEDATE".into(), json!(date));
    map.insert("VOUCHERNUMBER".into(), json!(voucher_number));
    map.insert("NARRATION".into(), json!(narration));
    map.insert("PARTYLEDGERNAME".into(), json!(debit_ledger));

    map.insert(
        "LEDGERENTRIES.LIST".into(),
        json!([
            {
                "LEDGERNAME": debit_ledger,
                "ISDEEMEDPOSITIVE": "Yes",
                "ISPARTYLEDGER": "Yes",
                "AMOUNT": debit_amount,
            },
            {
                "LEDGERNAME": credit_ledger,
                "ISDEEMEDPOSITIVE": "No",
                "ISPARTYLEDGER": "No",
                "AMOUNT": credit_amount,
            }
        ]),
    );

    map
}
