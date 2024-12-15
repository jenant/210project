// set of inputs needed to run the code
use std::collections::HashMap; 
use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;

// used fo cloning to make copies of the struct data_map
#[derive(Clone)]
// creating a struct that takes in string as keys and vectors of string as values
// keys = column names or the labels
// values = vectors of data
pub struct Data {
    pub data_table: HashMap<String, Vec<String>>,
}

// implements all the following functions on the struct created above 
impl Data {
    // the following function creates a table from the csv file provided 
    // file_path is specified in the main function, allows the code to find the file it should read 
    // the function returns an instance of the struct data
    pub fn create_readable(file_path: &str) -> Result<Self, Box<dyn Error>> {
        // uses ::open, to open the specified file and read it 
        let file = File::open(file_path)?;
        
        let mut rdr = ReaderBuilder::new()
            // the data set does have headers so its set to true 
            .has_headers(true)
            // use the opened file 
            .from_reader(file);

        // initialize and empty data set to store whats read from the csv
        let mut data_table: HashMap<String, Vec<String>> = HashMap::new();
        // sspecify the needed keys
        // not all columns in the data set will be used for this analysis so i chose the neccessary ones 
        // i also chose ones that would work on different data sets
        // the one im using is the data set for Rhode Island
        let keys = vec!["subject_race", "subject_age", "subject_sex", "arrest_made", "search_conducted"];

        // gets the headers from the csv file and puts them into a vector of strings
        // iterates over each header, turns them into strings and then pushes them into a vector
        let headers: Vec<String> = rdr.headers()?
            .iter()
            .map(|h| h.to_string())
            .collect();

        // initializes hashmap with empty keys for each key in the table 
        for key in &keys {
            if !headers.contains(&key.to_string()) {
            }
            data_table.insert(key.to_string(), Vec::new());
        }

        // uses a for loop to iterate over each row in the csv file and pushes it into the initiated table
        for result in rdr.records() {
            let record = result?;
            for key in &keys {
                if let Some(index) = headers.iter().position(|h| h == *key) {
                    // if the key exists in the csv file push it into the hashmap 
                    if let Some(value) = record.get(index) {
                        data_table.get_mut(*key).unwrap().push(value.to_string());
                    }
                // if the key does not exist, push the string "N/A" into the key 
                } else {
                    data_table.get_mut(*key).unwrap().push("N/A".to_string());
                }
            }
        }
        // returns an instance of the struct Data that is the table we created
        // will be used in all of our other funtions 
        Ok(Data {data_table})
    }

    // the following function prints out the table we created above 
    pub fn print_readable(&self) {
        // collects all the keys to use as the labels for the columns  
        let keys: Vec<&String> = self.data_table.keys().collect();
        let num_rows = self.data_table.values().next().map_or(0, |v| v.len());

        // prints out the labels/ column names 
        // for formatting and organization, makes the spaces between the headers/ labels the same 
        // seperates them using "|"
        // also print a row of "-----" to seperate labels from values
        for key in &keys {
            print!("{:^15}|", key);
        }
        println!();
        println!("{}", "-".repeat(16 * keys.len()));

        // uses a for loop to iterate over the number of rows
        // only prints out the first 20
        // just to give us a more readable version of the csv file 
        // shows the columns that are used in the rest of the analysis 
        // could be adjusted to whatever length is desired 
        for i in 0..num_rows.min(20) {
            for key in &keys {
                if let Some(value) = self.data_table.get(*key).and_then(|v| v.get(i)) {
                    print!("{:^15}|", value);
                } else {
                    print!("{:^15}|", "N/A");
                }
            }
            println!();
        }
    }
}
