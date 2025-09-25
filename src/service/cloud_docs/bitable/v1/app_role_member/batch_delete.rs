use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::AppRoleMemberService;
use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
};

/// 批量删除协作者请求
#[derive(Debug, Serialize, Default)]
pub struct BatchDeleteRoleMemberRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 自定义角色的id
    #[serde(skip)]
    role_id: String,
    /// 用户id类型
    #[serde(skip)]
    user_id_type: Option<String>,
    /// 成员id列表
    member_ids: Vec<String>,
}

impl BatchDeleteRoleMemberRequest {
    pub fn builder() -> BatchDeleteRoleMemberRequestBuilder {
        BatchDeleteRoleMemberRequestBuilder::default()
    }

    pub fn new(app_token: impl ToString, role_id: impl ToString, member_ids: Vec<String>) -> Self {
        Self {
            app_token: app_token.to_string(),
            role_id: role_id.to_string(),
            member_ids,
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct BatchDeleteRoleMemberRequestBuilder {
    request: BatchDeleteRoleMemberRequest,
}

impl BatchDeleteRoleMemberRequestBuilder {
    /// 多维表格的唯一标识符
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 自定义角色的id
    pub fn role_id(mut self, role_id: impl ToString) -> Self {
        self.request.role_id = role_id.to_string();
        self
    }

    /// 用户id类型
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 成员id列表
    pub fn member_ids(mut self, member_ids: Vec<String>) -> Self {
        self.request.member_ids = member_ids;
        self
    }

    /// 添加单个成员id
    pub fn add_member_id(mut self, member_id: impl ToString) -> Self {
        self.request.member_ids.push(member_id.to_string());
        self
    }

    pub fn build(mut self) -> BatchDeleteRoleMemberRequest {
        if let Some(user_id_type) = &self.request.user_id_type {
            self.request
                .api_request
                .query_params
                .insert("user_id_type", user_id_type.clone());
        }
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_owned!(
    BatchDeleteRoleMemberRequestBuilder,
    AppRoleMemberService,
    BatchDeleteRoleMemberRequest,
    BaseResponse<BatchDeleteRoleMemberResponse>,
    batch_delete
);

/// 删除结果
#[derive(Debug, Deserialize)]
pub struct DeleteResult {
    /// 成员ID
    pub member_id: String,
    /// 是否删除成功
    pub deleted: bool,
}

/// 批量删除协作者响应
#[derive(Debug, Deserialize)]
pub struct BatchDeleteRoleMemberResponse {
    /// 删除结果列表
    pub results: Vec<DeleteResult>,
}

impl ApiResponseTrait for BatchDeleteRoleMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量删除协作者
pub async fn batch_delete_role_members(
    request: BatchDeleteRoleMemberRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<BatchDeleteRoleMemberResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::DELETE;
    api_req.api_path = BITABLE_V1_ROLE_MEMBERS_BATCH_DELETE
        .replace("{app_token}", &request.app_token)
        .replace("{role_id}", &request.role_id);
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_delete_role_member_request_builder() {
        let request = BatchDeleteRoleMemberRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .role_id("rolxxxxxx")
            .add_member_id("ou_xxxxxx")
            .add_member_id("ou_yyyyyy")
            .user_id_type("open_id")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.role_id, "rolxxxxxx");
        assert_eq!(request.member_ids.len(), 2);
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }
}
