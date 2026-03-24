use crate::dataset::Command;
use std::fs;

pub fn predict(cmd: &str) -> u8 {
    let s = cmd.to_lowercase();

    if s.contains("powershell -enc") { return 1; }
    if s.contains("nc ") { return 1; }
    if s.contains("curl") && s.contains("sh") { return 1; }
    if s.contains("wget") && s.contains("sh") { return 1; }

    0
}

pub fn evaluate(file: &str) {
    let data = fs::read_to_string(file).unwrap();
    let dataset: Vec<Command> = serde_json::from_str(&data).unwrap();

    let mut correct = 0;
    let mut total = 0;

    let mut tp = 0;
    let mut tn = 0;
    let mut fp = 0;
    let mut fn_ = 0;

    for cmd in dataset {
        let pred = predict(&cmd.text);
        let actual = cmd.label;

        if pred == actual { correct += 1; }
        total += 1;

        match (pred, actual) {
            (1,1) => tp += 1,
            (0,0) => tn += 1,
            (1,0) => fp += 1,
            (0,1) => fn_ += 1,
            _ => {}
        }
    }

    let accuracy = correct as f64 / total as f64;

    println!("File: {}", file);
    println!("Accuracy: {:.3}", accuracy);
    println!("TP: {} TN: {} FP: {} FN: {}", tp, tn, fp, fn_);
}
