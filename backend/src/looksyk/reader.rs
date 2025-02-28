use std::str::Lines;

use crate::looksyk::model::{RawBlock, RawMarkdownFile};

const INDENTATION: char = '\t';
const BULLET: char = '-';

pub fn calculate_indentation(string: &str) -> usize {
    let mut tab_count = 0;
    for char in string.chars() {
        if char != INDENTATION {
            return tab_count;
        }
        tab_count += 1;
    }
    tab_count
}

pub fn read_file_contents(file_content: String) -> RawMarkdownFile {
    let lines = file_content.lines();
    RawMarkdownFile {
        blocks: parse_lines(lines),
    }
}

pub fn parse_lines(lines: Lines) -> Vec<RawBlock> {
    let mut result = vec![];
    let mut current_block = vec![];
    let mut indentation = 0;
    for line in lines {
        let mut rest_of_line = line.trim_start_matches(INDENTATION);

        if rest_of_line.starts_with(BULLET) {
            if !current_block.is_empty() {
                result.push(RawBlock {
                    indentation,
                    text_content: current_block,
                });
                current_block = vec![];
            }
            indentation = calculate_indentation(line);
            rest_of_line = rest_of_line.strip_prefix(BULLET).unwrap().trim_start();
        }
        current_block.push(rest_of_line.to_string());
    }
    if !current_block.is_empty() {
        result.push(RawBlock {
            indentation,
            text_content: current_block,
        })
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_two_lines_as_blocks() {
        let result = read_file_contents("- line 1\n- line 2".to_string());
        assert_eq!(result.blocks.len(), 2);
        let block1 = result.blocks.first().unwrap();
        assert_eq!(block1.text_content, vec!["line 1"]);
        assert_eq!(block1.indentation, 0);
        let block2 = result.blocks.first().unwrap();
        assert_eq!(block2.text_content, vec!["line 1"]);
        assert_eq!(block2.indentation, 0);
    }

    #[test]
    fn should_parse_two_nested_blocks() {
        let result = read_file_contents("- parent\n\t- child".to_string());

        assert_eq!(result.blocks.len(), 2);
        let parent = result.blocks.first().unwrap();
        assert_eq!(parent.text_content, vec!["parent"]);
        assert_eq!(parent.indentation, 0);

        let child = result.blocks.get(1).unwrap();
        assert_eq!(child.text_content, vec!["child"]);
        assert_eq!(child.indentation, 1);
    }

    #[test]
    fn should_add_following_line_to_block() {
        let result = read_file_contents("- line1\nline2".to_string());
        assert_eq!(result.blocks.len(), 1);
        let block = result.blocks.first().unwrap();
        assert_eq!(block.text_content, vec!["line1", "line2"]);
    }
}
