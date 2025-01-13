use crate::looksyk::model::{
    BlockContent, PageId, PageType, ParsedBlock, ParsedMarkdownFile, SimplePageName,
};
use crate::looksyk::parser::parse_text_content;
use crate::looksyk::syntax::looksyk_markdown::render_as_tag;
use crate::state::state::{CurrentPageOnDiskState, NewPageOnDiskState};
use crate::state::tag::TagIndex;
use crate::state::userpage::UserPageIndex;
use std::collections::HashSet;

pub struct RenameTagResult {
    pub new_page_associated_state: NewPageOnDiskState,
    pub file_changes: FileChanges,
}

pub struct FileChanges {
    pub changed_files: HashSet<PageId>,
    pub file_to_delete: HashSet<PageId>,
}

impl FileChanges {
    pub fn append(&self, other: FileChanges) -> FileChanges {
        FileChanges {
            changed_files: self
                .changed_files
                .union(&other.changed_files)
                .cloned()
                .collect(),
            file_to_delete: self
                .file_to_delete
                .union(&other.file_to_delete)
                .cloned()
                .collect(),
        }
    }
}

impl RenameTagResult {
    pub fn append(&self, new_state: RenameTagResult) -> RenameTagResult {
        RenameTagResult {
            new_page_associated_state: new_state.new_page_associated_state,
            file_changes: self.file_changes.append(new_state.file_changes),
        }
    }
}

pub struct OldPageName {
    pub page_name: SimplePageName,
}

pub struct NewPageName {
    pub page_name: SimplePageName,
}

pub fn rename_page_across_all_files(
    old_page_name: OldPageName,
    new_page_name: NewPageName,
    current_page_associated_state: CurrentPageOnDiskState,
    tag_index: &TagIndex,
) -> RenameTagResult {
    let state_after_rename_in_file = rename_tag_across_all_files(
        &render_as_tag(&old_page_name.page_name),
        &render_as_tag(&new_page_name.page_name),
        &old_page_name,
        tag_index,
        current_page_associated_state,
    );
    let new_state = combine_two_pages(
        &old_page_name,
        &new_page_name,
        &state_after_rename_in_file.new_page_associated_state,
    );
    state_after_rename_in_file.append(new_state)
}

fn rename_tag_across_all_files(
    old: &String,
    new: &String,
    old_page_name: &OldPageName,
    tag_index: &TagIndex,
    current_page_associated_state: CurrentPageOnDiskState,
) -> RenameTagResult {
    let mut changed_files: HashSet<PageId> = HashSet::new();
    let mut new_user_pages = current_page_associated_state.user_pages.clone();
    let mut new_journal_pages = current_page_associated_state.journal_pages.clone();

    let references = tag_index
        .entries
        .get(&old_page_name.page_name.as_user_page());

    if let Some(r) = references {
        for reference in r {
            match reference.page_type {
                PageType::UserPage => {
                    let file = new_user_pages.entries.get(&reference.name).unwrap();
                    let new_file = rename_tag_in_file(old, new, file);
                    new_user_pages
                        .entries
                        .insert(reference.name.clone(), new_file);
                    changed_files.insert(reference.clone());
                }
                PageType::JournalPage => {
                    let file = new_journal_pages.entries.get(&reference.name).unwrap();
                    let new_file = rename_tag_in_file(old, new, file);
                    new_journal_pages
                        .entries
                        .insert(reference.name.clone(), new_file);
                    changed_files.insert(reference.clone());
                }
            }
        }
    }

    RenameTagResult {
        new_page_associated_state: NewPageOnDiskState {
            user_pages: new_user_pages,
            journal_pages: new_journal_pages,
        },
        file_changes: FileChanges {
            changed_files,
            file_to_delete: HashSet::new(),
        },
    }
}

fn rename_tag_in_file(
    old: &String,
    new: &String,
    parsed_markdown_file: &ParsedMarkdownFile,
) -> ParsedMarkdownFile {
    let mut new_blocks = vec![];
    for block in &parsed_markdown_file.blocks {
        let new_block = rename_tag_in_block(old, new, block);
        new_blocks.push(new_block);
    }
    ParsedMarkdownFile { blocks: new_blocks }
}

fn rename_tag_in_block(old: &String, new: &String, parsed_block: &ParsedBlock) -> ParsedBlock {
    let mut new_content = vec![];
    for line in &parsed_block.content {
        let new_line = rename_tag_in_line(old, new, line);
        new_content.push(new_line);
    }
    ParsedBlock {
        indentation: parsed_block.indentation,
        content: new_content,
    }
}

fn rename_tag_in_line(old: &String, new: &String, p2: &BlockContent) -> BlockContent {
    let new_text = p2.as_text.replace(old, new);

    BlockContent {
        as_tokens: parse_text_content(&new_text),
        as_text: new_text,
    }
}

fn combine_two_pages(
    old_tag_name: &OldPageName,
    new_tag_name: &NewPageName,
    new_page_associated_state: &NewPageOnDiskState,
) -> RenameTagResult {
    let target = new_page_associated_state
        .user_pages
        .entries
        .get(&new_tag_name.page_name);
    let source = new_page_associated_state
        .user_pages
        .entries
        .get(&old_tag_name.page_name);

    if source.is_none() {
        return RenameTagResult {
            new_page_associated_state: new_page_associated_state.clone(),
            file_changes: FileChanges {
                changed_files: HashSet::new(),
                file_to_delete: HashSet::new(),
            },
        };
    }
    let mut new_user_state = new_page_associated_state.user_pages.entries.clone();
    if target.is_none() {
        new_user_state.insert(new_tag_name.page_name.clone(), source.unwrap().clone());
    } else {
        let target = target.unwrap();
        let source = source.unwrap();
        let mut new_blocks = target.blocks.clone();
        new_blocks.append(&mut source.blocks.clone());
        new_user_state.insert(
            new_tag_name.page_name.clone(),
            ParsedMarkdownFile { blocks: new_blocks },
        );
    }

    RenameTagResult {
        new_page_associated_state: NewPageOnDiskState {
            user_pages: UserPageIndex {
                entries: new_user_state,
            },
            journal_pages: new_page_associated_state.journal_pages.clone(),
        },
        file_changes: FileChanges {
            changed_files: HashSet::from([new_tag_name.page_name.as_user_page()]),
            file_to_delete: HashSet::from([old_tag_name.page_name.as_user_page()]),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::looksyk::builder::page_name_str;
    use crate::looksyk::index::rename::{rename_page_across_all_files, NewPageName, OldPageName};
    use crate::looksyk::model::builder::block_with_text_content;
    use crate::looksyk::model::{PageId, ParsedMarkdownFile, SimplePageName};
    use crate::state::journal::JournalPageIndex;
    use crate::state::state::CurrentPageOnDiskState;
    use crate::state::tag::TagIndex;
    use crate::state::userpage::UserPageIndex;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn should_rename_references_and_merge_files_when_files_are_set() {
        let old_page_name = page_name_str("old_page");
        let new_page_name = page_name_str("new_page");
        let referencing_page_name_journal = page_name_str("referencing_page_journal");
        let referencing_page_name_user = page_name_str("referencing_page_user");

        let mut user_pages: HashMap<SimplePageName, ParsedMarkdownFile> = HashMap::new();
        user_pages.insert(
            old_page_name.clone(),
            ParsedMarkdownFile {
                blocks: vec![block_with_text_content("old page content")],
            },
        );
        user_pages.insert(
            new_page_name.clone(),
            ParsedMarkdownFile {
                blocks: vec![block_with_text_content("new page content")],
            },
        );
        user_pages.insert(
            referencing_page_name_user.clone(),
            ParsedMarkdownFile {
                blocks: vec![block_with_text_content(
                    "Here is a link for users [[old_page]]",
                )],
            },
        );

        let mut journal_pages: HashMap<SimplePageName, ParsedMarkdownFile> = HashMap::new();
        journal_pages.insert(
            referencing_page_name_journal.clone(),
            ParsedMarkdownFile {
                blocks: vec![block_with_text_content(
                    "Here is a link for journal [[old_page]]",
                )],
            },
        );

        let mut tags_index: HashMap<PageId, HashSet<PageId>> = HashMap::new();
        let mut tag_list: HashSet<PageId> = HashSet::new();
        tag_list.insert(referencing_page_name_user.as_user_page());
        tag_list.insert(referencing_page_name_journal.as_journal_page());
        tags_index.insert(old_page_name.as_user_page(), tag_list);

        let result = rename_page_across_all_files(
            OldPageName {
                page_name: old_page_name.clone(),
            },
            NewPageName {
                page_name: new_page_name.clone(),
            },
            CurrentPageOnDiskState {
                user_pages: &UserPageIndex {
                    entries: user_pages,
                },
                journal_pages: &JournalPageIndex {
                    entries: journal_pages,
                },
            },
            &TagIndex {
                entries: tags_index,
            },
        );

        assert_eq!(result.file_changes.file_to_delete.len(), 1);
        assert_eq!(
            result
                .file_changes
                .file_to_delete
                .contains(&old_page_name.as_user_page()),
            true
        );
        assert_eq!(result.file_changes.changed_files.len(), 3);
        assert_eq!(
            result
                .file_changes
                .changed_files
                .contains(&new_page_name.as_user_page()),
            true
        );
        assert_eq!(
            result
                .file_changes
                .changed_files
                .contains(&referencing_page_name_user.as_user_page()),
            true
        );
        assert_eq!(
            result
                .file_changes
                .changed_files
                .contains(&referencing_page_name_journal.as_journal_page()),
            true
        );
        let new_user_pages = result.new_page_associated_state.user_pages.entries;
        assert_eq!(new_user_pages.contains_key(&old_page_name), true);
        let new_page = new_user_pages.get(&new_page_name).unwrap();
        assert_eq!(new_page.blocks.len(), 2);
        assert_eq!(new_page.blocks[0].content[0].as_text, "new page content");
        assert_eq!(new_page.blocks[1].content[0].as_text, "old page content");

        let referencing_page = new_user_pages.get(&referencing_page_name_user).unwrap();
        assert_eq!(
            referencing_page.blocks[0].content[0].as_text,
            "Here is a link for users [[new_page]]"
        );
    }

    #[test]
    fn rename_should_not_throw_error_on_empty_state() {
        let result = rename_page_across_all_files(
            OldPageName {
                page_name: page_name_str("old"),
            },
            NewPageName {
                page_name: page_name_str("new"),
            },
            CurrentPageOnDiskState {
                user_pages: &UserPageIndex {
                    entries: HashMap::new(),
                },
                journal_pages: &JournalPageIndex {
                    entries: HashMap::new(),
                },
            },
            &TagIndex {
                entries: HashMap::new(),
            },
        );
        assert_eq!(result.file_changes.file_to_delete.len(), 0);
        assert_eq!(result.file_changes.changed_files.len(), 0);
    }
}
