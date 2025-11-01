use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::core::{,
use crate::core::SDKResult;    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::{cloud_docs::*, EndpointBuilder}
    http::Transport,
    query_params::QueryParams,
    req_option::RequestOption,
    SDKResult,
};
/// 开启密码保护请求,
#[derive(Debug, Clone)]
pub struct CreatePasswordRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token,
#[serde(skip)]
    token: String,
    /// 文档类型,
#[serde(skip)]
    obj_type: String,
    /// 密码
    password: String}
impl CreatePasswordRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct CreatePasswordRequestBuilder {
    request: CreatePasswordRequest}
impl CreatePasswordRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}pub fn w+.*{
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
self.request}
/// 密码创建结果,
#[derive(Debug, Clone)]
pub struct PasswordResult {
    /// 密码
    pub password: String,
    /// 创建时间
    pub create_time: Option<i64>,
    /// 过期时间（如果有）
    pub expire_time: Option<i64>}
/// 开启密码保护响应,
#[derive(Debug, Clone)]
pub struct CreatePasswordResponse {
    /// 密码信息
    pub password: PasswordResult,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 开启密码保护,
pub async fn create_password(
    request: CreatePasswordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<CreatePasswordResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
api_req.set_api_path(EndpointBuilder::replace_param(,
        DRIVE_V1_PERMISSIONS_PUBLIC_PASSWORD,
        "token",
        &request.token,
    ));
// 添加查询参数,
    api_req
.query_params
        .insert(QueryParams::TYPE, request.obj_type);

    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp)}

impl PasswordResult {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取过期时间格式化字符串,
    pub fn w+.*{
self.expire_time,
            .map(|timestamp| format!("过期时间: {timestamp}")),
/// 密码强度评估,
    pub fn w+.*{
let password = &self.password;
        let length = password.len();
if length < 6 {,
            "弱"} else if length < 8 {,
"中等"} else if password.chars().any(|c| c.is_ascii_digit()),
&& password.chars().any(|c| c.is_ascii_alphabetic()),
        {,
"强"} else {,
"中等"}
/// 是否为数字密码,
    pub fn w+.*{
self.password.chars().all(|c| c.is_ascii_digit())}
/// 密码长度,
    pub fn w+.*{
self.password.len()}
/// 获取密码信息摘要,
    pub fn w+.*{
format!(,
            "密码长度: {} 强度: {} 类型: {}",
            self.password_length(),
            self.password_strength(),
            if self.is_numeric_password() {,
"纯数字"} else {,
"混合字符"}
),
    }
impl CreatePasswordResponse {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 安全提醒,
    pub fn w+.*{
let mut tips = Vec::new();
        if self.password.is_numeric_password() {,
tips.push("建议使用包含字母和数字的混合密码".to_string());
        }
if self.password.password_length() < 8 {,
            tips.push("建议使用8位以上的密码".to_string());
tips.push("请妥善保管密码，遗失后需要重新设置".to_string());
        if self.password.has_expire_time() {,
tips.push("密码有过期时间，请注意及时更新".to_string());
        }
tips,
    }
/// 获取操作建议,
    pub fn w+.*{
let mut recommendations = Vec::new();
        recommendations.push("建议定期更换密码".to_string());
recommendations.push("不要在不安全的环境中输入密码".to_string());
        recommendations.push("可以设置更复杂的密码提高安全性".to_string());
if self.password.password_strength() == "弱" {,
            recommendations.push("当前密码强度较弱，建议立即更换".to_string());
recommendations,
    }
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
#[test]
    fn test_create_password_request_builder() {
let request = CreatePasswordRequest::builder(),
            .token()
.as_doc()
            .password()
.build();
        assert_eq!(request.token, "doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");
        assert_eq!(request.password, "123456");
#[test]
    ,
        let request = CreatePasswordRequest::for_doc("doccnxxxxxx", "password123");
        assert_eq!(request.obj_type, "doc");
        assert_eq!(request.password, "password123");

        let request = CreatePasswordRequest::for_sheet("shtcnxxxxxx", "sheet456");
        assert_eq!(request.obj_type, "sheet");
        assert_eq!(request.password, "sheet456");
#[test]
    fn test_password_builder_methods() {
let request = CreatePasswordRequest::builder(),
            .token()
.as_doc()
            .simple_password()
.build();
        assert_eq!(request.password, "123456");
#[test]
    fn test_password_result_methods() {
let result = PasswordResult {,
            password: "password123".to_string(),
            create_time: Some(1234567890),
            expire_time: None};
assert!(result.has_create_time());
        assert!(!result.has_expire_time());
assert!(!result.is_numeric_password());
        assert_eq!(result.password_length(), 11);
        assert_eq!(result.password_strength(), "强");
let numeric_result = PasswordResult {,
            password: "123456".to_string(),
            create_time: None,
            expire_time: None};
assert!(numeric_result.is_numeric_password());
        assert_eq!(numeric_result.password_strength(), "中等");
