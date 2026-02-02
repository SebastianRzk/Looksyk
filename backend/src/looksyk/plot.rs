use crate::state::block_properties::{BlockPropertiesIndex, BlockPropertyKey};
use chrono::NaiveDate;

pub struct PlotData {
    pub label: String,
    pub caption: String,
    pub width: u32,
    pub height: u32,
    pub data: DataPoints,
}

pub struct DataPoints {
    pub points: Vec<DataPoint>,
}

#[derive(Clone)]
pub struct DataPoint {
    pub date: NaiveDate,
    pub value: i32,
}

pub struct PlotDataQuery {
    pub property_key: BlockPropertyKey,
    pub starting_at: String,
    pub ending_at: String,
}

pub struct PlotMetadata {
    pub label: String,
    pub title: String,
    pub width: u32,
    pub height: u32,
}

pub fn calculate_plot_data(
    properties_data: &BlockPropertiesIndex,
    plot_data_query: PlotDataQuery,
    plot_metadata: PlotMetadata,
) -> PlotData {
    let mut data_points: Vec<DataPoint> = Vec::new();

    let property_references = properties_data.find(&plot_data_query.property_key);

    if let Some(references) = property_references {
        for reference in references {
            if reference.block.page_id.name.name > plot_data_query.ending_at
                || reference.block.page_id.name.name < plot_data_query.starting_at
            {
                continue;
            }
            let try_parse_value = reference.value.value.parse::<i32>();
            if try_parse_value.is_err() {
                continue;
            }

            let date = &NaiveDate::parse_from_str(&reference.block.page_id.name.name, "%Y_%m_%d");

            if date.is_err() {
                continue;
            }

            data_points.push(DataPoint {
                date: date.unwrap().clone(),
                value: try_parse_value.unwrap(),
            })
        }
    }

    data_points.sort_by(|a, b| a.date.cmp(&b.date));

    PlotData {
        label: plot_metadata.label,
        caption: plot_metadata.title,
        width: plot_metadata.width,
        height: plot_metadata.height,
        data: DataPoints {
            points: data_points,
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::looksyk::builder::test_builder::journal_page_id;
    use crate::looksyk::plot::{calculate_plot_data, PlotDataQuery, PlotMetadata};
    use crate::state::block_properties::builder::{block_property_key, block_property_occurance};
    use crate::state::block_properties::BlockPropertyKey;
    use chrono::NaiveDate;

    #[test]
    fn test_calculate_plot_data_with_empty_tags_should_return_nothing() {
        let result = calculate_plot_data(
            &Default::default(),
            PlotDataQuery {
                property_key: BlockPropertyKey {
                    value: "test".to_string(),
                },
                starting_at: "2024-01-01".to_string(),
                ending_at: "2024-12-31".to_string(),
            },
            demo_metadata(),
        );

        assert!(result.data.points.is_empty());
        assert_eq!(result.label, "Test");
        assert_eq!(result.caption, "Test Caption");
        assert_eq!(result.width, 800);
        assert_eq!(result.height, 600);
    }

    #[test]
    fn test_calculate_plot_data_with_no_matching_dates_should_return_nothing() {
        let mut properties_index = crate::state::block_properties::BlockPropertiesIndex::default();
        let block_property_key = block_property_key("test");
        properties_index.entries.insert(
            block_property_key.clone(),
            vec![
                block_property_occurance("42", journal_page_id("2023_12_31").block_reference(0)),
                block_property_occurance("42", journal_page_id("2025_12_31").block_reference(0)),
            ],
        );

        let result = calculate_plot_data(
            &properties_index,
            PlotDataQuery {
                property_key: block_property_key,
                starting_at: "2024-01-01".to_string(),
                ending_at: "2024-12-31".to_string(),
            },
            demo_metadata(),
        );

        assert!(result.data.points.is_empty());
    }

    #[test]
    fn test_calculate_plot_data_should_return_sorted_points() {
        let mut properties_index = crate::state::block_properties::BlockPropertiesIndex::default();
        let block_property_key = block_property_key("test");
        properties_index.entries.insert(
            block_property_key.clone(),
            vec![
                block_property_occurance("12", journal_page_id("2023_12_31").block_reference(0)),
                block_property_occurance("22", journal_page_id("2025_12_31").block_reference(0)),
                block_property_occurance("32", journal_page_id("2024_12_31").block_reference(0)),
            ],
        );

        let result = calculate_plot_data(
            &properties_index,
            PlotDataQuery {
                property_key: block_property_key,
                starting_at: "1999-01-01".to_string(),
                ending_at: "2222-12-31".to_string(),
            },
            demo_metadata(),
        );

        assert_eq!(result.data.points.len(), 3);
        assert_eq!(
            result.data.points[0].date,
            NaiveDate::from_ymd_opt(2023, 12, 31).unwrap()
        );
        assert_eq!(result.data.points[0].value, 12);
        assert_eq!(
            result.data.points[1].date,
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()
        );
        assert_eq!(result.data.points[1].value, 32);
        assert_eq!(
            result.data.points[2].date,
            NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()
        );
        assert_eq!(result.data.points[2].value, 22);
    }

    #[test]
    fn test_calculate_plot_data_should_skip_non_integer_values() {
        let mut properties_index = crate::state::block_properties::BlockPropertiesIndex::default();
        let block_property_key = block_property_key("test");
        properties_index.entries.insert(
            block_property_key.clone(),
            vec![
                block_property_occurance(
                    "42-234",
                    journal_page_id("2023-12-31").block_reference(0),
                ),
                block_property_occurance("asdf", journal_page_id("2025_12_31").block_reference(0)),
                block_property_occurance("31.22", journal_page_id("2024_12_31").block_reference(0)),
            ],
        );

        let result = calculate_plot_data(
            &properties_index,
            PlotDataQuery {
                property_key: block_property_key,
                starting_at: "1999-01-01".to_string(),
                ending_at: "2222-12-31".to_string(),
            },
            demo_metadata(),
        );

        assert!(result.data.points.is_empty());
    }

    fn demo_metadata() -> PlotMetadata {
        PlotMetadata {
            label: "Test".to_string(),
            title: "Test Caption".to_string(),
            width: 800,
            height: 600,
        }
    }
}
