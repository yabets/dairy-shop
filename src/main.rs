use dairyshop::models::{Herd, Stock};
use dairyshop::parser;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <herd_xml_file> <days_elapsed>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let days_elapsed: u32 = match args[2].parse() {
        Ok(d) => d,
        Err(_) => {
            eprintln!("{} must be a positive integer", args[2]);
            std::process::exit(1);
        }
    };

    let herd = parser::parse_herd_xml(file_path)?;
    let stock = herd.calculate_stock(days_elapsed);

    display_results(&herd, &stock, days_elapsed);

    Ok(())
}

fn display_results(herd: &Herd, stock: &Stock, days_elapsed: u32) {
    println!("In Stock:");
    println!("  {:.3} liters of milk", stock.milk_liters);
    println!("  {} skins of wool", stock.wool_skins);

    println!("Herd:");
    for yak in &herd.yaks {
        if yak.is_alive(days_elapsed) {
            println!(
                "  {} {:.2} years old",
                yak.name,
                yak.current_age_years(days_elapsed)
            );
        }
    }
}
