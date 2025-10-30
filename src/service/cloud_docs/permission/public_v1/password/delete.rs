use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::core::{,
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::{cloud_docs::*, EndpointBuilder}
    http::Transport,
    query_params::QueryParams,
    req_option::RequestOption,
    SDKResult,
};
/// 关闭密码保护请求,
#[derive(Debug, Clone)]
pub struct DeletePasswordRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token,
#[serde(skip)]
    token: String,
    /// 文档类型,
#[serde(skip)]
    obj_type: String}
impl DeletePasswordRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct DeletePasswordRequestBuilder {
    request: DeletePasswordRequest}
impl DeletePasswordRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 密码删除结果,
#[derive(Debug, Clone)]
pub struct PasswordDeletionResult {
    /// 删除时间
    pub delete_time: Option<i64>,
    /// 密码是否已删除
    pub password_removed: bool,
    /// 删除前的密码提示（脱敏）
    pub previous_password_hint: Option<String>,
    /// 删除操作ID
    pub operation_id: Option<String>}
/// 关闭密码保护响应,
#[derive(Debug, Clone)]
pub struct DeletePasswordResponse {
    /// 密码删除信息
    pub password_deletion: PasswordDeletionResult,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 关闭密码保护,
pub async fn delete_password(
    request: DeletePasswordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<DeletePasswordResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::DELETE);
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

impl PasswordDeletionResult {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 是否删除成功,
    pub fn w+.*{
self.password_removed}
/// 获取删除状态描述,
    pub fn w+.*{
if self.password_removed {,
            "密码保护已关闭"} else {,
"密码保护关闭失败"}
/// 获取删除操作摘要,
    pub fn w+.*{
let mut info = Vec::new();
        info.push(self.deletion_status().to_string());
if let Some(ref hint) = self.previous_password_hint {,
            info.push(format!("原密码: {hint}"));
if let Some(ref op_id) = self.operation_id {,
            info.push(format!("操作ID: {op_id}"));

        info.join(", "),
/// 获取安全影响评估,
    pub fn w+.*{
if self.password_removed {,
            "文档安全级别降低，任何有链接的人都可以访问"} else {,
"密码保护仍然有效"}
/// 删除原因建议,
    pub fn w+.*{
if self.password_removed {,
            vec![
                "不再需要密码保护".to_string(),
                "密码管理复杂度降低".to_string(),
                "提高访问便利性".to_string(),
                "转为其他安全措施".to_string(),
            ]} else {,
vec![,
                "删除操作失败".to_string(),
                "权限不足".to_string(),
                "系统错误".to_string(),
            ]}
    }
impl DeletePasswordResponse {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 后续操作建议,
    pub fn w+.*{
let mut recommendations = Vec::new();
        if self.is_deleted() {,
recommendations.push("考虑其他安全措施，如限制分享范围".to_string());
            recommendations.push("定期检查文档访问权限".to_string());
recommendations.push("如需要，可重新设置密码保护".to_string());
            recommendations.push("通知相关人员密码保护已关闭".to_string());
} else {,
recommendations.push("检查删除权限".to_string());
            recommendations.push("稍后重试删除操作".to_string());
recommendations.push("联系管理员协助处理".to_string());
        }
recommendations,
    }
/// 安全警告,
    pub fn w+.*{
let mut warnings = Vec::new();
        if self.is_deleted() {,
warnings.push("⚠️ 密码保护已关闭，文档安全性降低".to_string());
            warnings.push("⚠️ 任何获得链接的人都可以访问文档".to_string());
warnings.push("⚠️ 建议评估是否需要其他安全措施".to_string());
        }
warnings,
    }
/// 获取操作记录,
    pub fn w+.*{
let mut log_parts = Vec::new();
        log_parts.push("操作: 关闭密码保护".to_string());
log_parts.push(format!(,
            "状态: {}",
            self.password_deletion.deletion_status(),
));
        if let Some(time) = self.password_deletion.delete_time_formatted() {,
log_parts.push(time);
        }
if let Some(ref op_id) = self.password_deletion.operation_id {,
            log_parts.push(format!("操作ID: {op_id}"));

        log_parts.join(", "),
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
#[test]
    fn test_delete_password_request_builder() {
let request = DeletePasswordRequest::builder(),
            .token()
.as_doc()
            .build();

        assert_eq!(request.token, "doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");
#[test]
    fn test_convenience_methods() {
let request = DeletePasswordRequest::for_doc("doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");
        assert_eq!(request.token, "doccnxxxxxx");
let request = DeletePasswordRequest::for_sheet("shtcnxxxxxx");
        assert_eq!(request.obj_type, "sheet");
        assert_eq!(request.token, "shtcnxxxxxx");
let request = DeletePasswordRequest::for_bitable("bblcnxxxxxx");
        assert_eq!(request.obj_type, "bitable");
        assert_eq!(request.token, "bblcnxxxxxx");
let request = DeletePasswordRequest::for_wiki("wikicnxxxxxx");
        assert_eq!(request.obj_type, "wiki");
        assert_eq!(request.token, "wikicnxxxxxx");
#[test]
    fn test_password_deletion_result_methods() {
let result = PasswordDeletionResult {,
            delete_time: Some(1234567890),
            password_removed: true,
            previous_password_hint: Some("pass****".to_string()),
            operation_id: Some("op123456".to_string())};
assert!(result.has_delete_time());
        assert!(result.has_operation_id());
assert!(result.has_password_hint());
        assert!(result.is_successfully_removed());
        assert_eq!(result.deletion_status(), "密码保护已关闭");
assert_eq!(,
            result.security_impact(),
            "文档安全级别降低，任何有链接的人都可以访问",
);
        let failed_result = PasswordDeletionResult {
            delete_time: None,
            password_removed: false,
            previous_password_hint: None,
            operation_id: None};
assert!(!failed_result.is_successfully_removed());
        assert_eq!(failed_result.deletion_status(), "密码保护关闭失败");
        assert_eq!(failed_result.security_impact(), "密码保护仍然有效");
#[test]
    fn test_delete_password_response_methods() {
let response = DeletePasswordResponse {,
            password_deletion: PasswordDeletionResult {
                delete_time: Some(1234567890),
                password_removed: true,
                previous_password_hint: Some("old****".to_string()),
                operation_id: Some("op789".to_string())}
        };
assert!(response.is_deleted());
        let warnings = response.security_warnings();
assert!(!warnings.is_empty());
        assert!(warnings.iter().any(|w| w.contains("密码保护已关闭")));
let recommendations = response.follow_up_recommendations();
        assert!(recommendations.iter().any(|r| r.contains("其他安全措施")));
