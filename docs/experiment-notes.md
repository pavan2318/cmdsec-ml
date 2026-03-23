# Experiment Log

## v0.1 - Rule-based baseline

### Setup
- Synthetic dataset (train/test split)
- Rule-based detection (rules.rs)

### Results
- Train Accuracy: 0.885
- Test Accuracy: 0.728
- TP: 82
- TN: 100
- FP: 0
- FN: high

### Observations
- Very low false positives
- High false negatives on obfuscated inputs
- Rules are too rigid, do not generalize

### Next Steps
- Introduce feature-based ML model
- Compare against rule baseline
