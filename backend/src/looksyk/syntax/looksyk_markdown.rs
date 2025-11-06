use crate::looksyk::model::{BlockToken, SimplePageName};
use crate::state::block_properties::{BlockPropertyKey, BlockPropertyValue};

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

pub fn serialize_property(key: &BlockPropertyKey, value: &BlockPropertyValue) -> String {
    format!("{}:: {}", key.value, value.value)
}

pub fn render_property(token: &BlockToken) -> String {
    format!("<code class=\"inline-property\">{}</code>", token.payload).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::looksyk::model::{BlockTokenType, SimplePageName};
    use crate::state::block_properties::builder::{block_property_key, block_property_value};

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

    #[test]
    fn test_serialize_property() {
        let result =
            serialize_property(&block_property_key("key1"), &block_property_value("value1"));

        assert_eq!(result, "key1:: value1");
    }

    #[test]
    fn test_render_property() {
        let result = render_property(&BlockToken {
            payload: "key:: value".to_string(),
            block_token_type: BlockTokenType::Property,
        });

        assert_eq!(result, "<code class=\"inline-property\">key:: value</code>");
    }
}
