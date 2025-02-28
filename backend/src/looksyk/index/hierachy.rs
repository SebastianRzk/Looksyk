use crate::looksyk::builder::page_name;
use crate::looksyk::model::{BlockTokenType, ParsedBlock, SimplePageName};

pub struct HierarchyParser {
    pub page_name: SimplePageName,
    pub current_hierarchy: Vec<HierarchyEntry>,
}

pub struct HierarchyEntry {
    depth: usize,
    tag_list: Vec<SimplePageName>,
}

impl HierarchyParser {
    pub fn feed(&mut self, block: &ParsedBlock) {
        while self.last_depth() >= block.indentation && !self.current_hierarchy.is_empty() {
            self.current_hierarchy.pop();
        }

        let mut tags = vec![];
        for content in &block.content {
            for token in &content.as_tokens {
                if token.block_token_type == BlockTokenType::Link {
                    tags.push(page_name(token.payload.clone()));
                }
            }
        }
        self.current_hierarchy.push(HierarchyEntry {
            depth: block.indentation,
            tag_list: tags,
        });
    }

    fn last_depth(&self) -> usize {
        self.current_hierarchy
            .iter()
            .last()
            .map(|x| x.depth)
            .unwrap_or(0)
    }

    pub fn get_current_tag_set(&self) -> Vec<SimplePageName> {
        if self.current_hierarchy.is_empty() {
            return vec![self.page_name.clone()];
        }

        let mut result = vec![self.page_name.clone()];
        for entry in &self.current_hierarchy {
            for tag in &entry.tag_list {
                result.push(tag.clone());
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::looksyk::builder::page_name_str;
    use crate::looksyk::index::hierachy::HierarchyParser;
    use crate::looksyk::model::{BlockContent, BlockToken, BlockTokenType, ParsedBlock};

    #[test]
    fn empty_parser_should_return_empty_taglist_only_pagename() {
        let cut = HierarchyParser {
            page_name: page_name_str("MyPage"),
            current_hierarchy: vec![],
        };

        assert_eq!(cut.get_current_tag_set(), vec![page_name_str("MyPage")])
    }

    #[test]
    fn should_return_tags_when_tag_line_feeded() {
        let mut cut = HierarchyParser {
            page_name: page_name_str("MyPage"),
            current_hierarchy: vec![],
        };

        cut.feed(&ParsedBlock {
            indentation: 0,
            content: vec![BlockContent {
                as_tokens: vec![BlockToken {
                    payload: "mytag".to_string(),
                    block_token_type: BlockTokenType::Link,
                }],
                as_text: "".to_string(),
            }],
        });

        assert_eq!(
            cut.get_current_tag_set(),
            vec![page_name_str("MyPage"), page_name_str("mytag")]
        )
    }

    #[test]
    fn should_union_tags_over_nested_blocks() {
        let mut cut = HierarchyParser {
            page_name: page_name_str("MyPage"),
            current_hierarchy: vec![],
        };

        cut.feed(&ParsedBlock {
            indentation: 0,
            content: vec![BlockContent {
                as_tokens: vec![BlockToken {
                    payload: "mytag".to_string(),
                    block_token_type: BlockTokenType::Link,
                }],
                as_text: "".to_string(),
            }],
        });

        cut.feed(&ParsedBlock {
            indentation: 1,
            content: vec![BlockContent {
                as_tokens: vec![BlockToken {
                    payload: "mytag2".to_string(),
                    block_token_type: BlockTokenType::Link,
                }],
                as_text: "".to_string(),
            }],
        });

        assert_eq!(
            cut.get_current_tag_set(),
            vec![
                page_name_str("MyPage"),
                page_name_str("mytag"),
                page_name_str("mytag2")
            ]
        )
    }

    #[test]
    fn should_reset_on_same_level() {
        let mut cut = HierarchyParser {
            page_name: page_name_str("MyPage"),
            current_hierarchy: vec![],
        };

        cut.feed(&ParsedBlock {
            indentation: 0,
            content: vec![BlockContent {
                as_tokens: vec![BlockToken {
                    payload: "mytag".to_string(),
                    block_token_type: BlockTokenType::Link,
                }],
                as_text: "".to_string(),
            }],
        });

        cut.feed(&ParsedBlock {
            indentation: 1,
            content: vec![BlockContent {
                as_tokens: vec![BlockToken {
                    payload: "mytag2".to_string(),
                    block_token_type: BlockTokenType::Link,
                }],
                as_text: "".to_string(),
            }],
        });

        cut.feed(&ParsedBlock {
            indentation: 1,
            content: vec![BlockContent {
                as_tokens: vec![BlockToken {
                    payload: "mytag3".to_string(),
                    block_token_type: BlockTokenType::Link,
                }],
                as_text: "".to_string(),
            }],
        });

        assert_eq!(
            cut.get_current_tag_set(),
            vec![
                page_name_str("MyPage"),
                page_name_str("mytag"),
                page_name_str("mytag3")
            ]
        )
    }
}
