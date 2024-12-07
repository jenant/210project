// src/main.rs
mod reading_csv;
mod basic_analysis;

use std::error::Error;
use reading_csv::Data;
use basic_analysis::*; 

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "/Users/jenantaibah/Desktop/BU/Fall24/DS210/project/ri_statewide_2020_04_01.csv";

    match Data::from_csv(file_path) {
        Ok(data) => {
            data.print_readable();

            let race_count = data.arrests_and_searches_by_race();
            let race_ratio = data.race_ratio(&race_count);

            println!();
            data.print_arrests_and_searches_by_race(&race_count);

            println!();
            data.print_race_ratios(&race_ratio); 


            let (male_count, female_count, male_to_female_ratio) = data.gender_ratio();

            println!();
            println!("Number of Males: {}", male_count);
            println!("Number of Females: {}", female_count);
            println!("Male to Female Ratio: {:.2}", male_to_female_ratio);
        }
        Err(e) => {
            eprintln!("Error reading CSV file: {}", e);
        }
    }

    Ok(())
}
