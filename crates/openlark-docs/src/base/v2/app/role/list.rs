//! Base 列出自定义角色API
///
/// API文档: https://open.feishu.cn/document/docs/bitable-v1/advanced-permission/app-role/list-2

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::{
    models::PaginationParams,
    RoleService,
};

/// 列出自定义角色请求
pub struct ListRolesRequest {
    app_token: String,
    /// 分页参数
    pagination: Option<PaginationParams>,
    /// 配置信息
    config: Config,
}

impl ListRolesRequest {
    /// 创建列出角色请求
    pub fn new(config: Config) -> Self {
        Self {
            app_token: String::new(),
            pagination: None,
            config,
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 设置分页参数
    pub fn pagination(mut self, pagination: PaginationParams) -> Self {
        self.pagination = Some(pagination);
        self
    }

    /// 设置页面大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        let mut pagination = self.pagination.unwrap_or_default();
        pagination.page_size = Some(page_size);
        self.pagination = Some(pagination);
        self
    }

    /// 设置页面 token
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        let mut pagination = self.pagination.unwrap_or_default();
        pagination.page_token = Some(page_token.into());
        self.pagination = Some(pagination);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListRolesResponseData> {
        // 验证必填字段
        validate_required!(self.app_token, "应用令牌不能为空");

        // 🚀 使用新的enum+builder系统生成API端点
        use crate::common::api_endpoints::BaseApiV2;
        let api_endpoint = BaseApiV2::RoleList(self.app_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<ListRolesResponseData> = ApiRequest::get(&api_endpoint.to_url());

        // 构建查询参数
        if let Some(ref pagination) = self.pagination {
            if let Some(page_size) = pagination.page_size {
                api_request = api_request.query("page_size", &page_size.to_string());
            }

            if let Some(ref page_token) = pagination.page_token {
                api_request = api_request.query("page_token", page_token);
            }
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// 列出自定义角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListRolesResponseData {
    /// 角色列表
    pub data: super::models::ListRolesResponse,
}

impl ApiResponseTrait for ListRolesResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl RoleService {
    /// 创建列出角色请求
    pub fn list_roles_builder(
        &self,
        app_token: impl Into<String>,
    ) -> ListRolesRequest {
        ListRolesRequest::new(self.config.clone()).app_token(app_token)
    }

    /// 创建列出角色请求（带分页参数）
    pub fn list_roles(
        &self,
        app_token: impl Into<String>,
        pagination: Option<PaginationParams>,
    ) -> ListRolesRequest {
        let mut request = ListRolesRequest::new(self.config.clone()).app_token(app_token);

        if let Some(pag) = pagination {
            request = request.pagination(pag);
        }

        request
    }

    /// 创建列出角色请求（带页面大小和token）
    pub fn list_roles_simple(
        &self,
        app_token: impl Into<String>,
        page_size: Option<i32>,
        page_token: Option<impl Into<String>>,
    ) -> ListRolesRequest {
        let mut request = ListRolesRequest::new(self.config.clone()).app_token(app_token);

        if let Some(size) = page_size {
            request = request.page_size(size);
        }

        if let Some(token) = page_token {
            request = request.page_token(token);
        }

        request
    }
}
