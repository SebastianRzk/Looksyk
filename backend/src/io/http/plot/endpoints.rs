use crate::io::http::plot::dtos::PlotDataDto;
use crate::io::plot::render_as_svg;
use crate::looksyk::plot::{calculate_plot_data, PlotDataQuery, PlotMetadata};
use crate::state::application_state::AppState;
use crate::state::block_properties::BlockPropertyKey;
use actix_web::http::header;
use actix_web::web::{Data, Query};
use actix_web::{get, Error, HttpResponse};

#[get("/api/plot/")]
pub async fn example_plot_svg(
    data: Data<AppState>,
    dto: Query<PlotDataDto>,
) -> Result<HttpResponse, Error> {
    let plot = dto.into_inner();

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
            label: plot.property_key.clone(),
            title: plot.title.clone(),
            width: plot.width,
            height: plot.height,
        },
    );

    drop(property_guard);

    if plot_data.data.points.is_empty() {
        return Ok(HttpResponse::BadRequest().body("no data points"));
    }
    let svg_buf = render_as_svg(&plot_data, &config_guard.design)?;
    drop(config_guard);

    let mut response = HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(svg_buf);

    response.headers_mut().insert(
        header::CACHE_CONTROL,
        header::HeaderValue::from_static("no-cache, no-store, must-revalidate"),
    );
    response
        .headers_mut()
        .insert(header::PRAGMA, header::HeaderValue::from_static("no-cache"));
    response
        .headers_mut()
        .insert(header::EXPIRES, header::HeaderValue::from_static("0"));

    Ok(response)
}
