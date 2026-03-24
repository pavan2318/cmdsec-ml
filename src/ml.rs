use std::error::Error;

use csv::ReaderBuilder;
use linfa::prelude::*;
use linfa_logistic::LogisticRegression;
use ndarray::{Array1, Array2};

fn load_csv(path: &str) -> Result<(Array2<f64>, Array1<usize>), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(path)?;

    let headers = rdr.headers()?.clone();
    let n_cols = headers.len();

    if n_cols < 2 {
        return Err("CSV must contain at least one feature column and one label column".into());
    }

    let feature_cols = n_cols - 1;

    let mut x_data: Vec<f64> = Vec::new();
    let mut y_data: Vec<usize> = Vec::new();

    for result in rdr.records() {
        let record = result?;

        for i in 0..feature_cols {
            let value: f64 = record[i].parse()?;
            x_data.push(value);
        }

        let label: usize = record[feature_cols].parse()?;
        y_data.push(label);
    }

    let rows = y_data.len();
    let x = Array2::from_shape_vec((rows, feature_cols), x_data)?;
    let y = Array1::from_vec(y_data);

    Ok((x, y))
}

pub fn run_train(train_path: &str) {
    match train_logreg(train_path) {
        Ok(acc) => println!("Train Accuracy: {:.4}", acc),
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub fn run_test(train_path: &str, test_path: &str) {
    match train_and_test_logreg(train_path, test_path) {
        Ok(acc) => println!("Test Accuracy: {:.4}", acc),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn train_logreg(train_path: &str) -> Result<f32, Box<dyn Error>> {
    let (x_train, y_train) = load_csv(train_path)?;
    let train_ds = Dataset::new(x_train, y_train);

    let model = LogisticRegression::default()
        .max_iterations(200)
        .fit(&train_ds)?;

    let pred = model.predict(&train_ds);
    let cm = pred.confusion_matrix(&train_ds)?;
    Ok(cm.accuracy())
}

fn train_and_test_logreg(train_path: &str, test_path: &str) -> Result<f32, Box<dyn Error>> {
    let (x_train, y_train) = load_csv(train_path)?;
    let (x_test, y_test) = load_csv(test_path)?;

    let train_ds = Dataset::new(x_train, y_train);
    let test_ds = Dataset::new(x_test, y_test);

    let model = LogisticRegression::default()
        .max_iterations(200)
        .fit(&train_ds)?;

let pred = model.predict(&test_ds);

let mut tp = 0;
let mut tn = 0;
let mut fp = 0;
let mut fn_ = 0;

for (y_true, y_pred) in test_ds.targets().iter().zip(pred.iter()) {
    match (*y_true, *y_pred) {
        (1, 1) => tp += 1,
        (0, 0) => tn += 1,
        (0, 1) => fp += 1,
        (1, 0) => fn_ += 1,
        _ => {}
    }
}

println!("TP: {}", tp);
println!("TN: {}", tn);
println!("FP: {}", fp);
println!("FN: {}", fn_);

let accuracy = (tp + tn) as f32 / (tp + tn + fp + fn_) as f32;
println!("Accuracy: {:.4}", accuracy);

Ok(accuracy)


}
