use crate::io::http::plot::dtos::PlotDataDto;
use crate::io::plot::render_as_svg;
use crate::looksyk::plot::{calculate_plot_data, PlotDataQuery, PlotMetadata};
use crate::state::application_state::AppState;
use crate::state::block_properties::BlockPropertyKey;
use actix_web::web::Data;
use actix_web::{get, Error, HttpResponse};

#[get("/plot/example.svg")]
pub async fn example_plot_svg(data: Data<AppState>) -> Result<HttpResponse, Error> {
    let plot = PlotDataDto {
        label: "Value".to_string(),
        caption: "An example plot with dates on the x-axis".to_string(),
        width: 800,
        height: 600,
        starting_at: "2026-01-01".to_string(),
        ending_at: "2026-01-10".to_string(),
        property_key: "example_property".to_string(),
    };

    let config_guard = data.g_config.lock().unwrap();
    let property_guard = data.h_block_properties.lock().unwrap();

    let plot_data = calculate_plot_data(
        &property_guard,
        PlotDataQuery {
            ending_at: plot.ending_at.clone(),
            starting_at: plot.starting_at.clone(),
            property_key: BlockPropertyKey {
                value: plot.property_key.clone(),
            },
        },
        PlotMetadata {
            label: plot.label.clone(),
            caption: plot.caption.clone(),
            width: plot.width,
            height: plot.height,
        },
    );

    if plot_data.data.points.is_empty() {
        return Ok(HttpResponse::BadRequest().body("no data points"));
    }
    let svg_buf = render_as_svg(&plot_data, &config_guard.design)?;
    Ok(HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(svg_buf))
}
