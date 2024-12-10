use ndarray::{Array1, Array2, Axis};
use std::collections::{HashMap, HashSet};

pub fn process_and_prepare_dataset(
    data: &[HashMap<String, String>],
) -> (Array2<f64>, Array1<f64>) {
    let mut race_encoded = Vec::new();
    let mut sex_encoded = Vec::new();
    let mut target_encoded = Vec::new();

    for row in data {
        let race = match row.get("subject_race").map(|s| s.as_str()) {
            Some("white") => 1.0,
            Some("black") => 2.0,
            Some("hispanic") => 3.0,
            _ => 0.0, 
        };
        race_encoded.push(race);

        let sex = match row.get("subject_sex").map(|s| s.as_str()) {
            Some("male") => 1.0,
            Some("female") => 2.0,
            _ => 0.0,
        };
        sex_encoded.push(sex);

        let arrest = match row.get("arrest_made").map(|s| s.as_str()) {
            Some("true") => 1.0,
            Some("false") => 0.0,
            _ => 0.0,
        };
        target_encoded.push(arrest);
    }

    let n_samples = race_encoded.len();

    let mut features = Array2::from_shape_vec((n_samples, 2), [race_encoded, sex_encoded].concat())
        .expect("Failed to create feature array");

    let targets = Array1::from_vec(target_encoded);
    let (features, targets) = remove_duplicate_rows(features, targets);
    let features_scaled = scale_features(features);

    (features_scaled, targets)
}

fn remove_duplicate_rows(features: Array2<f64>, target: Array1<f64>) -> (Array2<f64>, Array1<f64>) {
    let mut unique_rows = HashSet::new();
    let mut unique_features = Vec::new();
    let mut unique_targets = Vec::new();

    for (i, row) in features.genrows().into_iter().enumerate() {
        let row_key: Vec<String> = row.iter().map(|&v| format!("{:.10}", v)).collect();
        if unique_rows.insert(row_key) {
            unique_features.push(row.to_vec());
            unique_targets.push(target[i]);
        }
    }

    (
        Array2::from_shape_vec(
            (unique_features.len(), features.ncols()),
            unique_features.concat(),
        )
        .expect("Failed to create unique feature array"),
        Array1::from_vec(unique_targets),
    )
}

fn scale_features(features: Array2<f64>) -> Array2<f64> {
    let mean = features.mean_axis(Axis(0)).expect("Failed to compute mean");
    let std = features.std_axis(Axis(0), 0.0);
    (&features - &mean) / &std
}
