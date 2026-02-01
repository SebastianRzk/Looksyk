use chrono::NaiveDate;

pub struct PlotData {
    pub label: String,
    pub caption: String,
    pub  width: u32,
    pub height: u32,
    pub data: DataPoints,
}

pub struct DataPoints {
    pub points: Vec<DataPoint>
}

#[derive(Clone)]
pub struct DataPoint {
    pub date: NaiveDate,
    pub value: i32,
}
