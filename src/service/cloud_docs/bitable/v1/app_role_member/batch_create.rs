use reqwest::Method;
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
    service::bitable::v1::app_role_member::RoleMember,
};
/// 批量新增协作者请求,
#[derive(Debug, Clone)]
pub struct BatchCreateRoleMemberRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符,
#[serde(skip)]
    app_token: String,
    /// 自定义角色的id,
#[serde(skip)]
    role_id: String,
    /// 用户id类型,
#[serde(skip)]
    user_id_type: Option<String>,
    /// 成员列表
    members: Vec<MemberInfo>}
/// 成员信息,
#[derive(Debug, Clone)]
pub struct MemberInfo {
    /// 成员id
    pub member_id: String,
    /// 成员类型: user, chat, department, open_department_id
    pub member_type: String,
impl BatchCreateRoleMemberRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct BatchCreateRoleMemberRequestBuilder {
    request: BatchCreateRoleMemberRequest}
impl BatchCreateRoleMemberRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request,
impl_executable_builder_owned!(,
    BatchCreateRoleMemberRequestBuilder,
    AppRoleMemberService,
    BatchCreateRoleMemberRequest,
    BaseResponse<BatchCreateRoleMemberResponse>,
    batch_create,
);
/// 批量新增协作者响应
#[derive(Debug, Clone)]
pub struct BatchCreateRoleMemberResponse {
    /// 新增的协作者信息列表
    pub members: Vec<RoleMember>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 批量新增协作者,
pub async fn batch_create_role_members(
    request: BatchCreateRoleMemberRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<BatchCreateRoleMemberResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
api_req.api_path = BITABLE_V1_ROLE_MEMBERS_BATCH_CREATE,
        .replace("{app_token}", &request.app_token)
        .replace("{role_id}", &request.role_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
#[test]
    fn test_batch_create_role_member_request_builder() {
let request = BatchCreateRoleMemberRequest::builder(),
            .app_token()
.role_id()
            .add_member("ou_xxxxxx", "user")
            .add_member()
.user_id_type()
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.role_id, "rolxxxxxx");
        assert_eq!(request.members.len(), 2);
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
