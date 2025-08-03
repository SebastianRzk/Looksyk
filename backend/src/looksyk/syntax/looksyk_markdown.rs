use crate::looksyk::model::{BlockToken, SimplePageName};

pub fn render_as_tag(simple_page_name: &SimplePageName) -> String {
    render_as_tag_str(&simple_page_name.name)
}

pub fn render_as_tag_str(simple_page_name: &str) -> String {
    format!("[[{simple_page_name}]]")
}

pub fn render_as_todo(token: &BlockToken) -> String {
    format!("[{}] ", token.payload)
}

pub fn render_as_todo_without_padding(payload: &BlockToken) -> String {
    format!("[{}]", payload.payload)
}

pub fn render_as_query(token: &BlockToken) -> String {
    format!("{{query: {} }}", token.payload)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::looksyk::model::{BlockTokenType, SimplePageName};

    #[test]
    fn test_render_as_tag_with_simple_page_name_should_render_tag() {
        let simple_page_name = SimplePageName {
            name: String::from("simple_page_name"),
        };

        let result = render_as_tag(&simple_page_name);

        assert_eq!(result, "[[simple_page_name]]");
    }

    #[test]
    fn test_render_as_tag_str_should_render_tag() {
        let simple_page_name = "simple_page_name";

        let result = render_as_tag_str(simple_page_name);

        assert_eq!(result, "[[simple_page_name]]");
    }

    #[test]
    fn test_render_as_todo_should_render_todo() {
        let token = BlockToken {
            payload: String::from("X"),
            block_token_type: BlockTokenType::Todo,
        };

        let result = render_as_todo(&token);

        assert_eq!(result, "[X] ");
    }

    #[test]
    fn test_render_as_query_should_render_query() {
        let token = BlockToken {
            payload: String::from("query_content"),
            block_token_type: BlockTokenType::Query,
        };

        let result = render_as_query(&token);

        assert_eq!(result, "{query: query_content }");
    }

    #[test]
    fn test_render_as_todo_without_padding_should_render_todo_without_padding() {
        let result = render_as_todo_without_padding(&BlockToken {
            payload: String::from("X"),
            block_token_type: BlockTokenType::Todo,
        });

        assert_eq!(result, "[X]");
    }
}
