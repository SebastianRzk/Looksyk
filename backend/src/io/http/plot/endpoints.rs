use actix_web::{get, Error, HttpResponse};
use plotters::prelude::*;
use chrono::{NaiveDate, Duration};

// A simple example endpoint that returns a generated PNG using Plotters.
// URL: GET /plot/example.png
#[get("/plot/example.png")]
pub async fn example_plot_png() -> Result<HttpResponse, Error> {
    let width = 600u32;
    let height = 300u32;
    let mut buf = vec![0u8; (width * height * 3) as usize]; // RGB buffer
    let caption = "Simple Plotters Example (Dates on X-axis)";
    let label = "value per day";

    // Prepare date range and data points
    let start = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
    let end = start + Duration::days(9);
    let series: Vec<(NaiveDate, i32)> = (0..=9)
        .map(|d| (start + Duration::days(d), d as i32))
        .collect();

    {
        let root = BitMapBackend::with_buffer(&mut buf, (width as u32, height as u32)).into_drawing_area();
        root.fill(&WHITE).map_err(actix_web::error::ErrorInternalServerError)?;

        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .caption(caption, ("sans-serif", 20))
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(RangedDate::new(start..end), 0..10)
            .map_err(actix_web::error::ErrorInternalServerError)?;

        chart.configure_mesh()
            .x_label_formatter(&|d| d.format("%Y-%m-%d").to_string())
            .light_line_style(&WHITE.mix(0.3))
            .draw()
            .map_err(actix_web::error::ErrorInternalServerError)?;

        chart
            .draw_series(LineSeries::new(series.into_iter(), &RED))
            .map_err(actix_web::error::ErrorInternalServerError)?
            .label(label)
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()
            .map_err(actix_web::error::ErrorInternalServerError)?;

        root.present().map_err(actix_web::error::ErrorInternalServerError)?;
    }

    // Encode RGB buffer to PNG
    let mut png_bytes = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut png_bytes, width, height);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().map_err(actix_web::error::ErrorInternalServerError)?;
        writer.write_image_data(&buf).map_err(actix_web::error::ErrorInternalServerError)?;
    }

    Ok(HttpResponse::Ok().content_type("image/png").body(png_bytes))
}
