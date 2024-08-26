use std::collections::HashMap;
use std::string::ToString;

use crate::looksyk::model::{BlockContent, BlockToken, BlockTokenType, ParsedBlock, ParsedMarkdownFile, RawBlock, RawMarkdownFile, UpdateMarkdownFile};
use crate::looksyk::model::BlockTokenType::TEXT;

#[derive(Clone, Debug)]
struct BlockTokenPattern {
    start_sequence: String,
    stop_sequence: String,
    block_token_type: BlockTokenType,
}

impl BlockTokenPattern {
    fn feed(&self, c: char, state: &MatcherState) -> MatcherState {
        let mut new_inner_text = state.inner_text.clone();
        let  new_index ;
        if !state.active {
            new_index = self.feed_pattern(c, self.start_sequence.clone(), state);
            if new_index == self.start_sequence.len() {
                return MatcherState {
                    inner_text: new_inner_text,
                    current_index: 0,
                    active: true,
                };
            }
        } else {
            new_index = self.feed_pattern(c, self.stop_sequence.clone(), state);
            new_inner_text.push(c);
            if new_index == self.stop_sequence.len() {
                let additional_chars_to_remove = self.stop_sequence.len() - 1;
                let length = state.inner_text.len() - additional_chars_to_remove;
                new_inner_text.truncate(length);
                return MatcherState {
                    inner_text: new_inner_text,
                    current_index: 0,
                    active: false,
                };
            }
        }
        MatcherState {
            inner_text: new_inner_text,
            active: state.active,
            current_index: new_index,
        }
    }

    fn feed_pattern(&self, c: char, pattern: String, state: &MatcherState) -> usize {
        if pattern.chars().nth(state.current_index).unwrap() == c {
            return state.current_index + 1;
        }
        return 0;
    }
}

fn generate_block_patterns() -> Vec<BlockTokenPattern> {
    vec![
        BlockTokenPattern {
            block_token_type: BlockTokenType::LINK,
            start_sequence: "[[".to_string(),
            stop_sequence: "]]".to_string(),
        },
        BlockTokenPattern {
            block_token_type: BlockTokenType::QUERY,
            start_sequence: "{query: ".to_string(),
            stop_sequence: " }".to_string()
        }
    ]
}


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
        indentation: raw_block.indentation
    }
}


pub fn parse_all_text_lines(all_lines: &Vec<String>) -> Vec<BlockContent> {
    let mut parsed_content = vec![];


    for line in all_lines {
        parsed_content.push(BlockContent {
            as_text: line.clone(),
            as_tokens: parse_text_content(line)
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

    let all_matchers = generate_block_patterns();
    let mut current_matcher: Option<BlockTokenPattern> = None;
    let mut current_text: Vec<char> = vec![];

    let mut matcher_state = create_initial_matcher_states();

    let mut remaining_text_content = text_content.clone();

    if text_content.starts_with("[ ] "){
        parsed_tokens.push(BlockToken{
            payload: " ".to_string(),
            block_token_type: BlockTokenType::TODO
        });
        remaining_text_content = remaining_text_content.strip_prefix("[ ] ").unwrap().to_string();
    }
    if text_content.starts_with("[x] "){
        parsed_tokens.push(BlockToken{
            payload: "x".to_string(),
            block_token_type: BlockTokenType::TODO
        });
        remaining_text_content = remaining_text_content.strip_prefix("[x] ").unwrap().to_string();
    }


    for char in remaining_text_content.chars() {
        if current_matcher.is_none() {
            for matcher in all_matchers.clone() {
                let token_type = &matcher.block_token_type;
                let state = matcher_state.get(&token_type).unwrap();

                let new_state = matcher.feed(char, state);
                if new_state.active {
                    let length = current_text.len() - (matcher.start_sequence.len() - 1);
                    current_text.truncate(length);
                    parsed_tokens.push(BlockToken {
                        payload: current_text.clone().into_iter().collect(),
                        block_token_type: TEXT,
                    });
                    current_text = vec![];
                    current_matcher = Some(matcher.clone());
                }
                matcher_state.insert(token_type.to_owned(), new_state);
            }
            if current_matcher.is_none() {
                current_text.push(char)
            }
        } else {
            let current_matcher_unwraped = current_matcher.clone().unwrap();
            let token_type = current_matcher_unwraped.block_token_type.clone();
            let state = matcher_state.get(&token_type).unwrap();

            let new_state = current_matcher_unwraped.feed(char, state);
            if !new_state.active {
                current_matcher = None;
                parsed_tokens.push(BlockToken {
                    payload: new_state.inner_text.clone().into_iter().collect(),
                    block_token_type: token_type.clone(),
                });
                matcher_state = create_initial_matcher_states()
            }else {
                matcher_state.insert(token_type, new_state);
            }
        }
    }
    if let Some(m) = current_matcher {
        let state = matcher_state.get(&m.block_token_type).unwrap();
        let inner_text: String = state.inner_text.clone().into_iter().collect();
        parsed_tokens.push(BlockToken {
            payload: format!("{}{}", m.start_sequence, inner_text),
            block_token_type: TEXT,
        });
    } else {
        parsed_tokens.push(BlockToken {
            payload: current_text.into_iter().collect(),
            block_token_type: TEXT,
        });
    }
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