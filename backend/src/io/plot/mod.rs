use std::str::FromStr;
use crate::looksyk::plot::PlotData;
use actix_web::Error;
use chrono::NaiveDate;
use plotters::backend::SVGBackend;
use plotters::chart::ChartBuilder;
use plotters::element::PathElement;
use plotters::prelude::{Color, IntoDrawingArea, IntoTextStyle, LineSeries, BLACK, RED, WHITE};
use plotters::style::{RGBAColor, TextStyle};
use crate::looksyk::data::config::runtime_graph_configuration::Design;
use csscolorparser::Color as CssColor;

fn css_to_plotters_rgba(s: &str, default: RGBAColor) -> RGBAColor {
    match CssColor::from_str(s) {
        Ok(c) => {
            RGBAColor(
                (c.r * 255.0).round() as u8,
                (c.g * 255.0).round() as u8,
                (c.b * 255.0).round() as u8,
                c.a as f64,
            )
        }
        Err(_) => default,
    }
}

pub fn render_as_svg(plot: &PlotData, design: &Design) -> Result<String, Error> {
    let (min_y, max_y) = plot
        .data
        .points
        .iter()
        .fold((i32::MAX, i32::MIN), |(min, max), p| (min.min(p.value), max.max(p.value)));
    let y_range_min = min_y - 1; // kleiner Puffer
    let y_range_max = max_y + 1;

    // Farben abhängig vom Design-Appearance ODER explizit aus Design, falls gesetzt
    let axis_color = css_to_plotters_rgba(&design.foreground_color, match design.appearance {
        crate::looksyk::data::config::runtime_graph_configuration::Appearance::Light => BLACK.mix(0.8),
        crate::looksyk::data::config::runtime_graph_configuration::Appearance::Dark => WHITE.mix(0.8),
    });
    let grid_color = css_to_plotters_rgba(&design.primary_shading, match design.appearance {
        crate::looksyk::data::config::runtime_graph_configuration::Appearance::Light => BLACK.mix(0.1),
        crate::looksyk::data::config::runtime_graph_configuration::Appearance::Dark => WHITE.mix(0.3),
    });
    let label_color = css_to_plotters_rgba(&design.foreground_color, match design.appearance {
        crate::looksyk::data::config::runtime_graph_configuration::Appearance::Light => BLACK.to_rgba(),
        crate::looksyk::data::config::runtime_graph_configuration::Appearance::Dark => WHITE.to_rgba(),
    });

    let graph_color = css_to_plotters_rgba(&design.primary_color, RED.to_rgba());

    // SVG Backend mit String-Buffer
    let mut svg_buf: String = String::new();

    {
        let root = SVGBackend::with_string(&mut svg_buf, (plot.width as u32, plot.height as u32)).into_drawing_area();
        // Hintergrund transparent statt weiß
        root.fill(&RGBAColor(0, 0, 0, 0.0)).map_err(actix_web::error::ErrorInternalServerError)?;

        let caption_style = TextStyle::from(("sans-serif", 20)).with_color(&label_color);

        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .caption(&plot.caption, caption_style)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(plot.data.points[0].date..plot.data.points.last().unwrap().date, y_range_min..y_range_max)
            .map_err(actix_web::error::ErrorInternalServerError)?;

        chart.configure_mesh()
            .x_label_formatter(&|d: &NaiveDate| d.format("%Y-%m-%d").to_string())
            .label_style(TextStyle::from(("sans-serif", 12)).with_color(&label_color))
            .axis_style(axis_color)
            .light_line_style(&grid_color)
            .draw()
            .map_err(actix_web::error::ErrorInternalServerError)?;

        chart
            .draw_series(LineSeries::new(
                plot.data.points.clone().into_iter().map(|p| (p.date, p.value)),
                &graph_color,
            ))
            .map_err(actix_web::error::ErrorInternalServerError)?
            .label(&plot.label)
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], graph_color));

        chart
            .configure_series_labels()
            // Legenden-Hintergrund ebenfalls transparent
            .background_style(&RGBAColor(0, 0, 0, 0.0))
            .border_style(&axis_color)
            // Farbe des Legendentextes auf label_color setzen
            .label_font(TextStyle::from(("sans-serif", 12)).with_color(&label_color))
            .draw()
            .map_err(actix_web::error::ErrorInternalServerError)?;

        root.present().map_err(actix_web::error::ErrorInternalServerError)?;
    }
    Ok(svg_buf)
}
