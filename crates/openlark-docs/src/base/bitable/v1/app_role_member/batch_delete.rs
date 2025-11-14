#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use SDKResult;use reqwest::Method;
use openlark_core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use super::AppRoleMemberService;
use openlark_core::,
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
/// 批量删除协作者请求,
#[derive(Clone)]
pub struct BatchDeleteRoleMemberRequest {
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
    /// 成员id列表
    member_ids: Vec<String>}
impl BatchDeleteRoleMemberRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone)]
pub struct BatchDeleteRoleMemberRequestBuilder {
    request: BatchDeleteRoleMemberRequest}
impl BatchDeleteRoleMemberRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request,
impl_executable_builder_owned!(,
    BatchDeleteRoleMemberRequestBuilder,
    AppRoleMemberService,
    BatchDeleteRoleMemberRequest,
    BaseResponse<BatchDeleteRoleMemberResponse>,
    batch_delete,
);
/// 删除结果
#[derive(Clone)]
pub struct DeleteResult {
    /// 成员ID
    pub member_id: String,
    /// 是否删除成功
    pub deleted: bool,
/// 批量删除协作者响应,
#[derive(Clone)]
pub struct BatchDeleteRoleMemberResponse {
    /// 删除结果列表
    pub results: Vec<DeleteResult>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 批量删除协作者,
pub async fn batch_delete_role_members(
    request: BatchDeleteRoleMemberRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<BatchDeleteRoleMemberResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::DELETE);
api_req.api_path = BITABLE_V1_ROLE_MEMBERS_BATCH_DELETE,
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
    fn test_batch_delete_role_member_request_builder() {
let request = BatchDeleteRoleMemberRequest::builder(),
            .app_token()
.role_id()
            .add_member_id()
.add_member_id()
            .user_id_type()
.build();
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.role_id, "rolxxxxxx");
        assert_eq!(request.member_ids.len(), 2);
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
