use std::marker::PhantomData;

use crate::core::{
    api_req::ApiRequest,
    error::LarkAPIError,
    validation::{self, ValidateBuilder, ValidationResult},
    SDKResult,
};

/// 通用分页响应结构
#[derive(Debug, Clone)]
pub struct GenericPageResponse<T> {
    /// 数据列表
    pub items: Vec<T>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 下一页的分页标记
    pub page_token: Option<String>,
}

/// 通用分页请求构建器
///
/// 提供链式调用来构建分页请求，并内置数据验证功能。
/// 支持设置分页大小、分页标记等参数，并自动验证参数的有效性。
///
/// # 使用示例
///
/// ```rust
/// use open_lark::core::validation::pagination::PaginationRequestBuilder;
///
/// let request = PaginationRequestBuilder::<()>::default()
///     .with_page_size(50)
///     .with_page_token("next_page_token")
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct PaginationRequestBuilder<T> {
    page_size: Option<u32>,
    page_token: Option<String>,
    _marker: PhantomData<T>,
}

impl<T> Default for PaginationRequestBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> PaginationRequestBuilder<T> {
    /// 创建新的分页请求构建器
    ///
    /// # 返回
    /// 一个空的分页请求构建器实例
    pub fn new() -> Self {
        Self {
            page_size: None,
            page_token: None,
            _marker: PhantomData,
        }
    }

    /// 设置分页大小
    ///
    /// # 参数
    /// - `page_size`: 分页大小，必须在 1-500 之间
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::core::validation::pagination::PaginationRequestBuilder;
    ///
    /// let params = PaginationRequestBuilder::<()>::default()
    ///     .with_page_size(50)
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(params.get("page_size"), Some(&"50".to_string()));
    /// ```
    pub fn with_page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    ///
    /// # 参数
    /// - `page_token`: 分页标记，用于获取下一页数据
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::core::validation::pagination::PaginationRequestBuilder;
    ///
    /// let params = PaginationRequestBuilder::<()>::default()
    ///     .with_page_token("next_page_token")
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(params.get("page_token"), Some(&"next_page_token".to_string()));
    /// ```
    pub fn with_page_token(mut self, page_token: impl ToString) -> Self {
        self.page_token = Some(page_token.to_string());
        self
    }

    /// 重置分页标记（用于获取第一页数据）
    pub fn reset_page_token(mut self) -> Self {
        self.page_token = None;
        self
    }

    /// 构建分页请求参数
    ///
    /// 对所有设置的字段进行验证，确保数据符合要求。
    /// 验证通过后返回包含分页参数的 HashMap，验证失败则返回错误。
    ///
    /// # 验证规则
    ///
    /// - 分页大小：1-500 之间，推荐 50
    /// - 分页标记：必须是有效的 Base64 字符串，长度不超过 1024
    pub fn build(self) -> SDKResult<std::collections::HashMap<&'static str, String>> {
        let mut params = std::collections::HashMap::new();

        // 1. 验证并添加分页大小
        if let Some(page_size) = self.page_size {
            match validation::validate_page_size(page_size, "page_size") {
                ValidationResult::Valid => {}
                ValidationResult::Warning(msg) => {
                    log::warn!("Page size validation warning: {}", msg);
                }
                ValidationResult::Invalid(msg) => {
                    return Err(LarkAPIError::illegal_param(format!(
                        "Invalid page size: {}",
                        msg
                    )));
                }
            }
            params.insert("page_size", page_size.to_string());
        }

        // 2. 验证并添加分页标记
        if let Some(ref page_token) = self.page_token {
            // Special check for empty page token
            if page_token.is_empty() {
                return Err(LarkAPIError::illegal_param("page token cannot be empty"));
            }

            match validation::validate_page_token(page_token, "page_token") {
                ValidationResult::Valid => {}
                ValidationResult::Warning(msg) => {
                    log::warn!("Page token validation warning: {}", msg);
                }
                ValidationResult::Invalid(msg) => {
                    return Err(LarkAPIError::illegal_param(format!(
                        "Invalid page token: {}",
                        msg
                    )));
                }
            }
            params.insert("page_token", page_token.clone());
        }

        Ok(params)
    }

    /// 构建分页请求参数并添加到现有的 ApiRequest
    pub fn build_to_api_request(self, mut api_req: ApiRequest) -> SDKResult<ApiRequest> {
        let params = self.build()?;
        for (key, value) in params {
            api_req.query_params.insert(key, value);
        }
        Ok(api_req)
    }

    /// 获取推荐的分页大小
    pub fn recommended_page_size() -> u32 {
        validation::pagination_limits::RECOMMENDED_PAGE_SIZE
    }

    /// 获取最大允许的分页大小
    pub fn max_page_size() -> u32 {
        validation::pagination_limits::MAX_PAGE_SIZE
    }

    /// 获取最小允许的分页大小
    pub fn min_page_size() -> u32 {
        validation::pagination_limits::MIN_PAGE_SIZE
    }
}

impl<T> ValidateBuilder for PaginationRequestBuilder<T> {
    fn validate(&self) -> ValidationResult {
        // 验证分页大小
        if let Some(page_size) = self.page_size {
            if let ValidationResult::Invalid(msg) =
                validation::validate_page_size(page_size, "page_size")
            {
                return ValidationResult::Invalid(msg);
            }
        }

        // 验证分页标记
        if let Some(ref page_token) = self.page_token {
            if let ValidationResult::Invalid(msg) =
                validation::validate_page_token(page_token, "page_token")
            {
                return ValidationResult::Invalid(msg);
            }
        }

        ValidationResult::Valid
    }
}

/// 分页响应包装器，提供便捷的分页处理方法
#[derive(Debug, Clone)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub has_more: bool,
    pub page_token: Option<String>,
    pub total_count: Option<u32>,
}

impl<T> PaginatedResponse<T> {
    /// 从 GenericPageResponse 创建 PaginatedResponse
    pub fn from_page_response(page_resp: GenericPageResponse<T>) -> Self {
        Self {
            items: page_resp.items,
            has_more: page_resp.has_more,
            page_token: page_resp.page_token,
            total_count: None,
        }
    }

    /// 从通用的分页字段创建 PaginatedResponse
    pub fn from_fields(items: Vec<T>, has_more: bool, page_token: Option<String>) -> Self {
        Self {
            items,
            has_more,
            page_token,
            total_count: None,
        }
    }

    /// 检查是否还有更多数据
    pub fn has_more(&self) -> bool {
        self.has_more
    }

    /// 获取下一页的分页标记
    pub fn next_page_token(&self) -> Option<&str> {
        self.page_token.as_deref()
    }

    /// 获取数据数量
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T> From<GenericPageResponse<T>> for PaginatedResponse<T> {
    fn from(page_resp: GenericPageResponse<T>) -> Self {
        Self::from_page_response(page_resp)
    }
}

/// 分页迭代器，用于自动遍历所有分页数据
pub struct PaginationIterator<'a, T, F> {
    service: &'a T,
    page_size: u32,
    current_page_token: Option<String>,
    has_more: bool,
    fetch_fn: F,
}

impl<'a, T, F> PaginationIterator<'a, T, F>
where
    F: Fn(&'a T, u32, Option<&str>) -> SDKResult<GenericPageResponse<T>>,
{
    /// 创建新的分页迭代器
    pub fn new(service: &'a T, page_size: u32, fetch_fn: F) -> Self {
        Self {
            service,
            page_size,
            current_page_token: None,
            has_more: true,
            fetch_fn,
        }
    }

    /// 获取下一页数据
    pub async fn next(&mut self) -> SDKResult<Option<Vec<T>>> {
        if !self.has_more {
            return Ok(None);
        }

        let page_resp = (self.fetch_fn)(
            self.service,
            self.page_size,
            self.current_page_token.as_deref(),
        )?;

        self.has_more = page_resp.has_more;
        self.current_page_token = page_resp.page_token.clone();

        Ok(Some(page_resp.items))
    }

    /// 收集所有分页数据（注意：这可能会获取大量数据）
    pub async fn collect_all(mut self) -> SDKResult<Vec<T>> {
        let mut all_items = Vec::new();

        while let Some(mut items) = self.next().await? {
            all_items.append(&mut items);
        }

        Ok(all_items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_builder_valid() {
        let params = PaginationRequestBuilder::<String>::new()
            .with_page_size(50)
            .with_page_token("valid_token")
            .build()
            .unwrap();

        assert_eq!(params.get("page_size"), Some(&"50".to_string()));
        assert_eq!(params.get("page_token"), Some(&"valid_token".to_string()));
    }

    #[test]
    fn test_pagination_builder_min_page_size() {
        let params = PaginationRequestBuilder::<String>::new()
            .with_page_size(1)
            .build()
            .unwrap();

        assert_eq!(params.get("page_size"), Some(&"1".to_string()));
    }

    #[test]
    fn test_pagination_builder_max_page_size() {
        let params = PaginationRequestBuilder::<String>::new()
            .with_page_size(500)
            .build()
            .unwrap();

        assert_eq!(params.get("page_size"), Some(&"500".to_string()));
    }

    #[test]
    fn test_pagination_builder_invalid_page_size_too_small() {
        let result = PaginationRequestBuilder::<String>::new()
            .with_page_size(0)
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            LarkAPIError::IllegalParamError(msg) => {
                assert!(msg.contains("page size must be at least"));
            }
            _ => panic!("Expected IllegalParamError"),
        }
    }

    #[test]
    fn test_pagination_builder_invalid_page_size_too_large() {
        let result = PaginationRequestBuilder::<String>::new()
            .with_page_size(501)
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            LarkAPIError::IllegalParamError(msg) => {
                assert!(msg.contains("page size must not exceed"));
            }
            _ => panic!("Expected IllegalParamError"),
        }
    }

    #[test]
    fn test_pagination_builder_empty_page_token() {
        let result = PaginationRequestBuilder::<String>::new()
            .with_page_token("")
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            LarkAPIError::IllegalParamError(msg) => {
                assert!(msg.contains("page token cannot be empty"));
            }
            _ => panic!("Expected IllegalParamError"),
        }
    }

    #[test]
    fn test_pagination_builder_long_page_token() {
        let long_token = "a".repeat(1025); // 1025 characters
        let result = PaginationRequestBuilder::<String>::new()
            .with_page_token(&long_token)
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            LarkAPIError::IllegalParamError(msg) => {
                assert!(msg.contains("page token must not exceed"));
            }
            _ => panic!("Expected IllegalParamError"),
        }
    }

    #[test]
    fn test_pagination_builder_reset_page_token() {
        let params = PaginationRequestBuilder::<String>::new()
            .with_page_token("some_token")
            .reset_page_token()
            .build()
            .unwrap();

        assert!(!params.contains_key("page_token"));
    }

    #[test]
    fn test_pagination_builder_without_page_size() {
        let params = PaginationRequestBuilder::<String>::new()
            .with_page_token("valid_token")
            .build()
            .unwrap();

        assert!(!params.contains_key("page_size"));
        assert_eq!(params.get("page_token"), Some(&"valid_token".to_string()));
    }

    #[test]
    fn test_pagination_builder_without_page_token() {
        let params = PaginationRequestBuilder::<String>::new()
            .with_page_size(50)
            .build()
            .unwrap();

        assert_eq!(params.get("page_size"), Some(&"50".to_string()));
        assert!(!params.contains_key("page_token"));
    }

    #[test]
    fn test_validation_trait_implementation() {
        let builder = PaginationRequestBuilder::<String>::new()
            .with_page_size(50)
            .with_page_token("valid_token");

        // Test ValidateBuilder trait implementation
        let result = builder.validate();
        match result {
            ValidationResult::Valid => {}
            _ => panic!("Expected valid validation result"),
        }

        // Test invalid case
        let builder = PaginationRequestBuilder::<String>::new().with_page_size(0);
        let result = builder.validate();
        match result {
            ValidationResult::Invalid(msg) => {
                assert!(msg.contains("page size must be at least"));
            }
            _ => panic!("Expected invalid validation result"),
        }
    }

    #[test]
    fn test_paginated_response() {
        let page_resp = GenericPageResponse {
            items: vec!["item1".to_string(), "item2".to_string()],
            has_more: true,
            page_token: Some("next_token".to_string()),
        };

        let paginated_resp = PaginatedResponse::from(page_resp);

        assert_eq!(paginated_resp.len(), 2);
        assert!(paginated_resp.has_more());
        assert_eq!(paginated_resp.next_page_token(), Some("next_token"));
        assert!(!paginated_resp.is_empty());
    }

    #[test]
    fn test_pagination_constants() {
        assert_eq!(
            PaginationRequestBuilder::<String>::recommended_page_size(),
            50
        );
        assert_eq!(PaginationRequestBuilder::<String>::max_page_size(), 500);
        assert_eq!(PaginationRequestBuilder::<String>::min_page_size(), 1);
    }
}
