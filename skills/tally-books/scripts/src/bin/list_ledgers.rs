use serde_json::json;
use tally_books_skill_cli::{arg_value, client_from_env, has_flag};

fn main() {
    let client = client_from_env();
    let ledgers = client.get_ledgers().unwrap_or_else(|err| {
        eprintln!("Failed to fetch ledgers: {err}");
        std::process::exit(1);
    });

    let query = arg_value("--filter")
        .or_else(|| arg_value("-q"))
        .map(|q| q.to_lowercase());

    let filtered: Vec<_> = ledgers
        .into_iter()
        .filter(|ledger| {
            query.as_ref().map_or(true, |q| {
                ledger.name.to_lowercase().contains(q)
                    || ledger
                        .parent
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(q)
            })
        })
        .collect();

    if has_flag("--json") {
        let rows: Vec<_> = filtered
            .iter()
            .map(|ledger| {
                json!({
                    "name": ledger.name,
                    "parent": ledger.parent,
                })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&rows).unwrap());
        return;
    }

    for ledger in &filtered {
        println!(
            "{}\t{}",
            ledger.name,
            ledger.parent.as_deref().unwrap_or("")
        );
    }
    eprintln!("# {} ledger(s)", filtered.len());
}
