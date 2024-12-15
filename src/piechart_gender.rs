// uses neccesary imports
use piechart::{Chart, Color, Data};

// the following function creates a pie chart based on the gender ratios
// takes in the gender counts as input and gives back 
pub fn generate_pie_chart(male_count: usize, female_count: usize,) -> Result<(), Box<dyn std::error::Error>> {
    // gets the total to use as the base of the ratio
    let total = male_count + female_count;
    if total == 0 {
        return Err("No data available for males or females.".into());
    }

    // calculate the male and femal percentages based on the total 
    // makes sure theyre floats of 64 
    let male_percentage = male_count as f64 / total as f64 * 100.0;
    let female_percentage = female_count as f64 / total as f64 * 100.0;

    let data = vec![
        // part of the piechart specified for males based on the percentage 
        // prints out the corresponding percentage and then makes a blue 
        // square in the specified section of the chart
        Data {
            label: format!("Male: {:.2}%", male_percentage),
            value: male_count as f32,
            color: Some(Color::Blue.into()),
            fill: '▪',
        },
        // part of the piechart specified for females based on the percentage 
        // prints out the corresponding percentage and then makes a red 
        // circle in the specified section of the chart
        Data {
            label: format!("Female: {:.2}%", female_percentage),
            value: female_count as f32,
            color: Some(Color::Red.into()),
            fill: '•',
        },
    ];

    // uses the imported crate to draw the pie chart
    Chart::new()
        .radius(9)
        .aspect_ratio(3)
        .legend(true)
        .draw(&data);

    Ok(())
}
