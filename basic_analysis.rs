// importing the neccesary imports for function to work 
use std::collections::HashMap;
use crate::reading_csv::Data;

// the functions will be implemented on the Data struct we created and read in reading_csv 
impl Data {
    // the following function counts the arrests and searches made by race 
    // the parameters take inthe keys which contain the race names and a tuple of (arrests, searches)
    // representing the counts 
    pub fn arrests_and_searches_by_race(&self) -> HashMap<String, (usize, usize)> {
        // initialize a new hashmap for each race to store the counts 
        let mut race_count: HashMap<String, (usize, usize)> = HashMap::new(); 

        // making variables for every relevant column
        let arrest_made_index = self.data_table.get("arrest_made").unwrap();
        let search_conducted_index = self.data_table.get("search_conducted").unwrap();
        let race_index = self.data_table.get("subject_race").unwrap();

        // uses a for loop to iterate over the rows 
        // then normalize and get race and see whether or not the 
        // search and arrest are true or false 
        for i in 0..arrest_made_index.len() {
            let race = race_index[i].trim().to_lowercase();
            let arrest_made = arrest_made_index[i].trim().to_lowercase();
            let search_conducted = search_conducted_index[i].trim().to_lowercase();

            // incrments the counts if arrests and searches are true
            race_count.entry(race.clone()).or_insert((0, 0));
            if arrest_made == "true" {
                let (arrests, searches) = race_count.entry(race.clone()).or_insert((0, 0));
                *arrests += 1;
            }
            if search_conducted == "true" {
                let (arrests, searches) = race_count.entry(race.clone()).or_insert((0, 0));
                *searches += 1;
            }
        }
        // return the hashmap with counts 
        race_count
    }

    // the following function pritns the function above 
    // parameters takes in an instance of the impleemnted data and hashmap above
    // prints headers and a line of "-" that seperates the column labels from the data 
    // for organization and readability purposes also makes sure theyre the same difference apart 
    pub fn print_arrests_and_searches_by_race(&self, race_count: &HashMap<String, (usize, usize)>) {
        println!("{:<20} {:<15} {:<15}", "Race", "Arrests", "Searches");
        println!("{}", "-".repeat(50));

        for (race, &(arrests, searches)) in race_count {
            println!("{:<25} | {:<10} | {:<10}", race, arrests, searches);
        }
    }

    // the following function takes in an instance of the impleemnted data 
    // and the hashmap created for race_count, returns a hashnmap with the race and 
    // the ratios comparing the minority to the refrence race based on the counts
    pub fn race_ratio(&self, race_count: &HashMap<String, (usize, usize)>) -> HashMap<String, (f64, f64)> {
        // initalizes a mutable hashmap that will stopre the race ratios
        let mut race_ratio: HashMap<String, (f64, f64)> = HashMap::new();

        if let Some(white_count) = race_count.get("white") {
            let (white_arrests, white_searches) = *white_count;

            // uses a for loop to iterate over every race and get the ratio of arrests and searches
            // comapred the refrence race's arrests and searches 
            // ensures that there is no division by zero 
            // after getting ratios put them in the specific slot within the hashmap 
            for (race, &(arrests, searches)) in race_count {
                let arrest_ratio = if white_arrests > 0 {
                    arrests as f64 / white_arrests as f64
                } else {
                    0.0
                };

                let search_ratio = if white_searches > 0 {
                    searches as f64 / white_searches as f64
                } else {
                    0.0
                };

                race_ratio.insert(race.clone(), (arrest_ratio, search_ratio));
            }
        }
        // returns the hash map with the race ratios
        race_ratio
    }

    // the following function takes in an insatnce of the implemented data 
    // and the race_ratio function above and prints it 
    // organizes the table making it more readable by making the spaces the same 
    // also prints a seperater line
    pub fn print_race_ratios(&self, race_ratio: &HashMap<String, (f64, f64)>) {
        println!("{:<20} {:<15} {:<15}", "Race", "Arrest Ratio", "Search Ratio");
        println!("{}", "-".repeat(50));

        for (race, &(arrest_ratio, search_ratio)) in race_ratio {
            println!("{:<25} | {:<20.2} | {:<20.2}", race, arrest_ratio, search_ratio);
        }
    }

    // the following function calculuates the gender ratio
    // comapring male searches and arrests to females 
    // takes in an insatnce of the implemented data  and returns numbers of the ratio
    pub fn gender_ratio(&self) -> (usize, usize, f64) {
        let sex_index = self.data_table.get("subject_sex").unwrap();

        // initializes the two variables to zero 
        // uses a counter to first check if the race is male female
        // based on that incrmeent the counter
        let mut male_count = 0;
        let mut female_count = 0;

        for sex in sex_index {
            match sex.trim().to_lowercase().as_str() {
                "male" => male_count += 1,
                "female" => female_count += 1,
                _ => {}
            }
        }
        // after gettting the count of each gender find the ratio of males to females
        let male_to_female_ratio = if female_count > 0 {
            male_count as f64 / female_count as f64
        } else {
            0.0
        };

        // returnt the number of males 
        // return number of females
        // return the ratio of males to females 
        (male_count, female_count, male_to_female_ratio)
    }

}
