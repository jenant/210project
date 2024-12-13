mod reading_csv;
mod basic_analysis;
mod scatterplot;
mod linear_reg;

use std::error::Error;
use reading_csv::Data;
use basic_analysis::*;
use scatterplot::*;
use linear_reg::{one_hot_encode, matrix};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "/Users/jenantaibah/Desktop/BU/Fall24/DS210/project/ri_statewide_2020_04_01.csv";
    let scatter_plot_path = "/Users/jenantaibah/Desktop/BU/Fall24/DS210/project/scatterplot.png";
    let one_hot_output_path = "/Users/jenantaibah/Desktop/BU/Fall24/DS210/project/one_hot_encoded.csv";
    let reference_group = "White"; 
    let target_columns = ["subject_race", "subject_sex"]; 
    let target_column = "subject_race"; 

    match Data::from_csv(file_path) {
        Ok(mut data) => {
            println!("--- Original Dataset ---");
            data.print_readable();

            println!();
            println!("--- Analysis on Original Dataset ---");
            let race_count = data.arrests_and_searches_by_race();
            let race_ratio = data.race_ratio(&race_count);

            println!();
            data.print_arrests_and_searches_by_race(&race_count);
            println!();
            data.print_race_ratios(&race_ratio);

            let (male_count, female_count, male_to_female_ratio) = data.gender_ratio();
            println!();
            println!("Males: {}", male_count);
            println!("Females: {}", female_count);
            println!("Male to Female Ratio: {:.2}", male_to_female_ratio);

            println!();
            println!("--- Generating Scatter Plot ---");
            match scatterplot::generate_scatter_plot(&data, scatter_plot_path) {
                Ok(_) => println!("Scatter plot successfully saved to {}", scatter_plot_path),
                Err(e) => eprintln!("Error generating scatter plot: {}", e),
            }

            println!();
            println!("--- Performing One-Hot Encoding ---");
            match one_hot_encode(file_path, one_hot_output_path, reference_group, &target_columns) {
                Ok(_) => {
                    println!("Encoded file saved to: {}", one_hot_output_path);

                    println!();
                    println!("--- Creating Matrices for Linear Regression ---");
                    match matrix(one_hot_output_path, reference_group, target_column) {
                        Ok((x_matrix, y_vector)) => {
                            println!("Feature Matrix (X) - First 10 Rows:");
                            for row in x_matrix.rows().into_iter().take(10) {
                                println!("{:?}", row.to_vec());
                            }

                            println!("\nOutcome Vector (y) - First 10 Elements:");
                            for value in y_vector.iter().take(10) {
                                println!("{}", value);
                            }
                        }
                        Err(e) => eprintln!("Error creating matrices: {}", e),
                    }
                }
                Err(e) => eprintln!("Error during One-Hot Encoding: {}", e),
            }
        }
        Err(e) => {
            eprintln!("Error, can't read file: {}", e);
        }
    }

    Ok(())
}
