// using neccessary imports for the functions to work
use std::collections::HashMap;
use statrs::distribution::{ChiSquared, ContinuousCDF};
use crate::reading_csv::Data;

// the following funcion conductes a chi squared test on the data
// takes in the table we made in reading_csv which was based on the read cv file 
// takes in two categorical data columns (race and arrests made)
pub fn chi_squared_test(data: &Data, column_a: &str, column_b: &str) -> Result<(), Box<dyn std::error::Error>> {
    // initialize variables for two columns from the data set
    let column_a_values = data.data_table.get(column_a).ok_or("Column A not found")?;
    let column_b_values = data.data_table.get(column_b).ok_or("Column B not found")?;
    
     // creating a contingency table to count the value the choosen columns (a,b)
     // also initializes rows and columns with the starting total of 0
    let mut contingency_table: HashMap<(String, String), usize> = HashMap::new();
    let mut row_totals: HashMap<String, usize> = HashMap::new();
    let mut col_totals: HashMap<String, usize> = HashMap::new();
    let mut total_count = 0;

    // uses zip and a for loop to iterate of the columns and 
    // increments the row total for each unique value in a
    // also increments the column total for each unique value in b
    // then incrments the total count 
    for (a_value, b_value) in column_a_values.iter().zip(column_b_values.iter()) {
        *contingency_table.entry((a_value.clone(), b_value.clone())).or_insert(0) += 1;
        *row_totals.entry(a_value.clone()).or_insert(0) += 1;
        *col_totals.entry(b_value.clone()).or_insert(0) += 1;
        total_count += 1;
    }

    // creating a value initialized to zero that will keep the 
    // resulting statistic of the test
    let mut chi_squared_stat = 0.0;
    // uses a forloop to iterate over the the contingency table 
    // get the expected value of the pair and increment the statistic variable 
    for ((a_value, b_value), observed) in contingency_table.iter() {
        let expected = (*row_totals.get(a_value).unwrap() as f64) 
                     * (*col_totals.get(b_value).unwrap() as f64) 
                     / total_count as f64;
        chi_squared_stat += ((*observed as f64 - expected).powi(2)) / expected;
    }

    // creating a variable which stores the degrees of freedom
    // using the degrees of freedom to get the ditrubution 
    // using the ditrubution to get the p value
    let degrees_of_freedom = (row_totals.len() - 1) * (col_totals.len() - 1);
    let chi_squared_distrubution = ChiSquared::new(degrees_of_freedom as f64)?;
    let p_value = 1.0 - chi_squared_distrubution.cdf(chi_squared_stat);

    // printing the results of the tests
    println!("Chi-Squared Statistic: {:.2}", chi_squared_stat);
    println!("Degrees of Freedom: {}", degrees_of_freedom);
    println!("P-Value: {}", p_value);

    // if else statment to know if the relationship is significant or not
    if p_value < 0.05 {
        println!("The relationship is significant!!");
    } else {
        println!("The relationship is not significant");
    }

    Ok(())
}
