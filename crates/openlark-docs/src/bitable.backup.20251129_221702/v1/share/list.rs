//! Bitable V1 列出协作者API

#![allow(unused_variables, unused_imports, dead_code, non_snake_case)]
#![allow(clippy::too_many_arguments)]

use openlark_core::{
    api::ApiRequest,
    config::Config,
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 列出协作者请求
#[derive(Clone)]
pub struct ListCollaboratorV1Request {
    api_request: ApiRequest,
    app_token: String,
    page_size: Option<i32>,
    page_token: Option<String>,
}

/// 列出协作者响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListCollaboratorV1Response {
    pub data: ListCollaboratorV1Data,
    pub success: bool,
}

/// 列出协作者响应数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListCollaboratorV1Data {
    /// 协作者列表
    pub items: Vec<CollaboratorItem>,
    /// 分页信息
    pub page_token: Option<String>,
    /// 是否有更多数据
    pub has_more: bool,
}

/// 协作者项目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CollaboratorItem {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 权限
    pub perm: String,
    /// 加入时间
    pub add_time: Option<String>,
}

impl ListCollaboratorV1Request {
    /// 创建列出协作者请求
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config),
            app_token: String::new(),
            page_size: None,
            page_token: None,
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListCollaboratorV1Response> {
        // 构建API路径
        let path = format!("/open-apis/bitable/v1/apps/{}/collaborators", self.app_token);

        // 验证请求参数
        if self.app_token.is_empty() {
            return Err(openlark_core::error::SDKError::ValidationError("应用token不能为空".to_string()));
        }

        // 构建查询参数
        let mut query_params = Vec::new();

        if let Some(page_size) = self.page_size {
            query_params.push(format!("page_size={}", page_size));
        }

        if let Some(page_token) = &self.page_token {
            query_params.push(format!("page_token={}", page_token));
        }

        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };

        // 发送请求
        let response = self.api_request
            .method(&openlark_core::http::Method::GET)
            .path(&format!("{}{}", path, query_string))
            .execute::<ListCollaboratorV1Response>()
            .await?;

        Ok(response)
    }
}

/// 列出协作者Builder
#[derive(Clone)]
pub struct ListCollaboratorV1Builder {
    request: ListCollaboratorV1Request,
}

impl ListCollaboratorV1Builder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: ListCollaboratorV1Request::new(config),
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request = self.request.page_size(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.request = self.request.page_token(page_token);
        self
    }

    /// 构建请求
    pub fn build(self) -> ListCollaboratorV1Request {
        self.request
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::Config;

    #[test]
    fn test_list_collaborator_request_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = ListCollaboratorV1Request::builder(config)
            .app_token("bascnmBA*****yGehy8")
            .page_size(20)
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.page_size, Some(20));
    }

    #[test]
    fn test_list_collaborator_request_new() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = ListCollaboratorV1Request::new(config)
            .app_token("test_app_token")
            .page_size(50);

        assert_eq!(request.app_token, "test_app_token");
        assert_eq!(request.page_size, Some(50));
    }

    #[test]
    fn test_list_collaborator_response_creation() {
        let items = vec![
            CollaboratorItem {
                user_id: "user_123".to_string(),
                name: Some("张三".to_string()),
                email: Some("zhangsan@example.com".to_string()),
                perm: "view".to_string(),
                add_time: Some("2023-12-01T10:00:00Z".to_string()),
            },
            CollaboratorItem {
                user_id: "user_456".to_string(),
                name: Some("李四".to_string()),
                email: Some("lisi@example.com".to_string()),
                perm: "edit".to_string(),
                add_time: Some("2023-12-01T11:00:00Z".to_string()),
            },
        ];

        let response_data = ListCollaboratorV1Data {
            items: items.clone(),
            page_token: Some("next_page_token".to_string()),
            has_more: true,
        };

        let response = ListCollaboratorV1Response {
            data: response_data,
            success: true,
        };

        assert_eq!(response.success, true);
        assert_eq!(response.data.items.len(), 2);
        assert_eq!(response.data.has_more, true);
        assert!(response.data.page_token.is_some());
    }
}