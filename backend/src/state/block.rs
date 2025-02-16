use crate::looksyk::model::PageId;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BlockReference {
    pub page_id: PageId,
    pub block_number: usize,
}
