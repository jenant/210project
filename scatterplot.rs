use crate::reading_csv::Data;
use plotters::prelude::*;
use plotters::style::{WHITE, BLACK, RED};

pub fn generate_scatter_plot(
    data: &Data,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root_area = BitMapBackend::new(output_file, (1200, 600)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let areas = root_area.split_evenly((1, 2));
    let minority_groups = vec!["black", "hispanic"];
    let white = "white";

    for (i, minority) in minority_groups.iter().enumerate() {
        let panel = &areas[i];

        let mut chart = ChartBuilder::on(panel)
            .caption(
                format!("Scatterplot: {} vs White", minority),
                ("sans-serif", 30),
            )
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0.0..10.0, 0.0..30.0)?; 

        chart
            .configure_mesh()
            .x_desc("White Search Rate (%)")
            .y_desc(format!("{} Search Rate (%)", minority))
            .x_labels(10)
            .y_labels(10)
            .label_style(("sans-serif", 15))
            .draw()?;

        chart.draw_series(LineSeries::new(
            (0..100).map(|x| {
                let rate = x as f64 * 10.0 / 100.0; 
                (rate, rate * 3.0) 
            }),
            &BLACK,
        ))?;

        let mut white_stops = 0.0;
        let mut white_searches = 0.0;
        let mut minority_stops = 0.0;
        let mut minority_searches = 0.0;

        for (race, arrest) in data
            .data_map["subject_race"]
            .iter()
            .zip(data.data_map["arrest_made"].iter())
        {
            if race.eq_ignore_ascii_case(white) {
                white_stops += 1.0;
                if arrest.eq_ignore_ascii_case("true") {
                    white_searches += 1.0;
                }
            }

            if race.eq_ignore_ascii_case(minority) {
                minority_stops += 1.0;
                if arrest.eq_ignore_ascii_case("true") {
                    minority_searches += 1.0;
                }
            }

            if white_stops > 0.0 && minority_stops > 0.0 {
                let white_search_rate = (white_searches / white_stops) * 100.0;
                let minority_search_rate = (minority_searches / minority_stops) * 100.0; 

                if white_search_rate > 0.0 || minority_search_rate > 0.0 {
                    chart.draw_series(std::iter::once(Circle::new(
                        (white_search_rate, minority_search_rate),
                        3,
                        RED.filled(),
                    )))?;
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

    root_area.present()?;
    println!("Scatter plot saved to {}", output_file);

    Ok(())
}
