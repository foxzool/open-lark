use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestPaginationResponse {
    pub items: Vec<String>,
    pub has_more: bool,
    pub page_token: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::validation::{
        pagination::PaginationRequestBuilder,
        ValidationResult,
    };

    #[test]
    fn test_pagination_builder_valid_params() {
        let params = PaginationRequestBuilder::<TestPaginationResponse>::new()
            .with_page_size(50)
            .with_page_token("valid_token")
            .build()
            .unwrap();

        assert_eq!(params.get("page_size"), Some(&"50".to_string()));
        assert_eq!(params.get("page_token"), Some(&"valid_token".to_string()));
    }

    #[test]
    fn test_pagination_builder_min_page_size() {
        let params = PaginationRequestBuilder::<TestPaginationResponse>::new()
            .with_page_size(1)
            .build()
            .unwrap();

        assert_eq!(params.get("page_size"), Some(&"1".to_string()));
    }

    #[test]
    fn test_pagination_builder_max_page_size() {
        let params = PaginationRequestBuilder::<TestPaginationResponse>::new()
            .with_page_size(500)
            .build()
            .unwrap();

        assert_eq!(params.get("page_size"), Some(&"500".to_string()));
    }

    #[test]
    fn test_pagination_builder_empty_token() {
        let params = PaginationRequestBuilder::<TestPaginationResponse>::new()
            .with_page_size(20)
            .with_page_token("")
            .build()
            .unwrap();

        // Empty token should not be added
        assert_eq!(params.get("page_token"), None);
    }

    #[test]
    fn test_pagination_builder_no_params() {
        let params = PaginationRequestBuilder::<TestPaginationResponse>::new()
            .build()
            .unwrap();

        assert_eq!(params.len(), 0);
    }

    #[test]
    fn test_pagination_builder_reset_token() {
        let params = PaginationRequestBuilder::<TestPaginationResponse>::new()
            .with_page_size(30)
            .with_page_token("some_token")
            .reset_page_token()
            .build()
            .unwrap();

        assert_eq!(params.get("page_size"), Some(&"30".to_string()));
        assert_eq!(params.get("page_token"), None);
    }

    #[test]
    fn test_validate_page_size_too_small() {
        let result = PaginationRequestBuilder::<TestPaginationResponse>::new()
            .with_page_size(0)
            .build();

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("page_size"));
    }

    #[test]
    fn test_validate_page_size_too_large() {
        let result = PaginationRequestBuilder::<TestPaginationResponse>::new()
            .with_page_size(501)
            .build();

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("page_size"));
    }

    #[test]
    fn test_validate_page_token_too_long() {
        let long_token = "a".repeat(1025);
        let result = PaginationRequestBuilder::<TestPaginationResponse>::new()
            .with_page_token(&long_token)
            .build();

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("page_token"));
    }

    #[test]
    fn test_validate_page_token_invalid_base64() {
        let invalid_token = "invalid_base64_token!@#$%";
        let result = PaginationRequestBuilder::<TestPaginationResponse>::new()
            .with_page_token(invalid_token)
            .build();

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("page_token"));
    }

    #[test]
    fn test_paginated_response_wrapper() {
        use crate::core::validation::pagination::PaginatedResponse;

        let response = TestPaginationResponse {
            items: vec!["item1".to_string(), "item2".to_string()],
            has_more: true,
            page_token: Some("next_token".to_string()),
        };

        let paginated = PaginatedResponse::new(response);

        assert_eq!(paginated.items().len(), 2);
        assert_eq!(paginated.has_more(), true);
        assert_eq!(paginated.page_token(), Some(&"next_token".to_string()));
    }

    #[test]
    fn test_paginated_response_no_more() {
        use crate::core::validation::pagination::PaginatedResponse;

        let response = TestPaginationResponse {
            items: vec!["item1".to_string()],
            has_more: false,
            page_token: None,
        };

        let paginated = PaginatedResponse::new(response);

        assert_eq!(paginated.items().len(), 1);
        assert_eq!(paginated.has_more(), false);
        assert_eq!(paginated.page_token(), None);
    }

    #[test]
    fn test_validation_result_traits() {
        // Test Valid
        let valid = ValidationResult::Valid;
        assert!(!valid.is_error());

        // Test Warning
        let warning = ValidationResult::Warning("This is a warning".to_string());
        assert!(!warning.is_error());
        assert_eq!(warning.message(), Some("This is a warning"));

        // Test Invalid
        let invalid = ValidationResult::Invalid("This is an error".to_string());
        assert!(invalid.is_error());
        assert_eq!(invalid.message(), Some("This is an error"));
    }
}