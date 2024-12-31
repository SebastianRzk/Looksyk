use std::cmp::max;
use std::collections::HashMap;
use std::string::ToString;

use crate::looksyk::model::BlockTokenType::TEXT;
use crate::looksyk::model::{
    BlockContent, BlockToken, BlockTokenType, ParsedBlock, ParsedMarkdownFile, RawBlock,
    RawMarkdownFile, UpdateMarkdownFile,
};

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
    let new_index;
    new_index = feed_pattern(c, stop_sequence.clone(), state);
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
    ParsedBlock {
        content: parse_all_text_lines(&raw_block.text_content),
        indentation: raw_block.indentation,
    }
}

pub fn parse_all_text_lines(all_lines: &Vec<String>) -> Vec<BlockContent> {
    let mut parsed_content = vec![];

    for line in all_lines {
        parsed_content.push(BlockContent {
            as_text: line.clone(),
            as_tokens: parse_text_content(line),
        });
    }

    parsed_content
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

fn create_initial_matcher_states() -> HashMap<BlockTokenType, MatcherState> {
    let mut matcher_state = HashMap::new();
    matcher_state.insert(BlockTokenType::LINK, create_inactive_matcher_state());
    matcher_state.insert(BlockTokenType::QUERY, create_inactive_matcher_state());
    matcher_state
}

pub fn parse_text_content(text_content: &String) -> Vec<BlockToken> {
    let mut parsed_tokens = vec![];

    let mut remaining_text_content = text_content.clone();

    if text_content.starts_with("[ ] ") {
        parsed_tokens.push(BlockToken {
            payload: " ".to_string(),
            block_token_type: BlockTokenType::TODO,
        });
        remaining_text_content = remaining_text_content
            .strip_prefix("[ ] ")
            .unwrap()
            .to_string();
    }
    if text_content.starts_with("[x] ") {
        parsed_tokens.push(BlockToken {
            payload: "x".to_string(),
            block_token_type: BlockTokenType::TODO,
        });
        remaining_text_content = remaining_text_content
            .strip_prefix("[x] ")
            .unwrap()
            .to_string();
    }

    let mut current_matcher: Option<BlockTokenType> = None;
    let mut query_matcher = create_inactive_matcher_state();
    let mut link_matcher = create_inactive_matcher_state();
    let mut current_index = 0;
    let mut start_matching_index = 0;
    let mut end_matching_index = 0;

    for char in remaining_text_content.chars() {
        current_index = current_index + char.len_utf8();
        if current_matcher.is_none() {
            link_matcher = feed_inactive(char, link_matcher, LINK_START);
            if link_matcher.active {
                start_matching_index = current_index - LINK_START.len();
                parsed_tokens.push(BlockToken {
                    payload: remaining_text_content[end_matching_index..start_matching_index]
                        .to_string(),
                    block_token_type: TEXT,
                });
                current_matcher = Some(BlockTokenType::LINK);
            }

            query_matcher = feed_inactive(char, query_matcher, QUERY_START);
            if query_matcher.active {
                start_matching_index = current_index - QUERY_START.len();
                parsed_tokens.push(BlockToken {
                    payload: remaining_text_content[end_matching_index..start_matching_index]
                        .to_string(),
                    block_token_type: TEXT,
                });
                current_matcher = Some(BlockTokenType::QUERY);
            }
        } else {
            match current_matcher {
                Some(BlockTokenType::LINK) => {
                    link_matcher = feed_active(char, &link_matcher, LINK_END);
                    if !link_matcher.active {
                        let end_index = current_index - LINK_END.len();
                        let start_index = start_matching_index + LINK_START.len();
                        parsed_tokens.push(BlockToken {
                            payload: remaining_text_content[start_index..end_index].to_string(),
                            block_token_type: BlockTokenType::LINK,
                        });
                        current_matcher = None;
                        end_matching_index = current_index;
                        link_matcher = create_inactive_matcher_state();
                        query_matcher = create_inactive_matcher_state();
                    }
                }
                Some(BlockTokenType::QUERY) => {
                    query_matcher = feed_active(char, &query_matcher, QUERY_END);
                    if !query_matcher.active {
                        let end_index = current_index - QUERY_END.len();
                        let start_index = start_matching_index + QUERY_START.len();
                        parsed_tokens.push(BlockToken {
                            payload: remaining_text_content[start_index..end_index].to_string(),
                            block_token_type: BlockTokenType::QUERY,
                        });
                        current_matcher = None;
                        end_matching_index = current_index;
                        link_matcher = create_inactive_matcher_state();
                        query_matcher = create_inactive_matcher_state();
                    }
                }
                _ => {}
            }
        }
    }

    parsed_tokens.push(BlockToken {
        payload: remaining_text_content[max(start_matching_index, end_matching_index)..]
            .to_string(),
        block_token_type: TEXT,
    });
    parsed_tokens
}

#[cfg(test)]
mod tests {
    use crate::looksyk::model::BlockTokenType;
    use crate::looksyk::parser::parse_text_content;

    #[test]
    fn should_create_text_node_on_unclosed_pattern() {
        let input_text = "davor [[link".to_string();
        let result = parse_text_content(&input_text);
        assert_eq!(result.len(), 2);
        let element = result.get(1).unwrap();
        assert_eq!(element.payload, "[[link");
        assert_eq!(element.block_token_type, BlockTokenType::TEXT);
    }

    #[test]
    fn should_create_only_text_node() {
        let input_text = "das ist ein kleiner test".to_string();
        let result = parse_text_content(&input_text);
        assert_eq!(result.len(), 1);
        let element = result.get(0).unwrap();
        assert_eq!(element.payload, input_text);
        assert_eq!(element.block_token_type, BlockTokenType::TEXT);
    }

    #[test]
    fn should_create_text_and_link_node() {
        let input_text = "davor [[link]] dahinter".to_string();
        let result = parse_text_content(&input_text);

        assert_eq!(result.len(), 3);
        let element = result.get(0).unwrap();
        assert_eq!(element.payload, "davor ");
        assert_eq!(element.block_token_type, BlockTokenType::TEXT);

        let element = result.get(1).unwrap();
        assert_eq!(element.payload, "link");
        assert_eq!(element.block_token_type, BlockTokenType::LINK);

        let element = result.get(2).unwrap();
        assert_eq!(element.payload, " dahinter");
        assert_eq!(element.block_token_type, BlockTokenType::TEXT);
    }

    #[test]
    fn should_create_text_and_link_node_with_char_larger_1() {
        let input_text = "davor [[länk]] dahinter".to_string();
        let result = parse_text_content(&input_text);

        assert_eq!(result.len(), 3);
        let element = result.get(0).unwrap();
        assert_eq!(element.payload, "davor ");
        assert_eq!(element.block_token_type, BlockTokenType::TEXT);

        let element = result.get(1).unwrap();
        assert_eq!(element.payload, "länk");
        assert_eq!(element.block_token_type, BlockTokenType::LINK);

        let element = result.get(2).unwrap();
        assert_eq!(element.payload, " dahinter");
        assert_eq!(element.block_token_type, BlockTokenType::TEXT);
    }

    #[test]
    fn should_parse_query_type() {
        let input_text = "davor {query: querycontent } dahinter".to_string();
        let result = parse_text_content(&input_text);

        assert_eq!(result.len(), 3);
        let element = result.get(0).unwrap();
        assert_eq!(element.payload, "davor ");
        assert_eq!(element.block_token_type, BlockTokenType::TEXT);

        let element = result.get(1).unwrap();
        assert_eq!(element.payload, "querycontent");
        assert_eq!(element.block_token_type, BlockTokenType::QUERY);

        let element = result.get(2).unwrap();
        assert_eq!(element.payload, " dahinter");
        assert_eq!(element.block_token_type, BlockTokenType::TEXT);
    }

    #[test]
    fn should_parse_todo_type_unchecked() {
        let input_text = "[ ] Ein kleines TODO".to_string();
        let result = parse_text_content(&input_text);

        assert_eq!(result.len(), 2);
        let element = result.get(0).unwrap();
        assert_eq!(element.payload, " ");
        assert_eq!(element.block_token_type, BlockTokenType::TODO);

        let element = result.get(1).unwrap();
        assert_eq!(element.payload, "Ein kleines TODO");
        assert_eq!(element.block_token_type, BlockTokenType::TEXT);
    }

    #[test]
    fn should_parse_todo_type_checked() {
        let input_text = "[x] Ein kleines TODO".to_string();
        let result = parse_text_content(&input_text);

        assert_eq!(result.len(), 2);
        let element = result.get(0).unwrap();
        assert_eq!(element.payload, "x");
        assert_eq!(element.block_token_type, BlockTokenType::TODO);

        let element = result.get(1).unwrap();
        assert_eq!(element.payload, "Ein kleines TODO");
        assert_eq!(element.block_token_type, BlockTokenType::TEXT);
    }
}
