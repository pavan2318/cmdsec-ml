![CI](https://github.com/pavan2318/cmdsec-ml/actions/workflows/rust.yml/badge.svg)

# cmdsec-ml

Rust prototype for detecting obfuscated command-line payloads using rule-based and ML approaches.

## Overview

This project generates synthetic command datasets (benign vs malicious), extracts features, and applies rule-based detection as a baseline.

It is designed as a lightweight research prototype for experimenting with command-line threat detection.

## Features

- Synthetic dataset generation
- Feature extraction (CSV format)
- Rule-based detection engine
- Modular Rust design

## Project Structure

src/
├── dataset.rs     # dataset generation
├── features.rs    # feature extraction
├── rules.rs       # rule-based detection
└── main.rs        # entry point

## Usage

Run training dataset generation:

```bash
cargo run -- rules-train
```

Run testing:

```bash
cargo run -- rules-test
```

## Notes
Generated files (JSON/CSV) are excluded from version control
This is the baseline version (v0.1) before ML models are introduced

## Roadmap
Add ML models (Logistic Regression, Random Forest)
Improve feature engineering
Evaluate detection performance

## Author

Pavan
