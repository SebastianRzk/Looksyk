use crate::io::http::plot::dtos::{DataPointDto, DataPointsDto, PlotDataDto};
use crate::io::plot::render_as_svg;
use crate::state::application_state::AppState;
use actix_web::web::Data;
use actix_web::{get, Error, HttpResponse};
use crate::looksyk::data::config::runtime_graph_configuration::Design;

#[get("/plot/example.svg")]
pub async fn example_plot_svg(data: Data<AppState>) -> Result<HttpResponse, Error> {
    let plot = PlotDataDto {
        label: "Value".to_string(),
        caption: "An example plot with dates on the x-axis".to_string(),
        width: 800,
        height: 600,
        data: DataPointsDto {
            points: vec![
                DataPointDto {
                    date: "2026-01-01".to_string(),
                    value: 0,
                },
                DataPointDto {
                    date: "2026-01-02".to_string(),
                    value: 1,
                },
                DataPointDto {
                    date: "2026-01-03".to_string(),
                    value: 2,
                },
                DataPointDto {
                    date: "2026-01-04".to_string(),
                    value: 3,
                },
                DataPointDto {
                    date: "2026-01-05".to_string(),
                    value: 4,
                },
                DataPointDto {
                    date: "2026-01-06".to_string(),
                    value: 5,
                },
                DataPointDto {
                    date: "2026-01-07".to_string(),
                    value: 6,
                },
                DataPointDto {
                    date: "2026-01-08".to_string(),
                    value: 7,
                },
                DataPointDto {
                    date: "2026-01-09".to_string(),
                    value: 8,
                },
                DataPointDto {
                    date: "2026-01-10".to_string(),
                    value: 7,
                },
            ],
        },
    };

    if plot.data.points.is_empty() {
        return Ok(HttpResponse::BadRequest().body("no data points"));
    }
    let svg_buf = render_as_svg(&plot.into(), &Design{
        primary_color: "blue".to_string(),
        primary_shading: "lightblue".to_string(),
        foreground_color: "black".to_string(),
        background_color: "white".to_string(),
        appearance: crate::looksyk::data::config::runtime_graph_configuration::Appearance::Light,
    })?;
    Ok(HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(svg_buf))
}
