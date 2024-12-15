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


// the following lines of code are tests to check if the 
// code is running as desired
#[cfg(test)]
mod tests {
    use crate::reading_csv::Data;
    use std::collections::HashMap;
    
    // gets sample data from my data set
    // based on rows 324-352 from my data set
    fn get_sample_data() -> Data {
        let mut data_table = HashMap::new();
        data_table.insert("subject_race".to_string(), vec![
            "white", "white", "black", "hispanic", "black",
            "white", "white", "black", "white", "black",
        ].into_iter().map(|s| s.to_string()).collect());
    
        data_table.insert("subject_sex".to_string(), vec![
            "male", "female", "male", "male", "female",
            "male", "female", "male", "male", "male",
        ].into_iter().map(|s| s.to_string()).collect());
    
        data_table.insert("arrest_made".to_string(), vec![
            "true", "false", "true", "false", "false",
            "false", "true", "false", "true", "false",
        ].into_iter().map(|s| s.to_string()).collect());
    
        data_table.insert("search_conducted".to_string(), vec![
            "true", "false", "false", "true", "true",
            "false", "true", "true", "false", "true",
        ].into_iter().map(|s| s.to_string()).collect());
    
        Data { data_table }
    }    

    // the following tests ensures that the arrests and searches 
    // incrment the correct races when counting
    #[test]
    fn test_arrests_and_searches_by_race() {
        let data = get_sample_data();
        let result = data.arrests_and_searches_by_race();

        let expected = {
            let mut map = HashMap::new();
            map.insert("white".to_string(), (3, 2)); 
            map.insert("black".to_string(), (1, 3));
            map.insert("hispanic".to_string(), (0, 1));
            map
        };
        assert_eq!(result, expected);
    }

    // the following test ensures that the ratio of races are correct 
    // calculates the minorities to the refrence 
    #[test]
    fn test_race_ratio() {
        let data = get_sample_data();
        let race_counts = data.arrests_and_searches_by_race();
        let result = data.race_ratio(&race_counts);

        let expected = {
            let mut map = HashMap::new();
            map.insert("white".to_string(), (1.0, 1.0)); 
            map.insert("black".to_string(), (0.333333, 1.5)); 
            map.insert("hispanic".to_string(), (0.0, 0.5));
            map
        };

        for (key, &(expected_arrest, expected_search)) in &expected {
            let (actual_arrest, actual_search) = result.get(key).unwrap();
            assert!((actual_arrest - expected_arrest).abs() < 1e-3, "Arrest ratio mismatch for race: {}", key);
            assert!((actual_search - expected_search).abs() < 1e-3, "Search ratio mismatch for race: {}", key);
        }
    }

    // the following test ensures that the ratio of males 
    // to females is correct 
    #[test]
    fn test_gender_ratio() {
        let data = get_sample_data();
        let (male_count, female_count, ratio) = data.gender_ratio();

        assert_eq!(male_count, 7);
        assert_eq!(female_count, 3);
        assert!((ratio - 2.33).abs() < 0.01); 
    }

}
