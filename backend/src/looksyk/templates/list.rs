use crate::looksyk::model::SimplePageName;
use crate::state::userpage::UserPageIndex;

pub struct Templates {
    pub templates: Vec<Template>,
}

pub struct Template {
    pub title: TemplateTitle,
    pub id: TemplateId,
}

pub struct TemplateTitle {
    pub title: String,
}

#[derive(Clone)]
pub struct TemplateId {
    pub id: String,
}

const TEMPLATE_PREFIX: &str = "Template /";

impl SimplePageName {
    pub fn is_template(&self) -> bool {
        self.name.starts_with(TEMPLATE_PREFIX)
    }
}

impl From<TemplateId> for SimplePageName {
    fn from(val: TemplateId) -> Self {
        SimplePageName { name: val.id }
    }
}

pub fn list_all_templates(user_page_index: &UserPageIndex) -> Templates {
    let mut result = vec![];

    for name in user_page_index.entries.keys() {
        if name.is_template() {
            result.push(Template {
                title: calculate_template_title(name),
                id: TemplateId {
                    id: name.name.clone(),
                },
            });
        }
    }
    result.sort_by(|a, b| a.title.title.cmp(&b.title.title));
    Templates { templates: result }
}

fn calculate_template_title(name: &SimplePageName) -> TemplateTitle {
    let title = name
        .name
        .strip_prefix(TEMPLATE_PREFIX)
        .unwrap()
        .trim()
        .to_string();
    TemplateTitle { title }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::looksyk::builder::page_name_str;
    use crate::looksyk::builder::test_builder::any_parsed_markdown_file;
    use crate::state::userpage::UserPageIndex;

    #[test]
    fn should_list_templates() {
        let mut entries = UserPageIndex {
            entries: std::collections::HashMap::new(),
        };
        entries.entries.insert(
            page_name_str("Template /Test Template"),
            any_parsed_markdown_file(),
        );

        let templates = list_all_templates(&entries);
        assert_eq!(templates.templates.len(), 1);
        assert_eq!(templates.templates[0].title.title, "Test Template");
    }

    #[test]
    fn should_not_list_non_template_pages() {
        let mut entries = UserPageIndex {
            entries: std::collections::HashMap::new(),
        };
        entries
            .entries
            .insert(page_name_str("Not a Template"), any_parsed_markdown_file());

        let templates = list_all_templates(&entries);
        assert!(templates.templates.is_empty());
    }

    #[test]
    fn should_recognize_template_names() {
        let template_name = SimplePageName {
            name: "Template /My Template".to_string(),
        };
        assert!(template_name.is_template());

        let non_template_name = SimplePageName {
            name: "My Page".to_string(),
        };
        assert!(!non_template_name.is_template());
    }

    #[test]
    fn should_calculate_template_title() {
        let name = SimplePageName {
            name: "Template /My Template".to_string(),
        };
        let title = calculate_template_title(&name);
        assert_eq!(title.title, "My Template");
    }

    #[test]
    fn should_calculate_template_title_with_whitespace() {
        let name = SimplePageName {
            name: "Template /  My Template  ".to_string(),
        };
        let title = calculate_template_title(&name);
        assert_eq!(title.title, "My Template");
    }
}
