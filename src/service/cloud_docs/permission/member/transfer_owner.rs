use reqwest::Method;
use openlark_core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::{,
use SDKResult;    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::{cloud_docs::*, EndpointBuilder}
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
/// 转移所有者请求,
#[derive(Debug, Clone)]
pub struct TransferOwnerRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token,
#[serde(skip)]
    token: String,
    /// 文档类型,
#[serde(skip)]
    obj_type: String,
    /// 新所有者ID类型
    member_type: String,
    /// 新所有者ID
    member_id: String,
    /// 是否移除当前所有者的权限,
#[serde(skip_serializing_if = "Option::is_none")]
    remove_old_owner: Option<bool>,
    /// 是否通知,
#[serde(skip_serializing_if = "Option::is_none")]
    need_notification: Option<bool>}
impl TransferOwnerRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct TransferOwnerRequestBuilder {
    request: TransferOwnerRequest}
impl TransferOwnerRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 转移结果,
#[derive(Debug, Clone)]
pub struct TransferResult {
    /// 新所有者ID类型
    pub member_type: String,
    /// 新所有者ID
    pub member_id: String,
    /// 转移时间（毫秒时间戳）
    pub transfer_time: Option<i64>,
    /// 原所有者ID类型
    pub old_owner_type: Option<String>,
    /// 原所有者ID
    pub old_owner_id: Option<String>}
/// 转移所有者响应,
#[derive(Debug, Clone)]
pub struct TransferOwnerResponse {
    /// 转移结果
    pub member: TransferResult,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 转移所有者,
pub async fn transfer_owner(
    request: TransferOwnerRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<TransferOwnerResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
api_req.api_path = format!(,
        "{}?type={}",
        EndpointBuilder::replace_param(
            DRIVE_V1_PERMISSIONS_MEMBERS_TRANSFER_OWNER,
            "token",
            &request.token
        ),
        request.obj_type,
);
    // 添加其他查询参数,
let mut query_params = Vec::new();
    if let Some(remove_old_owner) = request.remove_old_owner {
        query_params.push(format!("remove_old_owner={remove_old_owner}"));
if let Some(need_notification) = request.need_notification {,
        query_params.push(format!("need_notification={need_notification}"));
if !query_params.is_empty() {,
        api_req.set_api_path(format!("{}&{}", api_req.api_path, query_params.join("&")));

    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

impl TransferResult {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取新所有者信息,
    pub fn new_owner_info(&self) -> String {
        format!("新所有者: {} ({})", self.member_id, self.member_type),
/// 获取原所有者信息,
    pub fn old_owner_info(&self) -> Option<String> {
        if let (Some(old_type), Some(old_id)) = (&self.old_owner_type, &self.old_owner_id) {
            Some(format!("原所有者: {old_id} ({old_type})")),
} else {,
None}
/// 获取转移摘要,
    pub fn w+.*{
let mut parts = vec![self.new_owner_info()];
        if let Some(old_info) = self.old_owner_info() {,
parts.push(old_info);
        }
if let Some(time_info) = self.transfer_time_formatted() {,
            parts.push(time_info);

        parts.join(", "),
impl TransferOwnerResponse {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
#[test]
    fn test_transfer_owner_request_builder() {
let request = TransferOwnerRequest::builder(),
            .token()
.as_doc()
            .to_user()
.remove_current_owner()
            .with_notification()
.build();
        assert_eq!(request.token, "doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");
        assert_eq!(request.member_type, "user");
        assert_eq!(request.member_id, "user123");
        assert_eq!(request.remove_old_owner, Some(true));
        assert_eq!(request.need_notification, Some(true));
#[test]
    ,
        let request = TransferOwnerRequest::to_user("doccnxxxxxx", "doc", "user123");
        assert_eq!(request.member_type, "user");
        assert_eq!(request.member_id, "user123");
