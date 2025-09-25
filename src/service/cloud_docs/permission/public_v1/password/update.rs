use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::{cloud_docs::*, EndpointBuilder},
    http::Transport,
    query_params::QueryParams,
    req_option::RequestOption,
    SDKResult,
};

/// 刷新密码请求
#[derive(Debug, Serialize, Default, Clone)]
pub struct UpdatePasswordRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token
    #[serde(skip)]
    token: String,
    /// 文档类型
    #[serde(skip)]
    obj_type: String,
    /// 新密码
    password: String,
}

impl UpdatePasswordRequest {
    pub fn builder() -> UpdatePasswordRequestBuilder {
        UpdatePasswordRequestBuilder::default()
    }

    pub fn new(token: impl ToString, obj_type: impl ToString, password: impl ToString) -> Self {
        Self {
            token: token.to_string(),
            obj_type: obj_type.to_string(),
            password: password.to_string(),
            ..Default::default()
        }
    }

    /// 刷新文档密码
    pub fn for_doc(token: impl ToString, password: impl ToString) -> Self {
        Self::new(token, "doc", password)
    }

    /// 刷新电子表格密码
    pub fn for_sheet(token: impl ToString, password: impl ToString) -> Self {
        Self::new(token, "sheet", password)
    }

    /// 刷新多维表格密码
    pub fn for_bitable(token: impl ToString, password: impl ToString) -> Self {
        Self::new(token, "bitable", password)
    }

    /// 刷新知识库密码
    pub fn for_wiki(token: impl ToString, password: impl ToString) -> Self {
        Self::new(token, "wiki", password)
    }
}

#[derive(Default)]
pub struct UpdatePasswordRequestBuilder {
    request: UpdatePasswordRequest,
}

impl UpdatePasswordRequestBuilder {
    /// 文档token
    pub fn token(mut self, token: impl ToString) -> Self {
        self.request.token = token.to_string();
        self
    }

    /// 文档类型
    pub fn obj_type(mut self, obj_type: impl ToString) -> Self {
        self.request.obj_type = obj_type.to_string();
        self
    }

    /// 设置为文档类型
    pub fn as_doc(mut self) -> Self {
        self.request.obj_type = "doc".to_string();
        self
    }

    /// 设置为电子表格类型
    pub fn as_sheet(mut self) -> Self {
        self.request.obj_type = "sheet".to_string();
        self
    }

    /// 设置为多维表格类型
    pub fn as_bitable(mut self) -> Self {
        self.request.obj_type = "bitable".to_string();
        self
    }

    /// 设置为知识库类型
    pub fn as_wiki(mut self) -> Self {
        self.request.obj_type = "wiki".to_string();
        self
    }

    /// 设置新密码
    pub fn password(mut self, password: impl ToString) -> Self {
        self.request.password = password.to_string();
        self
    }

    /// 设置新的简单密码（6位数字）
    pub fn simple_password(mut self, digits: u32) -> Self {
        self.request.password = format!("{:06}", digits % 1000000);
        self
    }

    /// 生成新的随机密码
    pub fn random_password(mut self, length: usize) -> Self {
        use rand::{distributions::Alphanumeric, thread_rng, Rng};

        let password: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length.clamp(6, 32))
            .map(char::from)
            .collect();

        self.request.password = password;
        self
    }

    /// 增强密码（在原密码基础上增加复杂度）
    pub fn enhance_password(mut self, base_password: impl ToString) -> Self {
        use rand::{thread_rng, Rng};

        let base = base_password.to_string();
        let suffix: u32 = thread_rng().gen_range(10..100);
        self.request.password = format!("{base}@{suffix}");
        self
    }

    pub fn build(mut self) -> UpdatePasswordRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 密码更新结果
#[derive(Debug, Deserialize)]
pub struct PasswordUpdateResult {
    /// 新密码
    pub password: String,
    /// 更新时间
    pub update_time: Option<i64>,
    /// 过期时间（如果有）
    pub expire_time: Option<i64>,
    /// 上次密码（脱敏显示，如果有）
    pub previous_password_hint: Option<String>,
}

/// 刷新密码响应
#[derive(Debug, Deserialize)]
pub struct UpdatePasswordResponse {
    /// 密码更新信息
    pub password: PasswordUpdateResult,
}

impl ApiResponseTrait for UpdatePasswordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 刷新密码
pub async fn update_password(
    request: UpdatePasswordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<UpdatePasswordResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::PUT;
    api_req.api_path = EndpointBuilder::replace_param(
        DRIVE_V1_PERMISSIONS_PUBLIC_PASSWORD,
        "token",
        &request.token,
    );

    // 添加查询参数
    api_req
        .query_params
        .insert(QueryParams::TYPE, request.obj_type);

    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

impl PasswordUpdateResult {
    /// 是否有更新时间
    pub fn has_update_time(&self) -> bool {
        self.update_time.is_some()
    }

    /// 是否有过期时间
    pub fn has_expire_time(&self) -> bool {
        self.expire_time.is_some()
    }

    /// 是否有上次密码提示
    pub fn has_previous_hint(&self) -> bool {
        self.previous_password_hint.is_some()
    }

    /// 获取更新时间格式化字符串
    pub fn update_time_formatted(&self) -> Option<String> {
        self.update_time
            .map(|timestamp| format!("更新时间: {timestamp}"))
    }

    /// 获取过期时间格式化字符串
    pub fn expire_time_formatted(&self) -> Option<String> {
        self.expire_time
            .map(|timestamp| format!("过期时间: {timestamp}"))
    }

    /// 密码强度评估
    pub fn password_strength(&self) -> &'static str {
        let password = &self.password;
        let length = password.len();

        if length < 6 {
            "弱"
        } else if length < 8 {
            "中等"
        } else if password.chars().any(|c| c.is_ascii_digit())
            && password.chars().any(|c| c.is_ascii_alphabetic())
            && password.chars().any(|c| !c.is_ascii_alphanumeric())
        {
            "很强"
        } else if password.chars().any(|c| c.is_ascii_digit())
            && password.chars().any(|c| c.is_ascii_alphabetic())
        {
            "强"
        } else {
            "中等"
        }
    }

    /// 是否为数字密码
    pub fn is_numeric_password(&self) -> bool {
        self.password.chars().all(|c| c.is_ascii_digit())
    }

    /// 密码长度
    pub fn password_length(&self) -> usize {
        self.password.len()
    }

    /// 密码类型
    pub fn password_type(&self) -> &'static str {
        let password = &self.password;

        if password.chars().all(|c| c.is_ascii_digit()) {
            "纯数字"
        } else if password.chars().all(|c| c.is_ascii_alphabetic()) {
            "纯字母"
        } else if password.chars().any(|c| !c.is_ascii_alphanumeric()) {
            "包含特殊字符"
        } else {
            "字母数字组合"
        }
    }

    /// 获取密码变更摘要
    pub fn change_summary(&self) -> String {
        let mut info = Vec::new();

        info.push(format!("新密码长度: {}", self.password_length()));
        info.push(format!("强度: {}", self.password_strength()));
        info.push(format!("类型: {}", self.password_type()));

        if let Some(ref hint) = self.previous_password_hint {
            info.push(format!("原密码: {hint}"));
        }

        info.join(", ")
    }

    /// 安全性改进评估
    pub fn security_improvement(&self) -> &'static str {
        match self.password_strength() {
            "很强" => "密码安全性显著提升",
            "强" => "密码安全性有所提升",
            "中等" => "密码安全性一般",
            "弱" => "建议使用更强的密码",
            _ => "未知",
        }
    }
}

impl UpdatePasswordResponse {
    /// 获取密码更新信息
    pub fn password_info(&self) -> &PasswordUpdateResult {
        &self.password
    }

    /// 获取新密码
    pub fn new_password(&self) -> &str {
        &self.password.password
    }

    /// 是否更新成功
    pub fn is_updated(&self) -> bool {
        !self.password.password.is_empty()
    }

    /// 获取更新摘要
    pub fn update_summary(&self) -> String {
        if self.is_updated() {
            format!("密码已更新 - {}", self.password.change_summary())
        } else {
            "密码更新失败".to_string()
        }
    }

    /// 安全性评估
    pub fn security_assessment(&self) -> String {
        format!(
            "安全评估: {} - {}",
            self.password.password_strength(),
            self.password.security_improvement()
        )
    }

    /// 安全建议
    pub fn security_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        if self.password.is_numeric_password() {
            recommendations.push("建议使用包含字母、数字和特殊字符的混合密码".to_string());
        }

        if self.password.password_length() < 8 {
            recommendations.push("建议使用8位以上的密码".to_string());
        }

        if self.password.password_strength() == "弱" {
            recommendations.push("当前密码强度较弱，建议立即更换为更复杂的密码".to_string());
        }

        recommendations.push("定期更换密码以提高安全性".to_string());
        recommendations.push("请妥善保管新密码".to_string());

        if self.password.has_expire_time() {
            recommendations.push("密码有过期时间，请注意及时更新".to_string());
        }

        recommendations
    }

    /// 获取操作建议
    pub fn operation_tips(&self) -> Vec<String> {
        let mut tips = Vec::new();

        tips.push("新密码已生效，旧密码立即失效".to_string());
        tips.push("请及时通知相关人员密码变更".to_string());
        tips.push("建议在安全环境下记录新密码".to_string());

        if self.password.password_type() == "包含特殊字符" {
            tips.push("输入密码时请注意特殊字符的准确性".to_string());
        }

        tips
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_update_password_request_builder() {
        let request = UpdatePasswordRequest::builder()
            .token("doccnxxxxxx")
            .as_doc()
            .password("newpassword123")
            .build();

        assert_eq!(request.token, "doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");
        assert_eq!(request.password, "newpassword123");
    }

    #[test]
    fn test_convenience_methods() {
        let request = UpdatePasswordRequest::for_doc("doccnxxxxxx", "newpass456");
        assert_eq!(request.obj_type, "doc");
        assert_eq!(request.password, "newpass456");

        let request = UpdatePasswordRequest::for_sheet("shtcnxxxxxx", "sheet789");
        assert_eq!(request.obj_type, "sheet");
        assert_eq!(request.password, "sheet789");
    }

    #[test]
    fn test_password_builder_methods() {
        let request = UpdatePasswordRequest::builder()
            .token("doccnxxxxxx")
            .as_doc()
            .simple_password(789012)
            .build();

        assert_eq!(request.password, "789012");

        let request = UpdatePasswordRequest::builder()
            .token("doccnxxxxxx")
            .as_doc()
            .enhance_password("base")
            .build();

        assert!(request.password.starts_with("base@"));
        assert!(request.password.len() > 5);
    }

    #[test]
    fn test_password_update_result_methods() {
        let result = PasswordUpdateResult {
            password: "Complex@123".to_string(),
            update_time: Some(1234567890),
            expire_time: Some(1234567999),
            previous_password_hint: Some("old****".to_string()),
        };

        assert!(result.has_update_time());
        assert!(result.has_expire_time());
        assert!(result.has_previous_hint());
        assert!(!result.is_numeric_password());
        assert_eq!(result.password_length(), 11);
        assert_eq!(result.password_type(), "包含特殊字符");
        assert_eq!(result.password_strength(), "很强");
        assert_eq!(result.security_improvement(), "密码安全性显著提升");

        let weak_result = PasswordUpdateResult {
            password: "123".to_string(),
            update_time: None,
            expire_time: None,
            previous_password_hint: None,
        };

        assert_eq!(weak_result.password_strength(), "弱");
        assert_eq!(weak_result.security_improvement(), "建议使用更强的密码");
    }
}
