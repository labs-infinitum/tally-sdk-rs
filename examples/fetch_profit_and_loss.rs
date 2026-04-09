mod common;

use common::{
    active_company_label, create_client_from_env, format_amount, format_yyyymmdd, has_flag,
    resolve_financial_period,
};

fn main() {
    let client = create_client_from_env();
    let company = active_company_label(&client);
    let (from_date, to_date) = resolve_financial_period();
    let explode_flag = !has_flag("--flat");

    let rows = client
        .get_profit_and_loss(Some(&from_date), Some(&to_date), explode_flag)
        .unwrap_or_else(|err| {
            eprintln!("Failed to fetch profit and loss: {err}");
            std::process::exit(1);
        });

    println!(
        "Profit and Loss for {} from {} to {}",
        company,
        format_yyyymmdd(&from_date),
        format_yyyymmdd(&to_date)
    );
    println!("Found {} row(s)", rows.len());
    for row in rows {
        println!(
            "{} | main {} | sub {}",
            row.name,
            format_amount(row.main_amount),
            format_amount(row.sub_amount)
        );
    }
}
