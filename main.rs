mod reading_csv;
mod basic_analysis;
mod scatterplot;

use std::error::Error;
use reading_csv::Data;
use basic_analysis::*;
use scatterplot::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "/Users/jenantaibah/Desktop/BU/Fall24/DS210/project/ri_statewide_2020_04_01.csv";
    let scatter_plot_path = "/Users/jenantaibah/Desktop/BU/Fall24/DS210/project/comparative_scatterplot.png";

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
                Err(e) => eprintln!("Error generating comparative scatter plot: {}", e),
            }
        }
        Err(e) => {
            eprintln!("Error, can't read file: {}", e);
        }
    }

    Ok(())
}
