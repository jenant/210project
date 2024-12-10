use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use ndarray::Array2;

#[derive(Clone)]
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

    pub fn preprocess(&mut self) {
        // Encode "subject_race" (Label Encoding)
        if let Some(race_column) = self.data_map.get_mut("subject_race") {
            for value in race_column.iter_mut() {
                *value = match value.as_str() {
                    "Black" => "1".to_string(),
                    "White" => "0".to_string(),
                    _ => "-1".to_string(), // Encode unknown values as -1
                };
            }
        }
    
        // Encode "subject_sex" (Label Encoding)
        if let Some(sex_column) = self.data_map.get_mut("subject_sex") {
            for value in sex_column.iter_mut() {
                *value = match value.as_str() {
                    "Male" => "1".to_string(),
                    "Female" => "0".to_string(),
                    _ => "-1".to_string(), // Encode unknown values as -1
                };
            }
        }
    
        // Convert "arrest_made" to binary
        if let Some(arrest_column) = self.data_map.get_mut("arrest_made") {
            for value in arrest_column.iter_mut() {
                *value = match value.as_str() {
                    "Yes" => "1".to_string(),
                    "No" => "0".to_string(),
                    _ => "0".to_string(), // Default missing or invalid values to 0
                };
            }
        }
    
        // Handle missing or null values by imputing with default
        for (key, column) in self.data_map.iter_mut() {
            for value in column.iter_mut() {
                if value.trim().is_empty() || value == "N/A" {
                    *value = match key.as_str() {
                        "subject_race" => "-1".to_string(),
                        "subject_sex" => "-1".to_string(),
                        "arrest_made" => "0".to_string(),
                        "search_conducted" => "0".to_string(),
                        _ => "0".to_string(), // Default for other columns
                    };
                }
            }
        }
    }

    pub fn prepare_features_and_target(&self) -> (Array2<f64>, Array2<f64>) {
        let races = self
            .data_map
            .get("subject_race")
            .unwrap()
            .iter()
            .map(|s| s.parse::<f64>().unwrap_or(-1.0)) // Handle missing or invalid values
            .collect::<Vec<f64>>();

        let sexes = self
            .data_map
            .get("subject_sex")
            .unwrap()
            .iter()
            .map(|s| s.parse::<f64>().unwrap_or(-1.0)) // Handle missing or invalid values
            .collect::<Vec<f64>>();

        let arrests = self
            .data_map
            .get("arrest_made")
            .unwrap()
            .iter()
            .map(|s| s.parse::<f64>().unwrap_or(0.0)) // Handle missing or invalid values
            .collect::<Vec<f64>>();

        let n_samples = races.len();
        // Combine `races` and `sexes` into a 2D feature array
        let features = Array2::from_shape_vec((n_samples, 2), [races, sexes].concat()).unwrap();
        // Create a 2D target array
        let targets = Array2::from_shape_vec((n_samples, 1), arrests).unwrap();

        (features, targets)
    }
}
