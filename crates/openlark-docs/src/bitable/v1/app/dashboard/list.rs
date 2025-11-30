//! 列出仪表盘API v1
//!
//! 提供飞书多维表格仪表盘的列表查询功能，支持：
//! - 分页查询仪表盘列表
//! - 支持筛选和排序
//! - 提供仪表盘基本信息和权限信息

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use openlark_core::{
    api::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};

use super::{AppDashboardService, Dashboard};

/// 列出仪表盘请求
///
/// 用于查询指定多维表格应用中的所有仪表盘列表。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDashboardsRequest {
    /// 多维表格应用的唯一标识符
    pub app_token: String,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 分页大小，默认为20，最大为100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl ListDashboardsRequest {
    /// 创建新的列表请求实例
    ///
    /// # 参数
    /// - `app_token`: 多维表格应用的唯一标识符
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::cloud_docs::bitable::v1::app_dashboard::list::ListDashboardsRequest;
    ///
    /// let request = ListDashboardsRequest::new("app_token_123");
    /// ```
    pub fn new(app_token: impl Into<String>) -> Self {
        Self {
            app_token: app_token.into(),
            page_token: None,
            page_size: None,
            user_id_type: None,
        }
    }

    /// 设置分页标记
    ///
    /// # 参数
    /// - `page_token`: 分页标记
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::cloud_docs::bitable::v1::app_dashboard::list::ListDashboardsRequest;
    /// let mut request = ListDashboardsRequest::new("app_token_123");
    /// request.set_page_token("next_page_token");
    /// ```
    pub fn set_page_token(&mut self, page_token: impl Into<String>) -> &mut Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置分页大小
    ///
    /// # 参数
    /// - `page_size`: 分页大小，范围1-100
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::cloud_docs::bitable::v1::app_dashboard::list::ListDashboardsRequest;
    /// let mut request = ListDashboardsRequest::new("app_token_123");
    /// request.set_page_size(50);
    /// ```
    pub fn set_page_size(&mut self, page_size: i32) -> &mut Self {
        if page_size > 0 && page_size <= 100 {
            self.page_size = Some(page_size);
        }
        self
    }

    /// 设置用户ID类型
    ///
    /// # 参数
    /// - `user_id_type`: 用户ID类型（open_id、user_id、union_id）
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::cloud_docs::bitable::v1::app_dashboard::list::ListDashboardsRequest;
    /// let mut request = ListDashboardsRequest::new("app_token_123");
    /// request.set_user_id_type("open_id");
    /// ```
    pub fn set_user_id_type(&mut self, user_id_type: impl Into<String>) -> &mut Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 验证请求参数
    ///
    /// # 返回值
    /// - `Ok(())`: 验证通过
    /// - `Err(String)`: 验证失败，返回错误信息
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }

        if let Some(page_size) = self.page_size {
            if page_size < 1 || page_size > 100 {
                return Err("分页大小必须在1-100之间".to_string());
            }
        }

        if let Some(ref user_id_type) = self.user_id_type {
            if user_id_type.trim().is_empty() {
                return Err("用户ID类型不能为空".to_string());
            }
        }

        Ok(())
    }
}

/// 列出仪表盘响应数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListDashboardsResponseData {
    /// 仪表盘列表
    pub items: Vec<Dashboard>,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
    /// 总数量（可能不存在）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

/// 列出仪表盘响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListDashboardsResponse {
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ListDashboardsResponseData>,
    /// 是否成功
    pub success: bool,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 错误代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

impl ApiResponseTrait for ListDashboardsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl AppDashboardService {
    /// 列出仪表盘
    ///
    /// 查询指定多维表格应用中的所有仪表盘列表，支持分页查询。
    ///
    /// # 参数
    /// * `req` - 列出仪表盘请求
    ///
    /// # 返回值
    /// 返回仪表盘列表信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::bitable::v1::app_dashboard::{
    ///     AppDashboardService, list::ListDashboardsRequest
    /// };
    ///
    /// let service = AppDashboardService::new(config);
    /// let request = ListDashboardsRequest::new("app_token_123")
    ///     .set_page_size(20)
    ///     .set_user_id_type("open_id");
    ///
    /// let result = service.list_dashboards(&request).await?;
    /// for dashboard in &result.data.as_ref().unwrap().items {
    ///     println!("仪表盘名称: {}", dashboard.name);
    /// }
    /// ```
    pub async fn list_dashboards(
        &self,
        req: &ListDashboardsRequest,
    ) -> SDKResult<ListDashboardsResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!("开始列出仪表盘: app_token={}", req.app_token);

        // 构建查询参数
        let mut query_params = Vec::new();
        if let Some(ref page_token) = req.page_token {
            query_params.push(("page_token", page_token.clone()));
        }
        if let Some(page_size) = req.page_size {
            query_params.push(("page_size", page_size.to_string()));
        }
        if let Some(ref user_id_type) = req.user_id_type {
            query_params.push(("user_id_type", user_id_type.clone()));
        }

        // 构建API路径
        let endpoint = openlark_core::endpoints::Endpoints::BITABLE_V1_DASHBOARDS
            .replace("{app_token}", &req.app_token);

        let api_req = ApiRequest::
            url: format!(
            url: endpoint,
            query(),
            body: None, // GET请求无body
            .query(HashMap::new())
        };

        let resp = Transport::<ListDashboardsResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        if response.success {
            if let Some(ref data) = response.data {
                log::info!(
                    "仪表盘列表获取完成: app_token={}, count={}, has_more={:?}",
                    req.app_token,
                    data.items.len(),
                    data.has_more
                );
            }
        } else {
            log::warn!(
                "仪表盘列表获取失败: app_token={}, error={:?}",
                req.app_token,
                response.error_message
            );
        }

        Ok(response)
    }
}

// ==================== 构建器模式 ====================

/// 列出仪表盘构建器
#[derive(Clone)]
pub struct ListDashboardsBuilder {
    request: ListDashboardsRequest,
}

impl Default for ListDashboardsBuilder {
    fn default() -> Self {
        Self {
            request: ListDashboardsRequest {
                app_token: String::new(),
                page_token: None,
                page_size: None,
                user_id_type: None,
            },
        }
    }
}

impl ListDashboardsBuilder {
    /// 创建新的构建器
    ///
    /// # 参数
    /// - `app_token`: 多维表格应用的唯一标识符
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::cloud_docs::bitable::v1::app_dashboard::list::ListDashboardsBuilder;
    ///
    /// let builder = ListDashboardsBuilder::new("app_token_123");
    /// ```
    pub fn new(app_token: impl Into<String>) -> Self {
        Self {
            request: ListDashboardsRequest::new(app_token),
        }
    }

    /// 设置分页标记
    ///
    /// # 参数
    /// - `page_token`: 分页标记
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::cloud_docs::bitable::v1::app_dashboard::list::ListDashboardsBuilder;
    /// let builder = ListDashboardsBuilder::new("app_token_123")
    ///     .page_token("next_page_token");
    /// ```
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.set_page_token(page_token);
        self
    }

    /// 设置分页大小
    ///
    /// # 参数
    /// - `page_size`: 分页大小，范围1-100
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::cloud_docs::bitable::v1::app_dashboard::list::ListDashboardsBuilder;
    /// let builder = ListDashboardsBuilder::new("app_token_123")
    ///     .page_size(50);
    /// ```
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.set_page_size(page_size);
        self
    }

    /// 设置用户ID类型
    ///
    /// # 参数
    /// - `user_id_type`: 用户ID类型
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::cloud_docs::bitable::v1::app_dashboard::list::ListDashboardsBuilder;
    /// let builder = ListDashboardsBuilder::new("app_token_123")
    ///     .user_id_type("open_id");
    /// ```
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.set_user_id_type(user_id_type);
        self
    }

    /// 执行列出仪表盘操作
    ///
    /// # 参数
    /// - `service`: 仪表盘服务实例
    ///
    /// # 返回值
    /// 返回仪表盘列表信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::bitable::v1::app_dashboard::{
    ///     AppDashboardService, list::ListDashboardsBuilder
    /// };
    ///
    /// let service = AppDashboardService::new(config);
    ///
    /// let result = ListDashboardsBuilder::new("app_token_123")
    ///     .page_size(20)
    ///     .user_id_type("open_id")
    ///     .execute(&service)
    ///     .await?;
    /// ```
    pub async fn execute(self, service: &AppDashboardService) -> SDKResult<ListDashboardsResponse> {
        service.list_dashboards(&self.request).await
    }
}

impl AppDashboardService {
    /// 创建列出仪表盘构建器
    ///
    /// # 返回值
    /// 返回列表构建器实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::bitable::v1::app_dashboard::AppDashboardService;
    ///
    /// let service = AppDashboardService::new(config);
    /// let builder = service.list_dashboards_builder("app_token_123");
    /// ```
    pub fn list_dashboards_builder(&self, app_token: impl Into<String>) -> ListDashboardsBuilder {
        ListDashboardsBuilder::new(app_token)
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_dashboards_request_creation() {
        let request = ListDashboardsRequest::new("app_token_123");
        assert_eq!(request.app_token, "app_token_123");
        assert_eq!(request.page_token, None);
        assert_eq!(request.page_size, None);
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_list_dashboards_request_with_fields() {
        let mut request = ListDashboardsRequest::new("app_token_123");
        request.set_page_token("next_page_token");
        request.set_page_size(50);
        request.set_user_id_type("open_id");

        assert_eq!(request.app_token, "app_token_123");
        assert_eq!(request.page_token, Some("next_page_token".to_string()));
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_request_validation_success() {
        let request = ListDashboardsRequest::new("app_token_123");
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_request_validation_empty_app_token() {
        let request = ListDashboardsRequest::new("");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "应用token不能为空");
    }

    #[test]
    fn test_request_validation_page_size_out_of_range() {
        let mut request = ListDashboardsRequest::new("app_token_123");
        request.set_page_size(0);
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "分页大小必须在1-100之间");

        request.set_page_size(101);
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "分页大小必须在1-100之间");
    }

    #[test]
    fn test_request_validation_empty_user_id_type() {
        let mut request = ListDashboardsRequest::new("app_token_123");
        request.set_user_id_type("");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "用户ID类型不能为空");
    }

    #[test]
    fn test_response_creation() {
        let dashboard = Dashboard {
            block_id: "block_123".to_string(),
            name: "测试仪表盘".to_string(),
        };

        let data = ListDashboardsResponseData {
            items: vec![dashboard],
            page_token: Some("next_token".to_string()),
            has_more: Some(true),
            total: Some(100),
        };

        let response = ListDashboardsResponse {
            data: Some(data),
            success: true,
            .query(HashMap::new())
        };

        assert!(response.success);
        assert_eq!(response.data.unwrap().items.len(), 1);
        assert_eq!(response.data.unwrap().items[0].name, "测试仪表盘");
    }

    #[test]
    fn test_list_dashboards_builder() {
        let builder = ListDashboardsBuilder::new("app_token_123")
            .page_token("next_token")
            .page_size(30)
            .user_id_type("union_id");

        assert_eq!(builder.request.app_token, "app_token_123");
        assert_eq!(builder.request.page_token, Some("next_token".to_string()));
        assert_eq!(builder.request.page_size, Some(30));
        assert_eq!(builder.request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_builder_default() {
        let builder = ListDashboardsBuilder::default();
        assert_eq!(builder.request.app_token, "");
        assert_eq!(builder.request.page_token, None);
        assert_eq!(builder.request.page_size, None);
        assert_eq!(builder.request.user_id_type, None);
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(ListDashboardsResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_request_serialization() {
        let request = ListDashboardsRequest {
            app_token: "app_token_123".to_string(),
            page_token: Some("next_token".to_string()),
            page_size: Some(50),
            user_id_type: Some("open_id".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: ListDashboardsRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.app_token, deserialized.app_token);
        assert_eq!(request.page_token, deserialized.page_token);
        assert_eq!(request.page_size, deserialized.page_size);
        assert_eq!(request.user_id_type, deserialized.user_id_type);
    }

    #[test]
    fn test_response_serialization() {
        let mut response = ListDashboardsResponse::default();
        response.data = Some(ListDashboardsResponseData {
            items: vec![Dashboard {
                block_id: "block_456".to_string(),
                name: "序列化测试".to_string(),
            }],
            page_token: Some("test_token".to_string()),
            has_more: Some(false),
            total: Some(1),
        });
        response.success = true;

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: ListDashboardsResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(response.success, deserialized.success);
        assert_eq!(response.data.unwrap().items.len(), deserialized.data.unwrap().items.len());
        assert_eq!(response.data.unwrap().items[0].name, "序列化测试");
    }

    #[test]
    fn test_service_builder_method() {
        let config = openlark_core::config::Config::default();
        let service = AppDashboardService::new(config);
        let builder = service.list_dashboards_builder("app_token_test");

        assert_eq!(builder.request.app_token, "app_token_test");
    }

    #[test]
    fn test_comprehensive_scenario() {
        // 测试完整的业务场景
        let request = ListDashboardsRequest::new("complex_app_token")
            .page_size(25)
            .user_id_type("union_id")
            .page_token("specific_page_token");

        assert!(request.validate().is_ok());

        // 验证所有字段都正确设置
        assert_eq!(request.app_token, "complex_app_token");
        assert_eq!(request.page_size, Some(25));
        assert_eq!(request.user_id_type, Some("union_id".to_string()));
        assert_eq!(request.page_token, Some("specific_page_token".to_string()));
    }
}