use dairyshop::models::Greeting;
use dairyshop::parser;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let name = if args.len() > 1 {
        parser::parse_name(&args[1])
    } else {
        "World".to_string()
    };

    let greeting = Greeting::new(&name);
    println!("{}", greeting.message());
}
