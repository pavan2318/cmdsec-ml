mod dataset;
mod features;
mod rules;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("cargo run -- generate");
        println!("cargo run -- features-train");
        println!("cargo run -- features-test");
        return;
    }

    match args[1].as_str() {
        "generate" => dataset::generate_datasets(),
        "features-train" => features::extract_from_file("train.json", "train.csv"),
        "features-test" => features::extract_from_file("test.json", "test.csv"),
        "rules-train" => rules::evaluate("train.json"),
        "rules-test" => rules::evaluate("test.json"),    
        _ => println!("Unknown command"),
    }
}
