use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::,
{
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api_resp::{ApiResponseTrait}
    constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
};
    impl_executable_builder_owned,
};
use super::AppRoleMemberService;
impl AppRoleMemberService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 新增协作者请求,
#[derive(Debug, Clone)]
pub struct CreateRoleMemberRequest {
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
    /// 成员id
    member_id: String,
    /// 成员类型: user, chat, department, open_department_id
    member_type: String}
impl CreateRoleMemberRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct CreateRoleMemberRequestBuilder {
    request: CreateRoleMemberRequest}
impl CreateRoleMemberRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request,
impl_executable_builder_owned!(,
    CreateRoleMemberRequestBuilder,
    AppRoleMemberService,
    CreateRoleMemberRequest,
    BaseResponse<CreateRoleMemberResponse>,
    create,
);
/// 协作者信息
#[derive(Debug, Clone)]
pub struct RoleMember {
    /// 成员id
    pub member_id: String,
    /// 成员类型
    pub member_type: String,
    /// 成员名称
    pub member_name: Option<String>}
/// 新增协作者响应,
#[derive(Debug, Clone)]
pub struct CreateRoleMemberResponse {
    /// 新增的协作者信息
    pub member: RoleMember,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
#[test]
    fn test_create_role_member_request_builder() {
let request = CreateRoleMemberRequest::builder(),
            .app_token()
.role_id()
            .member_id()
.member_type()
            .user_id_type()
.build();
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.role_id, "rolxxxxxx");
        assert_eq!(request.member_id, "ou_xxxxxx");
        assert_eq!(request.member_type, "user");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
