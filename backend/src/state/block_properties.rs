use crate::looksyk::parser::BlockProperty;
use crate::state::block::BlockReference;
use std::collections::HashMap;

pub struct BlockPropertiesIndex {
    pub entries: HashMap<BlockPropertyKey, Vec<BlockPropertyOccurence>>,
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
}

#[cfg(test)]
pub mod tests {
    use super::BlockPropertyKey;
    use crate::looksyk::builder::test_builder::user_page_id;
    use crate::state::block_properties::builder::block_property_value;

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
}
