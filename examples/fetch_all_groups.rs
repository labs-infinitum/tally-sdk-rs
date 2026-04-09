mod common;

use common::{active_company_label, create_client_from_env};

fn main() {
    let client = create_client_from_env();
    let company = active_company_label(&client);

    let mut groups = client.get_groups().unwrap_or_else(|err| {
        eprintln!("Failed to fetch groups: {err}");
        std::process::exit(1);
    });
    groups.sort_by(|a, b| a.0.cmp(&b.0));

    println!("Groups in company: {company}");
    println!("Found {} group(s)", groups.len());
    for (name, parent) in groups {
        println!("{} | parent {}", name, parent.unwrap_or_else(|| "-".into()));
    }
}
