use crate::dataset::Command;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;

#[derive(Serialize)]
pub struct Features {
    length: usize,
    special_chars: usize,
    entropy: f64,
    digit_ratio: f64,
    uppercase_ratio: f64,
    token_count: usize,
    avg_token_length: f64,
//    has_curl: u8,
//    has_wget: u8,
//    has_nc: u8,
//    has_powershell: u8,
//    has_base64: u8,
    label: u8,
}

/* ---------------- FEATURES ---------------- */

fn count_special(s: &str) -> usize {
    s.chars()
        .filter(|c| !c.is_alphanumeric() && !c.is_whitespace())
        .count()
}

fn entropy(s: &str) -> f64 {
    let mut freq = HashMap::new();

    for c in s.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }

    let len = s.len() as f64;

    freq.values()
        .map(|&count| {
            let p = count as f64 / len;
            -p * p.log2()
        })
        .sum()
}

fn contains(s: &str, keyword: &str) -> u8 {
    if s.to_lowercase().contains(keyword) { 1 } else { 0 }
}

fn extract_features(cmd: &Command) -> Features {
    let text = cmd.text.to_lowercase();
    let length = text.len();
    let special_chars = count_special(&text);
    let entropy_val = entropy(&text);
    let digits = text.chars().filter(|c| c.is_numeric()).count();
    let digit_ratio = digits as f64 / text.len() as f64;
    let uppercase = text.chars().filter(|c| c.is_uppercase()).count();
    let uppercase_ratio = uppercase as f64 / text.len() as f64;
    let token_count = text.split_whitespace().count();
    let avg_token_length = text.len() as f64 / token_count as f64;
    Features {
        length,
        special_chars,
        entropy: entropy_val,
        digit_ratio,
        uppercase_ratio,
        token_count,
        avg_token_length,
//        has_curl: contains(&text, "curl"),
//        has_wget: contains(&text, "wget"),
//        has_nc: contains(&text, "nc"),
//        has_powershell: contains(&text, "powershell"),
//        has_base64: contains(&text, "base64") | contains(&text, "-enc"),
        label: cmd.label,
    }
}


/* ---------------- PIPELINE ---------------- */

pub fn extract_from_file(input: &str, output: &str) {
    let data = fs::read_to_string(input).unwrap();
    let dataset: Vec<Command> = serde_json::from_str(&data).unwrap();

    let features: Vec<Features> = dataset
        .iter()
        .map(|cmd| extract_features(cmd))
        .collect();

    let mut wtr = csv::Writer::from_path(output).unwrap();

    for f in features {
        wtr.serialize(f).unwrap();
    }

    wtr.flush().unwrap();

    println!("Saved {}", output);
}
