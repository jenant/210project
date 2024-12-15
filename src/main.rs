mod reading_csv;
mod basic_analysis;
mod scatterplot;
mod chaisquared;
mod piechart_gender; 

use reading_csv::Data;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // specified path to the data set
    let file_path = "ri_statewide_2020_04_01.csv";
    // path to the scatterplot image created
    let scatter_plot_path = "scatterplot.png";
    // specifying the desired column for the chaisquaredtest
    let target_column_arrest = "arrest_made"; 

    match Data::create_readable(file_path) {
        Ok(data) => {
            println!("--- Printing Original Dataset Sample ---");
            data.print_readable();

            println!();
            println!("--- Basic Analysis on Original Dataset ---");
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
            println!("--- Generating Scatter Plot (Race) ---");
            match scatterplot::generate_scatter_plot(&data, scatter_plot_path) {
                Ok(_) => println!("Scatter plot can be found in: {}", scatter_plot_path),
                Err(e) => eprintln!("Error creating scatterplot: {}", e),
            }

            println!();
            println!("--- Generating Pie Chart (Gender) ---");
            match piechart_gender::generate_pie_chart(male_count, female_count) {
                Ok(_) => println!("Pie chart successfully generated."),
                Err(e) => eprintln!("Error creating pie-chart: {}", e),
            }

            println!();
            println!("--- Performing Chi-Squared Test ---");
            match chaisquared::chi_squared_test(&data, "subject_race", target_column_arrest) {
                Ok(_) => println!("Chi-squared test completed successfully."),
                Err(e) => eprintln!("Error performing chi-squared test: {}", e),
            }
            
        }
        Err(e) => {
            eprintln!("Error, can't read file: {}", e);
        }
    }

    Ok(())
}
