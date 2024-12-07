// src/data.rs
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

pub struct Data {
    pub data_map: HashMap<String, Vec<String>>,
}

impl Data {
    pub fn from_csv(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        let mut data_map: HashMap<String, Vec<String>> = HashMap::new();
        let keys = vec!["subject_race", "subject_age", "subject_sex", "arrest_made", "search_conducted"];

        let headers: Vec<String> = rdr.headers()?
            .iter()
            .map(|h| h.to_string())
            .collect();

        for key in &keys {
            if !headers.contains(&key.to_string()) {
            }
            data_map.insert(key.to_string(), Vec::new());
        }

        for result in rdr.records() {
            let record = result?;
            for key in &keys {
                if let Some(index) = headers.iter().position(|h| h == *key) {
                    if let Some(value) = record.get(index) {
                        data_map.get_mut(*key).unwrap().push(value.to_string());
                    }
                } else {
                    data_map.get_mut(*key).unwrap().push("N/A".to_string());
                }
            }
        }

        Ok(Data { data_map })
    }

    pub fn print_readable(&self) {
        let keys: Vec<&String> = self.data_map.keys().collect();
        let num_rows = self.data_map.values().next().map_or(0, |v| v.len());

        for key in &keys {
            print!("{:^15}|", key);
        }
        println!();
        println!("{}", "-".repeat(16 * keys.len()));

        for i in 0..num_rows.min(20) {
            for key in &keys {
                if let Some(value) = self.data_map.get(*key).and_then(|v| v.get(i)) {
                    print!("{:^15}|", value);
                } else {
                    print!("{:^15}|", "N/A");
                }
            }
            println!();
        }
    }
}
