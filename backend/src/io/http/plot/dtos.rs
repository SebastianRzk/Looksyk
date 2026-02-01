pub struct PlotDataDto {
    pub label: String,
    pub caption: String,
    pub  width: u32,
    pub height: u32,
    pub data: DataPointsDto,
}

pub struct DataPointsDto {
    pub points: Vec<DataPointDto>
}

#[derive(Clone)]
pub struct DataPointDto {
    pub date: String,
    pub value: i32,
}
