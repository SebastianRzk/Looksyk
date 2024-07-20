use std::io::{Error, ErrorKind};

use crate::looksyk::query::QueryDisplayType;

pub fn parse_property(input_string: &str, expected_property_name: &str) -> Result<PropertyParsed, Error> {
    let prefix = format!("{}:\"", expected_property_name);
    let chop = input_string.strip_prefix(prefix.as_str()).ok_or(Error::new(ErrorKind::Other, format!("Parse error, expected tag '{}', got '{}'", prefix, input_string)))?;
    let mut splittet = chop.splitn(2, "\"");
    let value = splittet.next().ok_or(Error::new(ErrorKind::Other, "Decode error"))?.to_string();
    let remaining_text = splittet.last().ok_or(Error::new(ErrorKind::Other, "Decode error"))?.trim().to_string();
    Ok(PropertyParsed {
        value,
        remaining_text,
    })
}

pub fn parse_display_type(input_string: String) -> Result<QueryDisplayType, Error> {
    let opt = parse_property(input_string.as_str(), "display");
    match opt?.value.as_str() {
        "inplace-list" => Ok(QueryDisplayType::InplaceList),
        "referenced-list" => Ok(QueryDisplayType::ReferencedList),
        "count" => Ok(QueryDisplayType::Count),
        _ => Ok(QueryDisplayType::Unknown)
    }
}

pub struct PropertyParsed {
    pub value: String,
    pub remaining_text: String,
}