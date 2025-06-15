use crate::io::http::page::templates::dtos::{TemplateDto, TemplatesDto};
use crate::looksyk::templates::list::Templates;

pub fn map_templates(templates: Templates) -> TemplatesDto {
    let mapped_templates: Vec<_> = templates
        .templates
        .into_iter()
        .map(|template| TemplateDto {
            title: template.title.title,
            id: template.id.id,
        })
        .collect();

    TemplatesDto {
        templates: mapped_templates,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::looksyk::templates::list::{Template, TemplateId, TemplateTitle};

    #[test]
    fn test_map_templates() {
        let templates = Templates {
            templates: vec![
                Template {
                    title: TemplateTitle {
                        title: "Test Template 1".to_string(),
                    },
                    id: TemplateId {
                        id: "template-1".to_string(),
                    },
                },
                Template {
                    title: TemplateTitle {
                        title: "Test Template 2".to_string(),
                    },
                    id: TemplateId {
                        id: "template-2".to_string(),
                    },
                },
            ],
        };

        let mapped = map_templates(templates);
        assert_eq!(mapped.templates.len(), 2);
        assert_eq!(mapped.templates[0].title, "Test Template 1");
        assert_eq!(mapped.templates[0].id, "template-1");
        assert_eq!(mapped.templates[1].title, "Test Template 2");
        assert_eq!(mapped.templates[1].id, "template-2");
    }
}
