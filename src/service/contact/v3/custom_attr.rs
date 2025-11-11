//! 获取企业自定义用户字段服务
//!
//! 提供企业自定义用户字段的查询功能：
//! - 查询企业自定义用户字段列表
//! - 支持分页查询
//! - 支持字段类型过滤

use crate::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};
// 使用本地的 CustomAttr 定义，避免依赖有问题的 models.rs

/// 自定义用户字段信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomAttr {
    /// 字段标识
    pub key: String,
    /// 字段名称
    pub name: String,
    /// 字段描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 字段类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 是否必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// 选项列表（枚举类型）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<CustomAttrOption>>,
}

/// 自定义字段选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomAttrOption {
    /// 选项名称
    pub name: String,
    /// 选项值
    pub value: String,
}

impl Default for CustomAttr {
    fn default() -> Self {
        Self {
            key: String::new(),
            name: String::new(),
            description: None,
            r#type: None,
            required: None,
            options: None,
        }
    }
}

/// 自定义用户字段服务
#[derive(Debug, Clone)]
pub struct CustomAttrService {
    config: Config,
}

impl CustomAttrService {
    /// 创建新的自定义用户字段服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取企业自定义用户字段
    ///
    /// 查询企业内所有自定义用户字段的列表
    ///
    /// # 参数
    /// * `req` - 查询自定义字段请求
    ///
    /// # 返回
    /// 返回自定义字段列表，支持分页
    pub async fn list(&self, req: &ListCustomAttrsRequest) -> SDKResult<ListCustomAttrsResponse> {
        let mut api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_CUSTOM_ATTRS.to_string();

        // 添加查询参数
        let mut query_params = Vec::new();
        if let Some(page_size) = &req.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &req.page_token {
            query_params.push(format!("page_token={}", page_token));
        }

        if !query_params.is_empty() {
            api_path.push('?');
            api_path.push_str(&query_params.join("&"));
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            ..Default::default()
        };

        let resp =
            Transport::<ListCustomAttrsResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

// ==================== 数据模型 ====================

/// 查询自定义用户字段请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCustomAttrsRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl Default for ListCustomAttrsRequest {
    fn default() -> Self {
        Self {
            page_size: None,
            page_token: None,
        }
    }
}

/// 查询自定义用户字段响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListCustomAttrsResponse {
    /// 自定义字段列表
    pub items: Vec<CustomAttr>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListCustomAttrsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 构建器模式 ====================

/// 自定义字段查询构建器
#[derive(Debug, Clone)]
pub struct ListCustomAttrsBuilder {
    request: ListCustomAttrsRequest,
}

impl Default for ListCustomAttrsBuilder {
    fn default() -> Self {
        Self {
            request: ListCustomAttrsRequest::default(),
        }
    }
}

impl ListCustomAttrsBuilder {
    /// 创建新的查询构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    /// 执行查询
    pub async fn execute(self, service: &CustomAttrService) -> SDKResult<ListCustomAttrsResponse> {
        service.list(&self.request).await
    }
}

impl CustomAttrService {
    /// 创建查询构建器
    pub fn list_custom_attrs_builder(&self) -> ListCustomAttrsBuilder {
        ListCustomAttrsBuilder::new()
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_attr_service_creation() {
        let config = Config::default();
        let service = CustomAttrService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_list_custom_attrs_request_default() {
        let request = ListCustomAttrsRequest::default();

        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_list_custom_attrs_request_with_pagination() {
        let request = ListCustomAttrsRequest {
            page_size: Some(50),
            page_token: Some("token123".to_string()),
        };

        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("token123".to_string()));
    }

    #[test]
    fn test_list_custom_attrs_builder() {
        let builder = ListCustomAttrsBuilder::new()
            .page_size(20)
            .page_token("test_token");

        assert_eq!(builder.request.page_size, Some(20));
        assert_eq!(builder.request.page_token, Some("test_token".to_string()));
    }

    #[test]
    fn test_list_custom_attrs_builder_default() {
        let builder = ListCustomAttrsBuilder::default();

        assert_eq!(builder.request.page_size, None);
        assert_eq!(builder.request.page_token, None);
    }

    #[test]
    fn test_response_default_creation() {
        let response = ListCustomAttrsResponse::default();

        assert_eq!(response.items.len(), 0);
        assert_eq!(response.has_more, None);
        assert_eq!(response.page_token, None);
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(ListCustomAttrsResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_response_with_data() {
        let mut response = ListCustomAttrsResponse::default();
        response.items = vec![CustomAttr {
            key: "field1".to_string(),
            name: "字段1".to_string(),
            ..Default::default()
        }];
        response.has_more = Some(true);
        response.page_token = Some("next_page".to_string());

        assert_eq!(response.items.len(), 1);
        assert_eq!(response.items[0].key, "field1");
        assert_eq!(response.has_more, Some(true));
        assert_eq!(response.page_token, Some("next_page".to_string()));
    }

    #[test]
    fn test_request_serialization() {
        let request = ListCustomAttrsRequest {
            page_size: Some(10),
            page_token: Some("token".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: ListCustomAttrsRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.page_size, deserialized.page_size);
        assert_eq!(request.page_token, deserialized.page_token);
    }

    #[test]
    fn test_query_parameters_construction() {
        let request = ListCustomAttrsRequest {
            page_size: Some(20),
            page_token: Some("test_token".to_string()),
        };

        let mut query_params = Vec::new();
        if let Some(page_size) = &request.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &request.page_token {
            query_params.push(format!("page_token={}", page_token));
        }

        assert_eq!(query_params.len(), 2);
        assert!(query_params.contains(&"page_size=20".to_string()));
        assert!(query_params.contains(&"page_token=test_token".to_string()));
    }
}
