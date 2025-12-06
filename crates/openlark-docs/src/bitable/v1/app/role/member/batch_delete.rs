//! Bitable 批量删除角色成员API
///
/// API文档: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/role/member/batchDelete

use openlark_core::{
    api::ApiRequest,
    config::Config,
    error::{SDKResult, validation_error},
    http::Transport,
};
use serde::{Deserialize, Serialize};

use super::models::{BatchDeleteRoleMemberRequestModel as ModelBatchDeleteRequest, BatchDeleteRoleMemberResponseModel as ModelBatchDeleteResponse, BatchDeleteResultItemModel as ModelBatchDeleteResultItem};

/// 批量删除角色成员请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BatchDeleteRoleMemberRequest {
    /// 配置信息
    config: Config,
    api_request: ApiRequest<BatchDeleteRoleMemberResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 角色的ID
    role_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 成员ID列表
    member_ids: Vec<String>,
}

impl BatchDeleteRoleMemberRequest {
    /// 创建批量删除角色成员请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::post("").header("Content-Type", "application/json"),
            app_token: String::new(),
            role_id: String::new(),
            user_id_type: None,
            member_ids: Vec::new(),
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置角色 ID
    pub fn role_id(mut self, role_id: String) -> Self {
        self.role_id = role_id;
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置成员ID列表
    pub fn member_ids(mut self, member_ids: Vec<String>) -> Self {
        self.member_ids = member_ids;
        self
    }

    /// 添加成员ID
    pub fn add_member_id(mut self, member_id: String) -> Self {
        self.member_ids.push(member_id);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchDeleteRoleMemberResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        if self.role_id.trim().is_empty() {
            return Err(validation_error("role_id", "角色ID不能为空"));
        }

        if self.member_ids.is_empty() {
            return Err(validation_error("member_ids", "成员ID列表不能为空"));
        }

        if self.member_ids.len() > 500 {
            return Err(validation_error("member_ids", "批量删除成员数量不能超过500个"));
        }

        // 构建完整的API URL
        let api_url = format!("{}/open-apis/bitable/v1/apps/{}/roles/{}/members/batch_delete",
                             self.config.base_url, self.app_token, self.role_id);

        // 构建请求体
        let request_body = ModelBatchDeleteRequest {
            member_ids: self.member_ids.clone(),
        };

        // 设置API URL和请求体
        let mut api_request = self.api_request;
        api_request.url = api_url;

        // 设置查询参数
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type);
        }

        api_request.body = Some(openlark_core::api::RequestData::Json(serde_json::to_value(&request_body)?));

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;

        // 解析响应
        let response_data: ModelBatchDeleteResponse = response
            .data
            .and_then(|data| serde_json::from_value(data).ok())
            .ok_or_else(|| validation_error("解析批量删除成员数据失败", "响应数据格式不正确"))?;

        // 转换为标准响应格式
        let standard_response = BatchDeleteRoleMemberResponse {
            results: response_data.results,
            has_more: response_data.has_more,
            page_token: response_data.page_token,
            success: response.raw_response.is_success(),
        };

        Ok(standard_response)
    }
}

/// 批量删除角色成员Builder
pub struct BatchDeleteRoleMemberRequestBuilder {
    request: BatchDeleteRoleMemberRequest,
}

impl BatchDeleteRoleMemberRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: BatchDeleteRoleMemberRequest::new(config),
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置角色 ID
    pub fn role_id(mut self, role_id: String) -> Self {
        self.request = self.request.role_id(role_id);
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 设置成员ID列表
    pub fn member_ids(mut self, member_ids: Vec<String>) -> Self {
        self.request = self.request.member_ids(member_ids);
        self
    }

    /// 添加成员ID
    pub fn add_member_id(mut self, member_id: String) -> Self {
        self.request = self.request.add_member_id(member_id);
        self
    }

    /// 构建请求
    pub fn build(self) -> BatchDeleteRoleMemberRequest {
        self.request
    }
}

/// 批量删除结果项
pub type BatchDeleteResultItem = ModelBatchDeleteResultItem;

/// 批量删除角色成员响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchDeleteRoleMemberResponse {
    /// 批量操作结果
    pub results: Vec<BatchDeleteResultItem>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 页面 token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 操作结果
    pub success: bool,
}