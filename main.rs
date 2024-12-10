mod reading_csv;
mod basic_analysis;
mod linear_reg;
mod data_processing;

use std::collections::HashMap;
use std::error::Error;
use reading_csv::Data;
use basic_analysis::*;
use linear_reg::*;
use linfa::prelude::*;
use linfa_linear::LinearRegression;
use ndarray::{Array2, ArrayView2};
use data_processing::process_and_prepare_dataset;


use linear_reg::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "/Users/jenantaibah/Desktop/BU/Fall24/DS210/project/ri_statewide_2020_04_01.csv";

    match Data::from_csv(file_path) {
        Ok(mut data) => {
            println!("--- Original Dataset ---");
            data.print_readable(); // Display the original dataset

            // Clone the dataset to preprocess without modifying the original
            let mut processed_data = data.clone();
            processed_data.preprocess();

            println!();
            println!("--- Analysis on Original Dataset ---");

            // Perform analysis using the original dataset
            let race_count = data.arrests_and_searches_by_race();
            let race_ratio = data.race_ratio(&race_count);

            println!();
            data.print_arrests_and_searches_by_race(&race_count);
            println!();
            data.print_race_ratios(&race_ratio);

            // Gender ratio analysis
            let (male_count, female_count, male_to_female_ratio) = data.gender_ratio();
            println!();
            println!("Males: {}", male_count);
            println!("Females: {}", female_count);
            println!("Male to Female Ratio: {:.2}", male_to_female_ratio);

            println!();
            println!("--- Linear Regression (on Preprocessed Data) ---");

// Use process_and_prepare_dataset to encode features and target
let data_vec: Vec<HashMap<String, String>> = processed_data
    .data_map
    .iter()
    .map(|(k, v)| {
        v.iter()
            .map(|val| (k.clone(), val.clone()))
            .collect::<HashMap<String, String>>()
    })
    .collect();

    let (features, target) = process_and_prepare_dataset(&data_vec);

    let model = fit_model(features.view(), target.view());

// Print regression results
let params = model.params();
println!("Slope (Weight): {}", params[0]);
println!("Intercept (Bias): {}", model.intercept());

// Plot the regression results
plot_data(features.view(), target.insert_axis(ndarray::Axis(1)).view(), &model);

println!("Linear regression plot saved as 'linear_regression.png'.");


        }
        Err(e) => {
            eprintln!("Error, can't read file: {}", e);
        }
    }

    Ok(())
}
