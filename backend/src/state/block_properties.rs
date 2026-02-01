use crate::looksyk::parser::BlockProperty;
use crate::state::block::BlockReference;
use std::collections::HashMap;

pub struct BlockPropertiesIndex {
    pub entries: HashMap<BlockPropertyKey, Vec<BlockPropertyOccurence>>,
}

impl Default for BlockPropertiesIndex {
    fn default() -> Self {
        BlockPropertiesIndex {
            entries: HashMap::new(),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct BlockPropertyKey {
    pub value: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BlockPropertyOccurence {
    pub value: BlockPropertyValue,
    pub block: BlockReference,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BlockPropertyValue {
    pub value: String,
}

impl BlockPropertiesIndex {
    pub fn get_all_keys(&self) -> Vec<BlockPropertyKey> {
        self.entries.keys().cloned().collect()
    }

    pub fn append_elements(&mut self, block_reference: BlockReference, property: BlockProperty) {
        let key = BlockPropertyKey {
            value: property.key,
        };

        if let Some(value) = self.entries.get_mut(&key) {
            value.push(BlockPropertyOccurence {
                value: BlockPropertyValue {
                    value: property.value,
                },
                block: block_reference.clone(),
            })
        } else {
            self.entries.insert(
                key,
                vec![BlockPropertyOccurence {
                    value: BlockPropertyValue {
                        value: property.value,
                    },
                    block: block_reference.clone(),
                }],
            );
        }
    }

    pub fn find(&self, key: &BlockPropertyKey) -> Option<&Vec<BlockPropertyOccurence>> {
        self.entries.get(key)
    }
}

#[cfg(test)]
pub mod builder {
    use crate::state::block::BlockReference;
    use crate::state::block_properties::{
        BlockPropertyKey, BlockPropertyOccurence, BlockPropertyValue,
    };

    pub fn block_property_key(value: &str) -> BlockPropertyKey {
        BlockPropertyKey {
            value: value.to_string(),
        }
    }

    pub fn block_property_occurance(
        value: &str,
        reference: BlockReference,
    ) -> BlockPropertyOccurence {
        BlockPropertyOccurence {
            value: block_property_value(value),
            block: reference,
        }
    }

    pub fn block_property_value(value: &str) -> BlockPropertyValue {
        BlockPropertyValue {
            value: value.to_string(),
        }
    }

    pub fn block_properties_index_with(
        key: BlockPropertyKey,
        occurences: Vec<BlockPropertyOccurence>,
    ) -> super::BlockPropertiesIndex {
        super::BlockPropertiesIndex {
            entries: vec![(key, occurences)].into_iter().collect(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::BlockPropertyKey;
    use crate::looksyk::builder::test_builder::user_page_id;
    use crate::state::block_properties::builder::{block_property_key, block_property_value};

    #[test]
    fn test_append_elements() {
        let mut index = super::BlockPropertiesIndex {
            entries: std::collections::HashMap::new(),
        };

        let block_ref = super::BlockReference {
            page_id: user_page_id("testabc"),
            block_number: 1,
        };

        index.append_elements(
            block_ref.clone(),
            crate::looksyk::parser::BlockProperty {
                key: "priority".to_string(),
                value: "high".to_string(),
            },
        );

        index.append_elements(
            block_ref,
            crate::looksyk::parser::BlockProperty {
                key: "status".to_string(),
                value: "open".to_string(),
            },
        );

        assert_eq!(index.entries.len(), 2);
        assert_eq!(
            index
                .entries
                .get(&BlockPropertyKey {
                    value: "priority".to_string()
                })
                .unwrap()[0]
                .value,
            block_property_value("high")
        );
        assert_eq!(
            index
                .entries
                .get(&BlockPropertyKey {
                    value: "status".to_string()
                })
                .unwrap()[0]
                .value,
            block_property_value("open")
        );
    }

    #[test]
    fn test_get_all_keys() {
        let mut index = super::BlockPropertiesIndex {
            entries: std::collections::HashMap::new(),
        };

        index.entries.insert(block_property_key("priority"), vec![]);

        index.entries.insert(block_property_key("status"), vec![]);

        let keys = index.get_all_keys();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&block_property_key("priority")));
        assert!(keys.contains(&block_property_key("status")));
    }
}
