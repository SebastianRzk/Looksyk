use crate::looksyk::model::SimplePageName;

pub fn render_as_tag(simple_page_name: &SimplePageName) -> String {
    format!("[[{}]]", simple_page_name.name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::looksyk::model::SimplePageName;

    #[test]
    fn test_render_as_tag_with_simple_page_name_should_render_tag() {
        let simple_page_name = SimplePageName {
            name: String::from("simple_page_name"),
        };

        let result = render_as_tag(&simple_page_name);

        assert_eq!(result, "[[simple_page_name]]");
    }
}
