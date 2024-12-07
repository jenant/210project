use std::collections::HashMap;
use crate::reading_csv::Data;

impl Data {
    pub fn arrests_and_searches_by_race(&self) -> HashMap<String, (usize, usize)> {
        let mut race_count: HashMap<String, (usize, usize)> = HashMap::new();

        let arrest_made_index = self.data_map.get("arrest_made").unwrap();
        let search_conducted_index = self.data_map.get("search_conducted").unwrap();
        let race_index = self.data_map.get("subject_race").unwrap();

        for i in 0..arrest_made_index.len() {
            let race = race_index[i].trim().to_lowercase();
            let arrest_made = arrest_made_index[i].trim().to_lowercase();
            let search_conducted = search_conducted_index[i].trim().to_lowercase();

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

        race_count
    }

    pub fn print_arrests_and_searches_by_race(&self, race_count: &HashMap<String, (usize, usize)>) {
        println!("{:<20} {:<15} {:<15}", "Race", "Arrests", "Searches");
        println!("{}", "-".repeat(50));

        for (race, &(arrests, searches)) in race_count {
            println!("{:<25} | {:<10} | {:<10}", race, arrests, searches);
        }
    }

    pub fn race_ratio(&self, race_count: &HashMap<String, (usize, usize)>) -> HashMap<String, (f64, f64)> {
        let mut race_ratio: HashMap<String, (f64, f64)> = HashMap::new();

        if let Some(white_count) = race_count.get("white") {
            let (white_arrests, white_searches) = *white_count;

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

        race_ratio
    }

    pub fn print_race_ratios(&self, race_ratio: &HashMap<String, (f64, f64)>) {
        println!("{:<20} {:<15} {:<15}", "Race", "Arrest Ratio", "Search Ratio");
        println!("{}", "-".repeat(50));

        for (race, &(arrest_ratio, search_ratio)) in race_ratio {
            println!("{:<25} | {:<20.2} | {:<20.2}", race, arrest_ratio, search_ratio);
        }
    }
    pub fn gender_ratio(&self) -> (usize, usize, f64) {
        let sex_index = self.data_map.get("subject_sex").unwrap();

        let mut male_count = 0;
        let mut female_count = 0;

        for sex in sex_index {
            match sex.trim().to_lowercase().as_str() {
                "male" => male_count += 1,
                "female" => female_count += 1,
                _ => {}
            }
        }
        let male_to_female_ratio = if female_count > 0 {
            male_count as f64 / female_count as f64
        } else {
            0.0
        };

        (male_count, female_count, male_to_female_ratio)
    }

}
