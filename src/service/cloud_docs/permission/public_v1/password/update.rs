use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::{,
use crate::SDKResult;    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::{cloud_docs::*, EndpointBuilder}
    http::Transport,
    query_params::QueryParams,
    req_option::RequestOption,
    SDKResult,
};
/// 刷新密码请求,
#[derive(Debug, Clone)]
pub struct UpdatePasswordRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token,
#[serde(skip)]
    token: String,
    /// 文档类型,
#[serde(skip)]
    obj_type: String,
    /// 新密码
    password: String}
impl UpdatePasswordRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct UpdatePasswordRequestBuilder {
    request: UpdatePasswordRequest}
impl UpdatePasswordRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 增强密码（在原密码基础上增加复杂度）,
    pub fn enhance_password(mut self, base_password: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}        use rand::{thread_rng, Rng};
let base = base_password.to_string();
        let suffix: u32 = thread_rng().gen_range(10..100);
        self.request.password = format!("{base}@{suffix}");
self,
    }
pub fn w+.*{
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
self.request}
/// 密码更新结果,
#[derive(Debug, Clone)]
pub struct PasswordUpdateResult {
    /// 新密码
    pub password: String,
    /// 更新时间
    pub update_time: Option<i64>,
    /// 过期时间（如果有）
    pub expire_time: Option<i64>,
    /// 上次密码（脱敏显示，如果有）
    pub previous_password_hint: Option<String>}
/// 刷新密码响应,
#[derive(Debug, Clone)]
pub struct UpdatePasswordResponse {
    /// 密码更新信息
    pub password: PasswordUpdateResult,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 刷新密码,
pub async fn update_password(
    request: UpdatePasswordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<UpdatePasswordResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::PUT);
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

impl PasswordUpdateResult {
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
            && password.chars().any(|c| !c.is_ascii_alphanumeric()),
{,
            "很强"} else if password.chars().any(|c| c.is_ascii_digit()),
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
/// 密码类型,
    pub fn w+.*{
let password = &self.password;
        if password.chars().all(|c| c.is_ascii_digit()) {,
"纯数字"} else if password.chars().all(|c| c.is_ascii_alphabetic()) {,
"纯字母"} else if password.chars().any(|c| !c.is_ascii_alphanumeric()) {,
"包含特殊字符"} else {,
"字母数字组合"}
/// 获取密码变更摘要,
    pub fn w+.*{
let mut info = Vec::new();
        info.push(format!("新密码长度: {}", self.password_length()));
        info.push(format!("强度: {}", self.password_strength()));
        info.push(format!("类型: {}", self.password_type()));
if let Some(ref hint) = self.previous_password_hint {,
            info.push(format!("原密码: {hint}"));

        info.join(", "),
/// 安全性改进评估,
    pub fn w+.*{
match self.password_strength() {,
            "很强" => "密码安全性显著提升",
            "强" => "密码安全性有所提升",
            "中等" => "密码安全性一般",
            "弱" => "建议使用更强的密码",
            _ => "未知"}
impl UpdatePasswordResponse {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 安全性评估,
    pub fn w+.*{
format!(,
            "安全评估: {} - {}",
            self.password.password_strength(),
            self.password.security_improvement(),
),
    }
/// 安全建议,
    pub fn w+.*{
let mut recommendations = Vec::new();
        if self.password.is_numeric_password() {,
recommendations.push("建议使用包含字母、数字和特殊字符的混合密码".to_string());
        }
if self.password.password_length() < 8 {,
            recommendations.push("建议使用8位以上的密码".to_string());
if self.password.password_strength() == "弱" {,
            recommendations.push("当前密码强度较弱，建议立即更换为更复杂的密码".to_string());
recommendations.push("定期更换密码以提高安全性".to_string());
        recommendations.push("请妥善保管新密码".to_string());
if self.password.has_expire_time() {,
            recommendations.push("密码有过期时间，请注意及时更新".to_string());
recommendations,
    }
/// 获取操作建议,
    pub fn w+.*{
let mut tips = Vec::new();
        tips.push("新密码已生效，旧密码立即失效".to_string());
tips.push("请及时通知相关人员密码变更".to_string());
        tips.push("建议在安全环境下记录新密码".to_string());
if self.password.password_type() == "包含特殊字符" {,
            tips.push("输入密码时请注意特殊字符的准确性".to_string());
tips,
    }
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
#[test]
    fn test_update_password_request_builder() {
let request = UpdatePasswordRequest::builder(),
            .token()
.as_doc()
            .password()
.build();
        assert_eq!(request.token, "doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");
        assert_eq!(request.password, "newpassword123");
#[test]
    ,
        let request = UpdatePasswordRequest::for_doc("doccnxxxxxx", "newpass456");
        assert_eq!(request.obj_type, "doc");
        assert_eq!(request.password, "newpass456");

        let request = UpdatePasswordRequest::for_sheet("shtcnxxxxxx", "sheet789");
        assert_eq!(request.obj_type, "sheet");
        assert_eq!(request.password, "sheet789");
#[test]
    fn test_password_builder_methods() {
let request = UpdatePasswordRequest::builder(),
            .token()
.as_doc()
            .simple_password()
.build();
        assert_eq!(request.password, "789012");
let request = UpdatePasswordRequest::builder(),
            .token()
.as_doc()
            .enhance_password()
.build();
        assert!(request.password.starts_with("base@"));
assert!(request.password.len() > 5);
    }
#[test]
    fn test_password_update_result_methods() {
let result = PasswordUpdateResult {,
            password: "Complex@123".to_string(),
            update_time: Some(1234567890),
            expire_time: Some(1234567999),
            previous_password_hint: Some("old****".to_string())};
assert!(result.has_update_time());
        assert!(result.has_expire_time());
assert!(result.has_previous_hint());
        assert!(!result.is_numeric_password());
        assert_eq!(result.password_length(), 11);
        assert_eq!(result.password_type(), "包含特殊字符");
        assert_eq!(result.password_strength(), "很强");
        assert_eq!(result.security_improvement(), "密码安全性显著提升");
let weak_result = PasswordUpdateResult {,
            password: "123".to_string(),
            update_time: None,
            expire_time: None,
            previous_password_hint: None};

        assert_eq!(weak_result.password_strength(), "弱");
        assert_eq!(weak_result.security_improvement(), "建议使用更强的密码");
