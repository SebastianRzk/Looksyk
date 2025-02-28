use crate::looksyk::model::{BlockToken, BlockTokenType, SimplePageName};

#[cfg(test)]
pub mod test_builder {
    use crate::looksyk::datatypes::AssetDescriptor;
    use crate::state::application_state::DataRootLocation;
    use std::path::PathBuf;

    pub fn asset_descriptor(file_name: &str) -> AssetDescriptor {
        AssetDescriptor::new(file_name.to_string())
    }

    pub fn data_root_location(location: &str) -> DataRootLocation {
        DataRootLocation {
            path: PathBuf::from(location),
        }
    }
}

pub fn text_token_str(text: &str) -> BlockToken {
    BlockToken {
        block_token_type: BlockTokenType::Text,
        payload: text.to_string(),
    }
}

pub fn text_token(payload: String) -> BlockToken {
    BlockToken {
        block_token_type: BlockTokenType::Text,
        payload,
    }
}

#[cfg(test)]
pub fn page_name_str(name: &str) -> SimplePageName {
    SimplePageName {
        name: name.to_string(),
    }
}

#[cfg(test)]
pub fn link_token(link: &str) -> BlockToken {
    BlockToken {
        payload: link.to_string(),
        block_token_type: BlockTokenType::Link,
    }
}

#[cfg(test)]
pub fn journal_link_token(link: &str) -> BlockToken {
    BlockToken {
        payload: link.to_string(),
        block_token_type: BlockTokenType::JournalLink,
    }
}

pub fn page_name(name: String) -> SimplePageName {
    SimplePageName { name }
}

#[cfg(test)]
pub mod builder {
    use crate::looksyk::builder::page_name_str;
    use crate::looksyk::model::{BlockToken, BlockTokenType, PageId};
    use crate::state::journal::JournalPageIndex;

    pub fn user_page_id(id: &str) -> PageId {
        page_name_str(id).as_user_page()
    }

    pub fn journal_page_id(id: &str) -> PageId {
        page_name_str(id).as_journal_page()
    }

    pub fn todo_token() -> BlockToken {
        BlockToken {
            block_token_type: BlockTokenType::Todo,
            payload: " ".to_string(),
        }
    }
    pub fn done_token() -> BlockToken {
        BlockToken {
            block_token_type: BlockTokenType::Todo,
            payload: "x".to_string(),
        }
    }
    pub fn any_text_token() -> BlockToken {
        BlockToken {
            block_token_type: BlockTokenType::Text,
            payload: "my todo".to_string(),
        }
    }
    pub fn any_page_id() -> PageId {
        user_page_id("")
    }
    pub fn empty_journal_index() -> JournalPageIndex {
        JournalPageIndex {
            entries: std::collections::HashMap::new(),
        }
    }
}
