mod common;

use common::{active_company_label, create_client_from_env};

fn main() {
    let client = create_client_from_env();
    let company = active_company_label(&client);

    let mut groups = client.get_groups().unwrap_or_else(|err| {
        eprintln!("Failed to fetch groups: {err}");
        std::process::exit(1);
    });
    groups.sort_by(|a, b| a.name.cmp(&b.name));

    println!("Groups in company: {company}");
    println!("Found {} group(s)", groups.len());
    for group in groups {
        println!(
            "{} | parent {}",
            group.name,
            group.parent.unwrap_or_else(|| "-".into())
        );
    }
}
