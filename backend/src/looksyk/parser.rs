use crate::looksyk::model::BlockTokenType::Text;
use crate::looksyk::model::{
    BlockContent, BlockToken, BlockTokenType, ParsedBlock, ParsedMarkdownFile, RawBlock,
    RawMarkdownFile, UpdateMarkdownFile,
};
use std::cmp::max;
use std::string::ToString;

fn feed_inactive(c: char, state: MatcherState, start_sequence: &str) -> MatcherState {
    let new_index = feed_pattern(c, start_sequence, &state);
    if new_index == start_sequence.len() {
        return MatcherState {
            inner_text: state.inner_text,
            current_index: 0,
            active: true,
        };
    }
    MatcherState {
        inner_text: state.inner_text,
        active: state.active,
        current_index: new_index,
    }
}

fn feed_active(c: char, state: &MatcherState, stop_sequence: &str) -> MatcherState {
    let mut new_inner_text = state.inner_text.clone();

    let new_index = feed_pattern(c, stop_sequence, state);
    new_inner_text.push(c);
    if new_index == stop_sequence.len() {
        let additional_chars_to_remove = stop_sequence.len() - 1;
        let length = state.inner_text.len() - additional_chars_to_remove;
        new_inner_text.truncate(length);
        return MatcherState {
            inner_text: new_inner_text,
            current_index: 0,
            active: false,
        };
    }
    MatcherState {
        inner_text: new_inner_text,
        active: state.active,
        current_index: new_index,
    }
}

fn feed_pattern(c: char, pattern: &str, state: &MatcherState) -> usize {
    if pattern.chars().nth(state.current_index).unwrap() == c {
        return state.current_index + 1;
    }
    0
}

const LINK_START: &str = "[[";
const LINK_END: &str = "]]";

const QUERY_START: &str = "{query: ";
const QUERY_END: &str = " }";

const PROPERTY_START: &str = ":: ";
const PROPERTY_END: &str = " ";

const WORD_BREAKING_CHAR: char = ' ';

pub fn parse_markdown_file(file: RawMarkdownFile) -> ParsedMarkdownFile {
    let mut parsed_blocks = vec![];

    for block in file.blocks {
        parsed_blocks.push(parse_block(&block));
    }

    ParsedMarkdownFile {
        blocks: parsed_blocks,
    }
}

pub fn parse_markdown_update_file(file: UpdateMarkdownFile) -> ParsedMarkdownFile {
    let mut parsed_blocks = vec![];

    for block in file.blocks {
        parsed_blocks.push(parse_block(&block));
    }

    ParsedMarkdownFile {
        blocks: parsed_blocks,
    }
}

pub fn parse_block(raw_block: &RawBlock) -> ParsedBlock {
    let parsed_text_lines = parse_all_text_lines(&raw_block.text_content);
    ParsedBlock {
        content: parsed_text_lines.content,
        indentation: raw_block.indentation,
        properties: parsed_text_lines.properties,
    }
}

pub struct ParseBlockResult {
    pub content: Vec<BlockContent>,
    pub properties: BlockProperties,
}

pub fn parse_all_text_lines(all_lines: &Vec<String>) -> ParseBlockResult {
    let mut parsed_content = vec![];
    let mut block_properties = BlockProperties::empty();

    for line in all_lines {
        let mut result = parse_text_content(line);
        parsed_content.push(BlockContent {
            as_text: line.clone(),
            as_tokens: result.tokens,
        });
        block_properties.append(&mut result.properties);
    }

    ParseBlockResult {
        content: parsed_content,
        properties: block_properties,
    }
}

struct MatcherState {
    current_index: usize,
    active: bool,
    inner_text: Vec<char>,
}

fn create_inactive_matcher_state() -> MatcherState {
    MatcherState {
        active: false,
        current_index: 0,
        inner_text: vec![],
    }
}

pub struct ParseTextResult {
    pub tokens: Vec<BlockToken>,
    pub properties: BlockProperties,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockProperties {
    pub properties: Vec<BlockProperty>,
}

impl BlockProperties {
    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.properties.len()
    }

    #[cfg(test)]
    pub fn get(&self, index: usize) -> Option<&BlockProperty> {
        self.properties.get(index)
    }

    pub fn empty() -> Self {
        BlockProperties { properties: vec![] }
    }

    pub fn append(&mut self, other: &mut BlockProperties) {
        self.properties.append(&mut other.properties);
    }
}

#[cfg(test)]
pub mod builder {
    use crate::looksyk::parser::BlockProperties;

    pub fn any_block_property() -> super::BlockProperty {
        super::BlockProperty {
            key: "key".to_string(),
            value: "value".to_string(),
        }
    }

    pub fn block_properties_from(key: &str, value: &str) -> BlockProperties {
        BlockProperties {
            properties: vec![super::BlockProperty {
                key: key.to_string(),
                value: value.to_string(),
            }],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockProperty {
    pub key: String,
    pub value: String,
}

pub fn parse_text_content(text_content: &str) -> ParseTextResult {
    let mut parsed_tokens = vec![];

    let mut remaining_text_content = text_content.to_string();

    if text_content.starts_with("[ ] ") {
        parsed_tokens.push(BlockToken {
            payload: " ".to_string(),
            block_token_type: BlockTokenType::Todo,
        });
        remaining_text_content = remaining_text_content
            .strip_prefix("[ ] ")
            .unwrap()
            .to_string();
    }
    if text_content.starts_with("[x] ") {
        parsed_tokens.push(BlockToken {
            payload: "x".to_string(),
            block_token_type: BlockTokenType::Todo,
        });
        remaining_text_content = remaining_text_content
            .strip_prefix("[x] ")
            .unwrap()
            .to_string();
    }

    let mut current_matcher: Option<BlockTokenType> = None;
    let mut query_matcher = create_inactive_matcher_state();
    let mut link_matcher = create_inactive_matcher_state();
    let mut property_matcher = create_inactive_matcher_state();

    let mut current_index = 0;
    let mut start_matching_index = 0;
    let mut end_matching_index = 0;

    let mut property_seperator_ended_index = 0;

    let mut last_matching_word_start = 0;
    let mut property_key_word: &str = "";

    let mut properties: Vec<BlockProperty> = vec![];

    for char in remaining_text_content.chars() {
        current_index += char.len_utf8();

        if current_matcher.is_none() {
            link_matcher = feed_inactive(char, link_matcher, LINK_START);
            if link_matcher.active {
                start_matching_index = current_index - LINK_START.len();
                if end_matching_index != start_matching_index {
                    parsed_tokens.push(BlockToken {
                        payload: remaining_text_content[end_matching_index..start_matching_index]
                            .to_string(),
                        block_token_type: Text,
                    })
                }
                current_matcher = Some(BlockTokenType::Link);
            }

            query_matcher = feed_inactive(char, query_matcher, QUERY_START);
            if query_matcher.active {
                start_matching_index = current_index - QUERY_START.len();
                if end_matching_index != start_matching_index {
                    parsed_tokens.push(BlockToken {
                        payload: remaining_text_content[end_matching_index..start_matching_index]
                            .to_string(),
                        block_token_type: Text,
                    })
                };
                current_matcher = Some(BlockTokenType::Query);
            }

            property_matcher = feed_inactive(char, property_matcher, PROPERTY_START);
            if property_matcher.active {
                start_matching_index = last_matching_word_start;
                if end_matching_index != start_matching_index {
                    parsed_tokens.push(BlockToken {
                        payload: remaining_text_content[end_matching_index..start_matching_index]
                            .to_string(),
                        block_token_type: Text,
                    })
                }
                property_key_word = &remaining_text_content
                    [last_matching_word_start..current_index - PROPERTY_START.len()];
                current_matcher = Some(BlockTokenType::Property);
                property_seperator_ended_index = current_index;
            }
            if char == WORD_BREAKING_CHAR {
                last_matching_word_start = current_index;
            }
        } else {
            match current_matcher {
                Some(BlockTokenType::Link) => {
                    link_matcher = feed_active(char, &link_matcher, LINK_END);
                    if !link_matcher.active {
                        let end_index = current_index - LINK_END.len();
                        let start_index = start_matching_index + LINK_START.len();
                        parsed_tokens.push(BlockToken {
                            payload: remaining_text_content[start_index..end_index].to_string(),
                            block_token_type: BlockTokenType::Link,
                        });
                        current_matcher = None;
                        end_matching_index = current_index;
                        link_matcher = create_inactive_matcher_state();
                        query_matcher = create_inactive_matcher_state();
                        property_matcher = create_inactive_matcher_state();
                    }
                }
                Some(BlockTokenType::Query) => {
                    query_matcher = feed_active(char, &query_matcher, QUERY_END);
                    if !query_matcher.active {
                        let end_index = current_index - QUERY_END.len();
                        let start_index = start_matching_index + QUERY_START.len();
                        parsed_tokens.push(BlockToken {
                            payload: remaining_text_content[start_index..end_index].to_string(),
                            block_token_type: BlockTokenType::Query,
                        });
                        current_matcher = None;
                        end_matching_index = current_index;
                        link_matcher = create_inactive_matcher_state();
                        query_matcher = create_inactive_matcher_state();
                        property_matcher = create_inactive_matcher_state();
                    }
                }
                Some(BlockTokenType::Property) => {
                    property_matcher = feed_active(char, &property_matcher, PROPERTY_END);
                    if !property_matcher.active {
                        let end_index = current_index - PROPERTY_END.len();
                        parsed_tokens.push(BlockToken {
                            payload: remaining_text_content[start_matching_index..end_index]
                                .to_string(),
                            block_token_type: BlockTokenType::Property,
                        });
                        properties.push(BlockProperty {
                            key: property_key_word.to_string(),
                            value: remaining_text_content
                                [property_seperator_ended_index..end_index]
                                .to_string(),
                        });
                        current_matcher = None;
                        end_matching_index = current_index - PROPERTY_END.len();
                        link_matcher = create_inactive_matcher_state();
                        query_matcher = create_inactive_matcher_state();
                        property_matcher = create_inactive_matcher_state();
                    }
                }
                _ => {}
            }
        }
    }
    if let Some(active_matcher) = current_matcher {
        match active_matcher {
            //Property at the end of the line
            BlockTokenType::Property => {
                parsed_tokens.push(BlockToken {
                    payload: remaining_text_content[start_matching_index..].to_string(),
                    block_token_type: BlockTokenType::Property,
                });
                properties.push(BlockProperty {
                    key: property_key_word.to_string(),
                    value: remaining_text_content[property_seperator_ended_index..].to_string(),
                });
            }
            _ => {
                if start_matching_index != remaining_text_content.len() {
                    parsed_tokens.push(remaining_as_text(
                        remaining_text_content,
                        start_matching_index,
                        end_matching_index,
                    ))
                }
            }
        }
    } else if start_matching_index != remaining_text_content.len() {
        parsed_tokens.push(remaining_as_text(
            remaining_text_content,
            start_matching_index,
            end_matching_index,
        ))
    }
    ParseTextResult {
        tokens: parsed_tokens,
        properties: BlockProperties { properties },
    }
}

fn remaining_as_text(
    remaining_text_content: String,
    start_matching_index: usize,
    end_matching_index: usize,
) -> BlockToken {
    BlockToken {
        payload: remaining_text_content[max(start_matching_index, end_matching_index)..]
            .to_string(),
        block_token_type: Text,
    }
}

#[cfg(test)]
mod tests {
    use crate::looksyk::model::BlockTokenType;
    use crate::looksyk::parser::builder::any_block_property;
    use crate::looksyk::parser::{
        parse_text_content, BlockProperties, BlockProperty, ParseTextResult,
    };

    fn test_not_properties(result: &ParseTextResult) {
        assert_eq!(result.properties.properties.len(), 0);
    }

    #[test]
    fn should_create_text_node_on_unclosed_pattern() {
        let input_text = "davor [[link".to_string();

        let result = parse_text_content(&input_text);

        test_not_properties(&result);
        assert_eq!(result.tokens.len(), 2);
        let element = result.tokens.get(1).unwrap();
        assert_eq!(element.payload, "[[link");
        assert_eq!(element.block_token_type, BlockTokenType::Text);
    }

    #[test]
    fn should_create_only_text_node() {
        let input_text = "das ist ein kleiner test".to_string();

        let result = parse_text_content(&input_text);

        test_not_properties(&result);
        assert_eq!(result.tokens.len(), 1);
        let element = result.tokens.first().unwrap();
        assert_eq!(element.payload, input_text);
        assert_eq!(element.block_token_type, BlockTokenType::Text);
    }

    #[test]
    fn should_create_text_and_link_node() {
        let input_text = "davor [[link]] dahinter".to_string();
        let result = parse_text_content(&input_text);

        test_not_properties(&result);
        assert_eq!(result.tokens.len(), 3);
        let element = result.tokens.first().unwrap();
        assert_eq!(element.payload, "davor ");
        assert_eq!(element.block_token_type, BlockTokenType::Text);

        let element = result.tokens.get(1).unwrap();
        assert_eq!(element.payload, "link");
        assert_eq!(element.block_token_type, BlockTokenType::Link);

        let element = result.tokens.get(2).unwrap();
        assert_eq!(element.payload, " dahinter");
        assert_eq!(element.block_token_type, BlockTokenType::Text);
    }

    #[test]
    fn should_create_text_and_link_node_with_char_larger_1() {
        let input_text = "davor [[länk]] dahinter".to_string();
        let result = parse_text_content(&input_text);

        test_not_properties(&result);
        assert_eq!(result.tokens.len(), 3);
        let element = result.tokens.first().unwrap();
        assert_eq!(element.payload, "davor ");
        assert_eq!(element.block_token_type, BlockTokenType::Text);

        let element = result.tokens.get(1).unwrap();
        assert_eq!(element.payload, "länk");
        assert_eq!(element.block_token_type, BlockTokenType::Link);

        let element = result.tokens.get(2).unwrap();
        assert_eq!(element.payload, " dahinter");
        assert_eq!(element.block_token_type, BlockTokenType::Text);
    }

    #[test]
    fn should_parse_query_type() {
        let input_text = "davor {query: querycontent } dahinter".to_string();
        let result = parse_text_content(&input_text);

        test_not_properties(&result);
        assert_eq!(result.tokens.len(), 3);
        let element = result.tokens.first().unwrap();
        assert_eq!(element.payload, "davor ");
        assert_eq!(element.block_token_type, BlockTokenType::Text);

        let element = result.tokens.get(1).unwrap();
        assert_eq!(element.payload, "querycontent");
        assert_eq!(element.block_token_type, BlockTokenType::Query);

        let element = result.tokens.get(2).unwrap();
        assert_eq!(element.payload, " dahinter");
        assert_eq!(element.block_token_type, BlockTokenType::Text);
    }

    #[test]
    fn should_parse_todo_type_unchecked() {
        let input_text = "[ ] Ein kleines TODO".to_string();
        let result = parse_text_content(&input_text);

        test_not_properties(&result);
        assert_eq!(result.tokens.len(), 2);
        let element = result.tokens.first().unwrap();
        assert_eq!(element.payload, " ");
        assert_eq!(element.block_token_type, BlockTokenType::Todo);

        let element = result.tokens.get(1).unwrap();
        assert_eq!(element.payload, "Ein kleines TODO");
        assert_eq!(element.block_token_type, BlockTokenType::Text);
    }

    #[test]
    fn should_parse_todo_type_checked() {
        let input_text = "[x] Ein kleines TODO".to_string();
        let result = parse_text_content(&input_text);

        test_not_properties(&result);
        assert_eq!(result.tokens.len(), 2);
        let element = result.tokens.first().unwrap();
        assert_eq!(element.payload, "x");
        assert_eq!(element.block_token_type, BlockTokenType::Todo);

        let element = result.tokens.get(1).unwrap();
        assert_eq!(element.payload, "Ein kleines TODO");
        assert_eq!(element.block_token_type, BlockTokenType::Text);
    }

    #[test]
    fn should_not_parse_on_splitted_start_signal() {
        let input_text = "davor [123[link]]".to_string();
        let result = parse_text_content(&input_text);

        test_not_properties(&result);
        assert_eq!(result.tokens.len(), 1);
        let element = result.tokens.first().unwrap();
        assert_eq!(element.payload, "davor [123[link]]");
        assert_eq!(element.block_token_type, BlockTokenType::Text);
    }

    #[test]
    fn should_parse_blank_property_in_line_surrounded_by_text() {
        let input_text = "asd key:: value fgh".to_string();
        let result = parse_text_content(&input_text);

        assert_eq!(result.properties.len(), 1);
        let property1 = result.properties.get(0).unwrap();
        assert_eq!(property1.key, "key");
        assert_eq!(property1.value, "value");

        assert_eq!(result.tokens.len(), 3);
        let element = result.tokens.first().unwrap();
        assert_eq!(element.payload, "asd ");
        assert_eq!(element.block_token_type, BlockTokenType::Text);

        let element = result.tokens.get(1).unwrap();
        assert_eq!(element.payload, "key:: value");
        assert_eq!(element.block_token_type, BlockTokenType::Property);

        let element = result.tokens.get(2).unwrap();
        assert_eq!(element.payload, " fgh");
        assert_eq!(element.block_token_type, BlockTokenType::Text);
    }

    #[test]
    fn should_parse_blank_property_in_line() {
        let input_text = "key:: value".to_string();
        let result = parse_text_content(&input_text);

        assert_eq!(result.properties.len(), 1);
        let property1 = result.properties.get(0).unwrap();
        assert_eq!(property1.key, "key");
        assert_eq!(property1.value, "value");

        assert_eq!(result.tokens.len(), 1);
        let element = result.tokens.first().unwrap();
        assert_eq!(element.payload, input_text);
        assert_eq!(element.block_token_type, BlockTokenType::Property);
    }

    #[test]
    fn should_parse_property_inline() {
        let input_text = "This is a line key:: value with property".to_string();
        let result = parse_text_content(&input_text);

        assert_eq!(result.properties.len(), 1);
        let property1 = result.properties.get(0).unwrap();
        assert_eq!(property1.key, "key");
        assert_eq!(property1.value, "value");

        assert_eq!(result.tokens.len(), 3);
        let element = result.tokens.first().unwrap();
        assert_eq!(element.payload, "This is a line ");
        assert_eq!(element.block_token_type, BlockTokenType::Text);
        let element = result.tokens.get(1).unwrap();
        assert_eq!(element.payload, "key:: value");
        assert_eq!(element.block_token_type, BlockTokenType::Property);
        let element = result.tokens.get(2).unwrap();
        assert_eq!(element.payload, " with property");
        assert_eq!(element.block_token_type, BlockTokenType::Text);
    }

    #[test]
    fn should_parse_multiple_properties_inline() {
        let input_text =
            "This is a line key1:: value1 and key2:: value2 with properties".to_string();
        let result = parse_text_content(&input_text);

        assert_eq!(result.properties.len(), 2);
        let property1 = result.properties.get(0).unwrap();
        assert_eq!(property1.key, "key1");
        assert_eq!(property1.value, "value1");

        let property2 = result.properties.get(1).unwrap();
        assert_eq!(property2.key, "key2");
        assert_eq!(property2.value, "value2");

        assert_eq!(result.tokens.len(), 5);
        let element = result.tokens.first().unwrap();
        assert_eq!(element.payload, "This is a line ");
        assert_eq!(element.block_token_type, BlockTokenType::Text);
        let element = result.tokens.get(1).unwrap();
        assert_eq!(element.payload, "key1:: value1");
        assert_eq!(element.block_token_type, BlockTokenType::Property);
        let element = result.tokens.get(2).unwrap();
        assert_eq!(element.payload, " and ");
        assert_eq!(element.block_token_type, BlockTokenType::Text);
        let element = result.tokens.get(3).unwrap();
        assert_eq!(element.payload, "key2:: value2");
        assert_eq!(element.block_token_type, BlockTokenType::Property);
        let element = result.tokens.get(4).unwrap();
        assert_eq!(element.payload, " with properties");
        assert_eq!(element.block_token_type, BlockTokenType::Text);
    }

    #[test]
    fn should_parse_property_with_special_characters() {
        let input_text = "This is a line käöy-1:: valußä_1 with property".to_string();
        let result = parse_text_content(&input_text);

        assert_eq!(result.properties.len(), 1);
        let property1 = result.properties.get(0).unwrap();
        assert_eq!(property1.key, "käöy-1");
        assert_eq!(property1.value, "valußä_1");

        assert_eq!(result.tokens.len(), 3);
        let element = result.tokens.first().unwrap();
        assert_eq!(element.payload, "This is a line ");
        assert_eq!(element.block_token_type, BlockTokenType::Text);
        let element = result.tokens.get(1).unwrap();
        assert_eq!(element.payload, "käöy-1:: valußä_1");
        assert_eq!(element.block_token_type, BlockTokenType::Property);
        let element = result.tokens.get(2).unwrap();
        assert_eq!(element.payload, " with property");
        assert_eq!(element.block_token_type, BlockTokenType::Text);
    }

    #[test]
    fn should_count_properties_in_len() {
        assert_eq!(BlockProperties { properties: vec![] }.len(), 0);
        assert_eq!(
            BlockProperties {
                properties: vec![any_block_property()]
            }
            .len(),
            1
        );
    }

    #[test]
    fn should_append() {
        let mut props1 = BlockProperties {
            properties: vec![BlockProperty {
                key: "key1".to_string(),
                value: "value1".to_string(),
            }],
        };
        let mut props2 = BlockProperties {
            properties: vec![BlockProperty {
                key: "key2".to_string(),
                value: "value2".to_string(),
            }],
        };
        props1.append(&mut props2);
        assert_eq!(props1.len(), 2);
        let prop1 = props1.get(0).unwrap();
        assert_eq!(prop1.key, "key1");
        assert_eq!(prop1.value, "value1");
        let prop2 = props1.get(1).unwrap();
        assert_eq!(prop2.key, "key2");
        assert_eq!(prop2.value, "value2");
    }
}
