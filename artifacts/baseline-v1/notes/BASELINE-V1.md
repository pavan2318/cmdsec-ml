Baseline-V1

Date frozen: 2026-03-24
Branch: feature/ml-baseline
Purpose: frozen baseline before adding new features/models/evaluation changes

Current pipeline:
- dataset generation
- feature extraction
- rule-based baseline
- logistic regression baseline

Current features:
- length
- special_chars
- entropy
- digit_ratio
- uppercase_ratio
- token_count
- avg_token_length

Current results snapshot:
Rules:
- Accuracy: ~0.644
- TP: 71
- TN: 90
- FP: 10
- FN: 79

Logistic Regression:
- Accuracy: ~0.708
- TP: 111
- TN: 66
- FP: 34
- FN: 39

Notes:
- No keyword features
- Synthetic dataset only
- Current train/test split frozen in artifacts/baseline-v1/datasets
- Current feature CSVs frozen in artifacts/baseline-v1/features
