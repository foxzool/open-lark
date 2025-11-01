use crate::core::SDKResult;use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use super::AppRoleMemberService;
use crate::,
{,
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api_resp::{ApiResponseTrait}
    config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
};
    impl_executable_builder_owned,
};
/// 删除协作者请求,
#[derive(Debug, Clone)]
pub struct DeleteRoleMemberRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符,
#[serde(skip)]
    app_token: String,
    /// 自定义角色的id,
#[serde(skip)]
    role_id: String,
    /// 成员id,
#[serde(skip)]
    member_id: String,
    /// 用户id类型,
#[serde(skip)]
    user_id_type: Option<String>}
impl DeleteRoleMemberRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct DeleteRoleMemberRequestBuilder {
    request: DeleteRoleMemberRequest}
impl DeleteRoleMemberRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}self.request,
    }
impl_executable_builder_owned!(,
    DeleteRoleMemberRequestBuilder,
    AppRoleMemberService,
    DeleteRoleMemberRequest,
    BaseResponse<DeleteRoleMemberResponse>,
    delete,
);
/// 删除协作者响应
#[derive(Debug, Clone)]
pub struct DeleteRoleMemberResponse {
    /// 删除的成员ID
    pub member_id: String,
    /// 是否删除成功
    pub deleted: bool,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 删除协作者,
pub async fn delete_role_member(
    request: DeleteRoleMemberRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<DeleteRoleMemberResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::DELETE);
api_req.api_path = BITABLE_V1_ROLE_MEMBER_DELETE,
        .replace("{app_token}", &request.app_token)
        .replace("{role_id}", &request.role_id)
        .replace("{member_id}", &request.member_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
#[test]
    fn test_delete_role_member_request_builder() {
let request = DeleteRoleMemberRequest::builder(),
            .app_token()
.role_id()
            .member_id()
.user_id_type()
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.role_id, "rolxxxxxx");
        assert_eq!(request.member_id, "ou_xxxxxx");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
