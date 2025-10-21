use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
};

use super::{AppRoleMemberService, RoleMember};

impl AppRoleMemberService {
    /// 列出协作者
    pub async fn list(
        &self,
        request: ListRoleMemberRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListRoleMemberResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::GET;
        api_req.api_path = BITABLE_V1_ROLE_MEMBERS
            .replace("{app_token}", &request.app_token)
            .replace("{role_id}", &request.role_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

/// 列出协作者请求
#[derive(Debug, Serialize, Default)]
pub struct ListRoleMemberRequest {
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
    /// 分页标记
    #[serde(skip)]
    page_token: Option<String>,
    /// 分页大小
    #[serde(skip)]
    page_size: Option<i32>,
}

impl ListRoleMemberRequest {
    pub fn builder() -> ListRoleMemberRequestBuilder {
        ListRoleMemberRequestBuilder::default()
    }

    pub fn new(app_token: impl ToString, role_id: impl ToString) -> Self {
        Self {
            app_token: app_token.to_string(),
            role_id: role_id.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct ListRoleMemberRequestBuilder {
    request: ListRoleMemberRequest,
}

impl ListRoleMemberRequestBuilder {
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

    /// 分页标记
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    pub fn build(mut self) -> ListRoleMemberRequest {
        if let Some(user_id_type) = &self.request.user_id_type {
            self.request
                .api_request
                .query_params
                .insert("user_id_type", user_id_type.clone());
        }
        if let Some(page_token) = &self.request.page_token {
            self.request
                .api_request
                .query_params
                .insert("page_token", page_token.clone());
        }
        if let Some(page_size) = &self.request.page_size {
            self.request
                .api_request
                .query_params
                .insert("page_size", page_size.to_string());
        }
        self.request
    }
}

impl_executable_builder_owned!(
    ListRoleMemberRequestBuilder,
    AppRoleMemberService,
    ListRoleMemberRequest,
    BaseResponse<ListRoleMemberResponse>,
    list
);

/// 列出协作者响应
#[derive(Debug, Deserialize)]
pub struct ListRoleMemberResponse {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: i32,
    /// 协作者信息列表
    pub items: Vec<RoleMember>,
}

impl ApiResponseTrait for ListRoleMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_list_role_member_request_builder() {
        let request = ListRoleMemberRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .role_id("rolxxxxxx")
            .page_size(20)
            .user_id_type("open_id")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.role_id, "rolxxxxxx");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }
}
