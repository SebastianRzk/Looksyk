use crate::io::http::plot::dtos::{DataPointDto, DataPointsDto, PlotDataDto};
use crate::looksyk::plot::{DataPoint, DataPoints, PlotData};
use chrono::NaiveDate;

impl Into<PlotData> for PlotDataDto {
    fn into(self) -> PlotData {
        PlotData {
            label: self.label,
            caption: self.caption,
            width: self.width,
            height: self.height,
            data: self.data.into(),
        }
    }
}

impl Into<DataPoints> for DataPointsDto {
    fn into(self) -> DataPoints {
        DataPoints {
            points: self.points.into_iter().map(|point| point.into()).collect(),
        }
    }
}
impl Into<DataPoint> for DataPointDto {
    fn into(self) -> DataPoint {
        DataPoint {
            date: NaiveDate::parse_from_str(&self.date, "%Y-%m-%d").unwrap(),
            value: self.value,
        }
    }
}
