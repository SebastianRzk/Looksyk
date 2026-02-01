pub struct ParamValidator {
    errors: Vec<String>,
}

impl Default for ParamValidator {
    fn default() -> Self {
        ParamValidator { errors: Vec::new() }
    }
}

impl ParamValidator {
    pub fn validate_as_integer(mut self, param_value: &str, name: &str) -> Self {
        match param_value.parse::<i32>() {
            Ok(_) => self,
            Err(_) => {
                self.errors.push(format!(
                    "Parameter '{}' with value '{}' is not a valid integer.",
                    name, param_value
                ));
                self
            }
        }
    }

    pub fn validate_as_non_empty(mut self, param_value: &str, name: &str) -> Self {
        if param_value.trim().is_empty() {
            self.errors
                .push(format!("Parameter '{}' should not be empty.", name));
        }
        self
    }

    pub fn validate_as_date(mut self, param_value: &str, name: &str) -> Self {
        match chrono::NaiveDate::parse_from_str(param_value, "%Y-%m-%d") {
            Ok(_) => self,
            Err(_) => {
                self.errors.push(format!(
                    "Parameter '{}' with value '{}' is not a valid date (expected format: YYYY-MM-DD).",
                    name, param_value
                ));
                self
            }
        }
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn format_errors_as_markdown(&self) -> String {
        let mut markdown = String::from("**Parameter Validation Errors:**\n");
        for error in &self.errors {
            markdown.push_str(&format!("- {}\n", error));
        }
        markdown
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_param_validator_with_valid_date_should_pass() {
        let validator =
            super::ParamValidator::default().validate_as_date("2024-06-15", "startDate");
        assert!(validator.errors.is_empty());
    }

    #[test]
    fn test_param_validator_with_invalid_date_should_fail() {
        let validator =
            super::ParamValidator::default().validate_as_date("15-06-2024", "startDate");
        assert!(!validator.errors.is_empty());
        assert_eq!(validator.errors[0], "Parameter 'startDate' with value '15-06-2024' is not a valid date (expected format: YYYY-MM-DD).");
    }

    #[test]
    fn test_param_validator_with_empty_string_should_fail() {
        let validator = super::ParamValidator::default().validate_as_non_empty("   ", "username");
        assert!(!validator.errors.is_empty());
        assert_eq!(
            validator.errors[0],
            "Parameter 'username' should not be empty."
        );
    }

    #[test]
    fn test_param_validator_with_valid_integer_should_pass() {
        let validator = super::ParamValidator::default().validate_as_integer("42", "age");
        assert!(validator.errors.is_empty());
    }

    #[test]
    fn test_param_validator_with_invalid_integer_should_fail() {
        let validator = super::ParamValidator::default().validate_as_integer("forty-two", "age");
        assert!(!validator.errors.is_empty());
        assert_eq!(
            validator.errors[0],
            "Parameter 'age' with value 'forty-two' is not a valid integer."
        );
    }

    #[test]
    fn test_param_validator_format_errors_as_markdown() {
        let validator = super::ParamValidator::default()
            .validate_as_integer("not_an_integer", "param1")
            .validate_as_date("invalid_date", "param2")
            .validate_as_non_empty("   ", "param3");

        let markdown = validator.format_errors_as_markdown();
        let expected = "**Parameter Validation Errors:**\n- Parameter 'param1' with value 'not_an_integer' is not a valid integer.\n- Parameter 'param2' with value 'invalid_date' is not a valid date (expected format: YYYY-MM-DD).\n- Parameter 'param3' should not be empty.\n";

        assert_eq!(markdown, expected);
    }

    #[test]
    fn test_param_validator_has_errors() {
        let validator =
            super::ParamValidator::default().validate_as_integer("not_an_integer", "param1");
        assert!(validator.has_errors());

        let valid_validator = super::ParamValidator::default().validate_as_integer("123", "param2");
        assert!(!valid_validator.has_errors());
    }
}
