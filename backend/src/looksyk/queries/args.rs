use std::collections::HashMap;
use std::io::Error;

use crate::looksyk::query::QueryDisplayType;

pub const PARAM_DISPLAY: &str = "display";
pub const PARAM_DISPLAY_INPLACE_LIST: &str = "inplace-list";
pub const PARAM_DISPLAY_REFERENCED_LIST: &str = "referenced-list";
pub const PARAM_DISPLAY_COUNT: &str = "count";
pub const PARAM_DISPLAY_PARAGRAPH: &str = "paragraphs";
pub const PARAM_DISPLAY_CODE_BLOCK: &str = "code-block";
pub const PARAM_DISPLAY_INLINE_TEXT: &str = "inline-text";
pub const PARAM_DISPLAY_VIDEO: &str = "video";
pub const PARAM_DISPLAY_AUDIO: &str = "audio";
pub const PARAM_DISPLAY_LINK: &str = "link";
pub const PARAM_DISPLAY_CARDS: &str = "cards";
pub const PARAM_DISPLAY_LINECHART: &str = "linechart";

pub const PARAM_TARGET_FILE: &str = "target-file";
pub const PARAM_TARGET: &str = "target";
pub const PARAM_ROOT: &str = "root";
pub const PARAM_STATE: &str = "state";
pub const PARAM_TAG: &str = "tag";

pub const ERROR_CAN_NOT_STRIP_QUERY_NAME_PREFIX: &str =
    "Decode error: Can not strip query name prefix";

pub fn parse_property(
    input_string: &str,
    expected_property_name: &str,
) -> Result<PropertyParsed, Error> {
    let prefix = format!("{expected_property_name}:\"");
    let chop = input_string
        .strip_prefix(prefix.as_str())
        .ok_or(Error::other(format!(
            "Parse error, expected tag '{prefix}', got '{input_string}'"
        )))?;
    let mut splittet = chop.splitn(2, "\"");
    let value = splittet
        .next()
        .ok_or(Error::other("Decode error"))?
        .to_string();
    let remaining_text = splittet
        .last()
        .ok_or(Error::other("Decode error"))?
        .trim()
        .to_string();
    Ok(PropertyParsed {
        value,
        remaining_text,
    })
}

pub fn parse_display_type(input_string: String) -> Result<QueryDisplayType, Error> {
    let opt = parse_property(input_string.as_str(), PARAM_DISPLAY);
    match opt?.value.as_str() {
        PARAM_DISPLAY_INPLACE_LIST => Ok(QueryDisplayType::InplaceList),
        PARAM_DISPLAY_REFERENCED_LIST => Ok(QueryDisplayType::ReferencedList),
        PARAM_DISPLAY_COUNT => Ok(QueryDisplayType::Count),
        PARAM_DISPLAY_PARAGRAPH => Ok(QueryDisplayType::Paragraphs),
        PARAM_DISPLAY_CODE_BLOCK => Ok(QueryDisplayType::CodeBlock),
        PARAM_DISPLAY_INLINE_TEXT => Ok(QueryDisplayType::InlineText),
        PARAM_DISPLAY_VIDEO => Ok(QueryDisplayType::Video),
        PARAM_DISPLAY_AUDIO => Ok(QueryDisplayType::Audio),
        PARAM_DISPLAY_LINK => Ok(QueryDisplayType::Link),
        PARAM_DISPLAY_CARDS => Ok(QueryDisplayType::Cards),
        PARAM_DISPLAY_LINECHART => Ok(QueryDisplayType::Linechart),
        _ => Ok(QueryDisplayType::Unknown),
    }
}

pub struct ParamBuilder {
    current_value: String,
    parsed_args: HashMap<String, String>,
}

impl ParamBuilder {
    pub fn init(content: String) -> ParamBuilder {
        ParamBuilder {
            parsed_args: HashMap::new(),
            current_value: content,
        }
    }

    pub fn next(mut self, key: &str) -> Result<ParamBuilder, Error> {
        let result = parse_property(&self.current_value, key)?;
        self.parsed_args.insert(key.to_owned(), result.value);
        Ok(ParamBuilder {
            current_value: result.remaining_text,
            parsed_args: self.parsed_args,
        })
    }

    pub fn build(self) -> ParamBuilderResult {
        ParamBuilderResult {
            parsed_args: self.parsed_args,
            remaining_value: self.current_value,
        }
    }
}

pub struct ParamBuilderResult {
    pub remaining_value: String,
    pub parsed_args: HashMap<String, String>,
}

pub struct PropertyParsed {
    pub value: String,
    pub remaining_text: String,
}
