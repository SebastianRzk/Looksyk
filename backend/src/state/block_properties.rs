use crate::looksyk::parser::BlockProperty;
use crate::state::block::BlockReference;
use std::collections::HashMap;

pub struct BlockPropertiesIndex {
    pub entries: HashMap<BlockPropertyKey, Vec<BlockPropertyValue>>,
}

#[derive(Eq, PartialEq, Hash)]
pub struct BlockPropertyKey {
    pub value: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlockPropertyValue {
    pub value: String,
    pub block: BlockReference,
}

impl BlockPropertiesIndex {
    pub fn append_elements(&mut self, block_reference: BlockReference, property: BlockProperty) {
        let key = BlockPropertyKey {
            value: property.key,
        };

        if let Some(value) = self.entries.get_mut(&key) {
            value.push(BlockPropertyValue {
                value: property.value,
                block: block_reference.clone(),
            })
        } else {
            self.entries.insert(
                key,
                vec![BlockPropertyValue {
                    value: property.value,
                    block: block_reference.clone(),
                }],
            );
        }
    }
}

#[cfg(test)]
pub mod builder {
    use crate::looksyk::builder::test_builder::user_page_id;
    use crate::state::block::BlockReference;
    use crate::state::block_properties::{BlockPropertyKey, BlockPropertyValue};


    pub fn block_property_key(value: &str) -> BlockPropertyKey {
        BlockPropertyKey {
            value: value.to_string(),
        }
    }

    pub fn block_property_value(value: &str, reference: BlockReference) -> BlockPropertyValue {
        BlockPropertyValue {
            value: value.to_string(),
            block: reference,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::BlockPropertyKey;
    use crate::looksyk::builder::test_builder::user_page_id;

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
            "high"
        );
        assert_eq!(
            index
                .entries
                .get(&BlockPropertyKey {
                    value: "status".to_string()
                })
                .unwrap()[0]
                .value,
            "open"
        );
    }
}
