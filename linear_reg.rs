extern crate plotters;
use plotters::prelude::*;
use linfa::prelude::*;
use linfa::Dataset;
use linfa_linear::{FittedLinearRegression, LinearRegression};
use ndarray::{array, Array2, ArrayView1, ArrayView2};

pub fn fit_model(features: ArrayView2<f64>, target: ArrayView1<f64>) -> FittedLinearRegression<f64> {
    let dataset = Dataset::new(features.to_owned(), target.to_owned()); // Create dataset

    let lin_reg = LinearRegression::new();
    let model = lin_reg.fit(&dataset).unwrap(); // Fit the model

    let predictions = model.predict(&dataset);
    let loss = (dataset.targets() - predictions)
        .mapv(|x| x.abs())
        .mean();

    println!("Mean Absolute Error: {:?}", loss);
    model
}


pub fn plot_data(
    features: ArrayView2<f64>,
    target: ArrayView2<f64>,
    model: &FittedLinearRegression<f64>,
) {
    let x_values = features.column(0).to_vec(); 
    let y_values = target.column(0).to_vec();  

    let x_range: Vec<f64> = (x_values[0] as i32..=x_values[x_values.len() - 1] as i32)
        .map(|x| x as f64)
        .collect();
    let y_predictions: Vec<f64> = x_range
        .iter()
        .map(|&x| {
            let input = Array2::from_shape_vec((1, 1), vec![x]).unwrap();
            model.predict(&input)[0]
        })
        .collect();

    let root = BitMapBackend::new("linear_regression.png", (640, 480))
        .into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Linear Regression", ("Arial", 20).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..x_values.last().unwrap() + 1.0, 0f64..y_values.iter().cloned().fold(0.0, f64::max) + 1.0)
        .unwrap();

    chart.configure_mesh().draw().unwrap();


    chart
        .draw_series(
            x_values
                .iter()
                .zip(y_values.iter())
                .map(|(&x, &y)| Circle::new((x, y), 5, RED.filled())),
        )
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            x_range.into_iter().zip(y_predictions.into_iter()),
            &BLUE,
        ))
        .unwrap();
}
