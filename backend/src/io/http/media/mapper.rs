use crate::io::http::media::dtos::{AssetPreviewDto, AssetPropertiesDto, SuggestionDto, SuggestionsDto};
use crate::looksyk::media::asset_preview::{AssetPreview, AssetProperties};
use crate::looksyk::media::suggestion::{Suggestion, Suggestions};

pub fn map_to_dto(suggestions: Suggestions) -> SuggestionsDto {
    SuggestionsDto {
        suggestions: suggestions.suggestions.iter().map(|x| map_to_suggestion_dto(x)).collect()
    }
}


fn map_to_suggestion_dto(suggestion: &Suggestion) -> SuggestionDto {
    SuggestionDto {
        explanation: suggestion.explanation.clone(),
        inplace_markdown: suggestion.inplace_markdown.clone(),
    }
}

pub fn map_to_asset_preview_dto(asset_preview: AssetPreview) -> AssetPreviewDto {
    AssetPreviewDto {
        markdown_preview: asset_preview.markdown_preview,
        html_preview_link: asset_preview.html_preview_link,
        properties: map_to_properties_dto(asset_preview.properties),
    }
}

fn map_to_properties_dto(asset_properties_dto: AssetProperties) -> AssetPropertiesDto {
    AssetPropertiesDto {
        size: asset_properties_dto.size,
        full_qualified_path: asset_properties_dto.full_qualified_path,
    }
}

#[cfg(test)]
mod tests {
    use crate::io::http::media::mapper::map_to_dto;
    use crate::looksyk::media::suggestion::{Suggestion, Suggestions};

    #[test]
    fn test_map_to_dto() {
        let suggestions = Suggestions {
            suggestions: vec![
                Suggestion {
                    explanation: "explanation".to_string(),
                    inplace_markdown: "inplace_markdown".to_string()
                }
            ]
        };

        let dto = map_to_dto(suggestions);

        assert_eq!(dto.suggestions.len(), 1);
        assert_eq!(dto.suggestions[0].explanation, "explanation");
        assert_eq!(dto.suggestions[0].inplace_markdown, "inplace_markdown");
    }
}