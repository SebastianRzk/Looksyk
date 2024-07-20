use crate::looksyk::model::{BlockToken, BlockTokenType, PageId, SimplePageName};
use crate::looksyk::page_index::{append_journal_page_prefix, append_user_page_prefix};

pub fn text_token(text: &str) -> BlockToken {
    BlockToken {
        block_token_type: BlockTokenType::TEXT,
        payload: text.to_string(),
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
        block_token_type: BlockTokenType::LINK,
    }
}

#[cfg(test)]
pub fn journal_link_token(link: &str) -> BlockToken {
    BlockToken {
        payload: link.to_string(),
        block_token_type: BlockTokenType::JOURNAL_LINK,
    }
}

pub fn page_name(name: String) -> SimplePageName {
    SimplePageName {
        name,
    }
}

#[cfg(test)]
pub fn page_id(id: &str) -> PageId {
    PageId {
        id: id.to_string(),
    }
}


#[cfg(test)]
pub fn user_page_id(id: &str) -> PageId {
    append_user_page_prefix(&page_name(id.to_string()))
}

#[cfg(test)]
pub fn journal_page_id(id: &str) -> PageId {
    append_journal_page_prefix(&page_name(id.to_string()))
}

#[cfg(test)]
pub fn todo_token() -> BlockToken {
    BlockToken {
        block_token_type: BlockTokenType::TODO,
        payload: " ".to_string(),
    }
}

#[cfg(test)]
pub fn done_token() -> BlockToken {
    BlockToken {
        block_token_type: BlockTokenType::TODO,
        payload: "x".to_string(),
    }
}

#[cfg(test)]
pub fn any_text_token() -> BlockToken {
    BlockToken {
        block_token_type: BlockTokenType::TEXT,
        payload: "my todo".to_string(),
    }
}

#[cfg(test)]
pub fn any_page_name() -> SimplePageName {
    page_name_str("")
}

#[cfg(test)]
pub fn any_page_id()  -> PageId {
    page_id("")
}
