/// Bitable 批量新增协作者（自定义角色）
///
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-role-member/batch_create
/// doc: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-role-member/batch_create
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

use super::models::RoleMemberId;

/// 批量新增协作者请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BatchCreateRoleMemberRequest {
    config: Config,
    app_token: String,
    role_id: String,
    member_list: Vec<RoleMemberId>,
}

impl BatchCreateRoleMemberRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            role_id: String::new(),
            member_list: Vec::new(),
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    pub fn role_id(mut self, role_id: String) -> Self {
        self.role_id = role_id;
        self
    }

    pub fn member_list(mut self, member_list: Vec<RoleMemberId>) -> Self {
        self.member_list = member_list;
        self
    }

    pub fn add_member(mut self, member: RoleMemberId) -> Self {
        self.member_list.push(member);
        self
    }

    pub async fn execute(self) -> SDKResult<BatchCreateRoleMemberResponse> {
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "app_token 不能为空"));
        }
        if self.role_id.trim().is_empty() {
            return Err(validation_error("role_id", "role_id 不能为空"));
        }
        if self.member_list.is_empty() {
            return Err(validation_error("member_list", "member_list 不能为空"));
        }
        if self.member_list.len() > 100 {
            return Err(validation_error("member_list", "member_list 最多 100 项"));
        }
        for (idx, item) in self.member_list.iter().enumerate() {
            if item.id.trim().is_empty() {
                return Err(validation_error(
                    "member_list",
                    &format!("第 {} 个成员的 id 不能为空", idx + 1),
                ));
            }
        }

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint =
            BitableApiV1::RoleMemberBatchCreate(self.app_token.clone(), self.role_id.clone());

        let api_request: ApiRequest<BatchCreateRoleMemberResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serde_json::to_vec(
                &BatchCreateRoleMemberRequestBody {
                    member_list: self.member_list,
                },
            )?);

        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| validation_error("response", "响应数据为空"))
    }
}

/// 批量新增协作者 Builder
pub struct BatchCreateRoleMemberRequestBuilder {
    request: BatchCreateRoleMemberRequest,
}

impl BatchCreateRoleMemberRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: BatchCreateRoleMemberRequest::new(config),
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    pub fn role_id(mut self, role_id: String) -> Self {
        self.request = self.request.role_id(role_id);
        self
    }

    pub fn member_list(mut self, member_list: Vec<RoleMemberId>) -> Self {
        self.request = self.request.member_list(member_list);
        self
    }

    pub fn add_member(mut self, member: RoleMemberId) -> Self {
        self.request = self.request.add_member(member);
        self
    }

    pub fn build(self) -> BatchCreateRoleMemberRequest {
        self.request
    }
}

#[derive(Serialize)]
struct BatchCreateRoleMemberRequestBody {
    member_list: Vec<RoleMemberId>,
}

/// 批量新增协作者响应（data 为 {}）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateRoleMemberResponse {}

impl ApiResponseTrait for BatchCreateRoleMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
