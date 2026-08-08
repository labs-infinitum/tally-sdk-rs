mod common;

use common::{
    active_company_label, create_client_from_env, format_yyyymmdd, resolve_financial_period,
};

fn main() {
    let client = create_client_from_env();
    let company = active_company_label(&client);
    let (from_date, to_date) = resolve_financial_period();

    let computation = client
        .get_gst_computation(Some(&from_date), Some(&to_date))
        .unwrap_or_else(|err| {
            eprintln!("Failed to fetch GST Computation: {err}");
            std::process::exit(1);
        });

    println!(
        "GST Computation for {} from {} to {}",
        company,
        format_yyyymmdd(&from_date),
        format_yyyymmdd(&to_date)
    );
    if let Some(name_field) = &computation.name_field {
        println!("Name field: {name_field}");
    }
    println!("Found {} computation row(s)", computation.entries.len());
    for row in &computation.entries {
        println!(
            "{} | taxable {:?} | IGST {:?} | CGST {:?} | SGST {:?} | cess {:?} | tax {:?}",
            row.particulars,
            row.taxable_value,
            row.igst,
            row.cgst,
            row.sgst,
            row.cess,
            row.total_tax
        );
    }

    let gstr1 = client
        .get_gstr1(&from_date, &to_date)
        .unwrap_or_else(|err| {
            eprintln!("Failed to build GSTR-1 summary: {err}");
            std::process::exit(1);
        });

    println!();
    println!(
        "GSTR-1 (voucher-derived) for {} from {} to {}",
        company,
        format_yyyymmdd(&from_date),
        format_yyyymmdd(&to_date)
    );
    println!("Source: {:?}", gstr1.source);
    println!("Company GSTIN: {:?}", gstr1.company_gstin);
    println!(
        "B2B={} B2CL={} B2CS={} CDNR={} HSN={} docs={}",
        gstr1.b2b.len(),
        gstr1.b2cl.len(),
        gstr1.b2cs.len(),
        gstr1.cdnr.len(),
        gstr1.hsn.len(),
        gstr1.documents.len()
    );

    for inv in &gstr1.b2b {
        println!(
            "B2B | {} | {:?} | {} | taxable {:.2} | tax {:.2}",
            inv.party_gstin,
            inv.invoice_number,
            format_yyyymmdd(&inv.invoice_date),
            inv.taxes.taxable_value,
            inv.taxes.total_tax()
        );
    }
    for inv in &gstr1.b2cl {
        println!(
            "B2CL | {:?} | {:?} | taxable {:.2}",
            inv.party_name, inv.invoice_number, inv.taxes.taxable_value
        );
    }
    for row in &gstr1.b2cs {
        println!(
            "B2CS | pos {:?} | rate {:?} | invoices {} | taxable {:.2}",
            row.place_of_supply, row.rate, row.invoice_count, row.taxes.taxable_value
        );
    }
    for note in &gstr1.cdnr {
        println!(
            "CDNR | {:?} | {:?} | {} | taxable {:.2}",
            note.party_gstin, note.note_number, note.note_type, note.taxes.taxable_value
        );
    }
    for hsn in &gstr1.hsn {
        println!(
            "HSN | {} | {:?} | qty {:?} | taxable {:.2}",
            hsn.hsn_code, hsn.description, hsn.total_quantity, hsn.taxes.taxable_value
        );
    }
}
