![CI](https://github.com/pavan2318/cmdsec-ml/actions/workflows/rust.yml/badge.svg)

# cmdsec-ml

Rust prototype for detecting obfuscated command-line payloads using rule-based and ML approaches.

## Overview

This project explores detection of obfuscated command-line attacks using:
- Rule-based detection (baseline)
- Machine learning (logistic regression)
- Synthetic dataset with controlled obfuscation techniques


It is designed as a lightweight research prototype for experimenting with command-line threat detection.

## Pipeline

1. Dataset generation (benign + malicious commands)
2. Obfuscation (train vs test separation)
3. Feature extraction (statistical features)
4. Rule-based detection
5. ML classification

## Features

- Synthetic dataset generation
- Feature extraction (CSV format)
- Rule-based detection engine
- Modular Rust design
- length
- special_chars
- entropy
- digit_ratio
- uppercase_ratio
- token_count
- avg_token_length

## Project Structure

src/
├── dataset.rs     # dataset generation
├── features.rs    # feature extraction
├── rules.rs       # rule-based detection
└── main.rs        # entry point

## Usage

Generate dataset:

```bash
cargo run -- generate
```

Extract features:

```bash
cargo run -- features-train
cargo run -- features-test
```

Run rules:

```bash
cargo run -- rules-test
```

Run ML:

```bash
cargo run -- ml-test
```

## Results

### Rule-based (baseline)

- Accuracy: 0.644
- FP: 10
- FN: 79

→ conservative, but misses many attacks


### ML (Logistic Regression)

- Accuracy: 0.708
- FP: 34
- FN: 39

→ significantly reduces missed attacks (~50% FN reduction)  
→ at the cost of higher false positives

## Future Work

- Random Forest / tree-based models
- Better feature engineering
- More realistic obfuscation techniques
- Generalisation testing on unseen transformations
