// imports needed
// using plotters to draw the scatterplot
use crate::reading_csv::Data;
use plotters::prelude::*;
use plotters::style::{WHITE, BLACK, RED};

// the following function is used to genrates two scatter plot comparing the search rate of
// minorities (black, hispanis) to the refrence race (white)
// parameters: takes in a refrence to the data struct we read and created from reading_csv 
pub fn generate_scatter_plot(data: &Data, output_file: &str,) -> Result<(), Box<dyn std::error::Error>> {
    // initiating a drawing area for the scatterplot to work with using "BitMapBackend"
    // specifying the resoulation 1200 x 600 pixels and making it white
    let drawing_area = BitMapBackend::new(output_file, (1200, 600)).into_drawing_area();
    drawing_area.fill(&WHITE)?;

    // dividing the initiated drawing area into two, using one side for white v black 
    // and the other for white v hispanic 
    // splitting it by turning it into one row and two coloumns using the "split_evenly" method
    let areas = drawing_area.split_evenly((1, 2));
    // specifying the y values as the minorities 
    let minority_groups = vec!["black", "hispanic"];
    // sppecifying the x value for both scatterplots as white 
    let white = "white";

    // iterate over each minority and use the specified area to create its corresponding scatterplot 
    for (i, minority) in minority_groups.iter().enumerate() {
        let panel = &areas[i];

        // creating lables for eachc hart
        // organization purposes 
        let mut chart = ChartBuilder::on(panel)
            .caption(
                format!("Scatterplot: {} vs White", minority),
                ("sans-serif", 30),
            )
            .margin(5) // space of 5 pixels around the graphs
            .x_label_area_size(30) // space for the x axis
            .y_label_area_size(30) // space for the y axis 
            .build_cartesian_2d(0.0..10.0, 0.0..30.0)?; // specifying the cartesian coordinates

        // setting the labels and lines on both axis'
        chart
            .configure_mesh()
            .x_desc("White Search Rate (%)")
            .y_desc(format!("{} Search Rate (%)", minority))
            .x_labels(10)
            .y_labels(10)
            .label_style(("sans-serif", 15))
            .draw()?;

        // drawing a refrence linear refrence line going diagnolly 
        // slope of y=3x 
        chart.draw_series(LineSeries::new(
            (0..100).map(|x| {
                let rate = x as f64 * 10.0 / 100.0; 
                (rate, rate * 3.0) 
            }),
            &BLACK,
        ))?;

        // initializes variables with value of zero 
        let mut white_stops = 0.0;
        let mut white_searches = 0.0;
        let mut minority_stops = 0.0;
        let mut minority_searches = 0.0;

        // uses a for loop to iterate over the columns for race and arrrests 
        // if the current race is white
        // incrmeent white stops and if  arrrest count is true incrmenet the count
        for (race, arrest) in data
            .data_table["subject_race"]
            .iter()
            .zip(data.data_table["arrest_made"].iter())
        {
            if race.eq_ignore_ascii_case(white) {
                white_stops += 1.0;
                if arrest.eq_ignore_ascii_case("true") {
                    white_searches += 1.0;
                }
            }
            // if current race is a minority 
            // incrment the stop count and if  arrrest count is true incrmenet the count
            if race.eq_ignore_ascii_case(minority) {
                minority_stops += 1.0;
                if arrest.eq_ignore_ascii_case("true") {
                    minority_searches += 1.0;
                }
            }

            // check if the minority and white stops are greater than one and then 
            // calculate search rates, then plot the points 
            // ensures that the points are red circles  
            if white_stops > 0.0 && minority_stops > 0.0 {
                let white_search_rate = (white_searches / white_stops) * 100.0;
                let minority_search_rate = (minority_searches / minority_stops) * 100.0; 

                if white_search_rate > 0.0 || minority_search_rate > 0.0 {
                    chart.draw_series(std::iter::once(Circle::new(
                        (white_search_rate, minority_search_rate),
                        3,
                        RED.filled(),
                    )))?;

                    // the following commented out line is writte for the purpose of 
                    // seeing the coordinates of each point on the scatetr plot 
                    /* 
                    println!(
                        "Plotted Point -> White Rate: {}, Minority Rate: {}, Race: {}",
                        white_search_rate, minority_search_rate, race
                    );
                     */
                }
            }
        }
    }
// save the output to the file, the path will be printed out in the main function 
    drawing_area.present()?;

    Ok(())
}
