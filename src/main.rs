use plotters::prelude::*;

use dice_string_parser::dice_distribtion;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root =
        BitMapBackend::new("histogram.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Histogram Test", ("Arial", 50.0).into_font())
        .build_ranged(1i64..30i64, 0i64..100i64)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .line_style_1(&WHITE.mix(0.3))
        .x_label_offset(30)
        .y_desc("Count")
        .x_desc("Bucket")
        .axis_desc_style(("Arial", 15).into_font())
        .draw()?;

    let data = dice_distribtion("2d6 + 2d4");
    println!("{}", data.iter().count());

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(data.iter().map(|x| (*x, 1))),
    )?;

    Ok(())
}
