use crate::looksyk::model::ReferencedMarkdown;
use crate::state::journal::JournalPageIndex;
use crate::state::tag::TagIndex;
use crate::state::todo::TodoIndex;
use crate::state::userpage::UserPageIndex;

pub struct RenderResult {
    pub inline_markdown: String,
    pub referenced_markdown: Vec<ReferencedMarkdown>,
    pub has_dynamic_content: bool,
}

pub struct StaticRenderContext<'a> {
    pub user_pages: &'a UserPageIndex,
    pub journal_pages: &'a JournalPageIndex,
    pub todo_index: &'a TodoIndex,
    pub tag_index: &'a TagIndex,
}

#[cfg(test)]
pub mod builder {
    use crate::looksyk::builder::test_builder::empty_journal_index;
    use crate::looksyk::renderer::model::StaticRenderContext;
    use crate::state::journal::JournalPageIndex;
    use crate::state::tag::builder::empty_tag_index;
    use crate::state::tag::TagIndex;
    use crate::state::todo::builder::empty_todo_index;
    use crate::state::todo::TodoIndex;
    use crate::state::userpage::builder::empty_user_page_index;
    use crate::state::userpage::UserPageIndex;

    pub struct TestRenderContext {
        pub user_pages: UserPageIndex,
        pub journal_pages: JournalPageIndex,
        pub todo_index: TodoIndex,
        pub tag_index: TagIndex,
    }

    impl TestRenderContext {
        pub fn to_static(&self) -> StaticRenderContext {
            StaticRenderContext {
                user_pages: &self.user_pages,
                todo_index: &self.todo_index,
                tag_index: &self.tag_index,
                journal_pages: &self.journal_pages,
            }
        }
    }

    pub fn create_render_context_with_user_page_index(
        user_page_index: UserPageIndex,
    ) -> TestRenderContext {
        TestRenderContext {
            user_pages: user_page_index,
            journal_pages: empty_journal_index(),
            todo_index: empty_todo_index(),
            tag_index: empty_tag_index(),
        }
    }

    pub fn create_render_context_with_todo_index(todo_index: TodoIndex) -> TestRenderContext {
        TestRenderContext {
            user_pages: empty_user_page_index(),
            journal_pages: empty_journal_index(),
            todo_index,
            tag_index: empty_tag_index(),
        }
    }

    pub fn create_render_context_with_tag_index(tag_index: TagIndex) -> TestRenderContext {
        TestRenderContext {
            user_pages: empty_user_page_index(),
            journal_pages: empty_journal_index(),
            todo_index: empty_todo_index(),
            tag_index,
        }
    }

    pub fn create_render_context(
        user_page_index: UserPageIndex,
        todo_index: TodoIndex,
        tag_index: TagIndex,
    ) -> TestRenderContext {
        TestRenderContext {
            user_pages: user_page_index,
            journal_pages: empty_journal_index(),
            todo_index,
            tag_index,
        }
    }

    pub fn create_empty_render_context() -> TestRenderContext {
        TestRenderContext {
            user_pages: empty_user_page_index(),
            journal_pages: empty_journal_index(),
            todo_index: empty_todo_index(),
            tag_index: empty_tag_index(),
        }
    }
}
