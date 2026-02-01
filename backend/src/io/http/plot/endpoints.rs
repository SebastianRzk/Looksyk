use actix_web::{get, Error, HttpResponse};
use plotters::prelude::*;
use chrono::{Duration, NaiveDate};


struct PlotRequest {
    label: String,
    caption: String,
    width: u32,
    height: u32,
    data: DataPoints,
}

struct DataPoints {
    points: Vec<DataPoint>
}

#[derive(Clone)]
struct DataPoint {
    date: NaiveDate,
    value: i32,
}

#[get("/plot/example.svg")]
pub async fn example_plot_svg() -> Result<HttpResponse, Error> {
    let plot = PlotRequest {
        label: "Value".to_string(),
        caption: "An example plot with dates on the x-axis".to_string(),
        width: 800,
        height: 600,
        data: DataPoints {
            points: vec![
                DataPoint { date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(), value: 0 },
                DataPoint { date: NaiveDate::from_ymd_opt(2026, 1, 2).unwrap(), value: 1 },
                DataPoint { date: NaiveDate::from_ymd_opt(2026, 1, 3).unwrap(), value: 2 },
                DataPoint { date: NaiveDate::from_ymd_opt(2026, 1, 4).unwrap(), value: 3 },
                DataPoint { date: NaiveDate::from_ymd_opt(2026, 1, 5).unwrap(), value: 4 },
                DataPoint { date: NaiveDate::from_ymd_opt(2026, 1, 6).unwrap(), value: 5 },
                DataPoint { date: NaiveDate::from_ymd_opt(2026, 1, 7).unwrap(), value: 6 },
                DataPoint { date: NaiveDate::from_ymd_opt(2026, 1, 8).unwrap(), value: 7 },
                DataPoint { date: NaiveDate::from_ymd_opt(2026, 1, 9).unwrap(), value: 8 },
                DataPoint { date: NaiveDate::from_ymd_opt(2026, 1, 10).unwrap(), value: 9 },
            ],
        },
    };

    // Guard: keine Punkte vorhanden
    if plot.data.points.is_empty() {
        return Ok(HttpResponse::BadRequest().body("no data points"));
    }

    // y-Achse dynamisch bestimmen
    let (min_y, max_y) = plot
        .data
        .points
        .iter()
        .fold((i32::MAX, i32::MIN), |(min, max), p| (min.min(p.value), max.max(p.value)));
    let y_range_min = min_y - 1; // kleiner Puffer
    let y_range_max = max_y + 1;

    // SVG Backend mit String-Buffer
    let mut svg_buf: String = String::new();

    {
        let root = SVGBackend::with_string(&mut svg_buf, (plot.width as u32, plot.height as u32)).into_drawing_area();
        root.fill(&WHITE).map_err(actix_web::error::ErrorInternalServerError)?;

        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .caption(&plot.caption, ("sans-serif", 20))
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(plot.data.points[0].date..plot.data.points.last().unwrap().date, y_range_min..y_range_max)
            .map_err(actix_web::error::ErrorInternalServerError)?;

        chart.configure_mesh()
            .x_label_formatter(&|d: &NaiveDate| d.format("%Y-%m-%d").to_string())
            .light_line_style(&WHITE.mix(0.3))
            .draw()
            .map_err(actix_web::error::ErrorInternalServerError)?;

        chart
            .draw_series(LineSeries::new(
                plot.data.points.clone().into_iter().map(|p| (p.date, p.value)),
                &RED,
            ))
            .map_err(actix_web::error::ErrorInternalServerError)?
            .label(&plot.label)
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()
            .map_err(actix_web::error::ErrorInternalServerError)?;

        root.present().map_err(actix_web::error::ErrorInternalServerError)?;
    }

    Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg_buf))
}

