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

/// 开启密码保护请求
#[derive(Debug, Serialize, Default, Clone)]
pub struct CreatePasswordRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token
    #[serde(skip)]
    token: String,
    /// 文档类型
    #[serde(skip)]
    obj_type: String,
    /// 密码
    password: String,
}

impl CreatePasswordRequest {
    pub fn builder() -> CreatePasswordRequestBuilder {
        CreatePasswordRequestBuilder::default()
    }

    pub fn new(token: impl ToString, obj_type: impl ToString, password: impl ToString) -> Self {
        Self {
            token: token.to_string(),
            obj_type: obj_type.to_string(),
            password: password.to_string(),
            ..Default::default()
        }
    }

    /// 为文档开启密码保护
    pub fn for_doc(token: impl ToString, password: impl ToString) -> Self {
        Self::new(token, "doc", password)
    }

    /// 为电子表格开启密码保护
    pub fn for_sheet(token: impl ToString, password: impl ToString) -> Self {
        Self::new(token, "sheet", password)
    }

    /// 为多维表格开启密码保护
    pub fn for_bitable(token: impl ToString, password: impl ToString) -> Self {
        Self::new(token, "bitable", password)
    }

    /// 为知识库开启密码保护
    pub fn for_wiki(token: impl ToString, password: impl ToString) -> Self {
        Self::new(token, "wiki", password)
    }
}

#[derive(Default)]
pub struct CreatePasswordRequestBuilder {
    request: CreatePasswordRequest,
}

impl CreatePasswordRequestBuilder {
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

    /// 设置密码
    pub fn password(mut self, password: impl ToString) -> Self {
        self.request.password = password.to_string();
        self
    }

    /// 设置简单密码（6位数字）
    pub fn simple_password(mut self, digits: u32) -> Self {
        self.request.password = format!("{:06}", digits % 1000000);
        self
    }

    /// 生成随机密码
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

    pub fn build(mut self) -> CreatePasswordRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 密码创建结果
#[derive(Debug, Deserialize)]
pub struct PasswordResult {
    /// 密码
    pub password: String,
    /// 创建时间
    pub create_time: Option<i64>,
    /// 过期时间（如果有）
    pub expire_time: Option<i64>,
}

/// 开启密码保护响应
#[derive(Debug, Deserialize)]
pub struct CreatePasswordResponse {
    /// 密码信息
    pub password: PasswordResult,
}

impl ApiResponseTrait for CreatePasswordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 开启密码保护
pub async fn create_password(
    request: CreatePasswordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<CreatePasswordResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::POST;
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

impl PasswordResult {
    /// 是否有创建时间
    pub fn has_create_time(&self) -> bool {
        self.create_time.is_some()
    }

    /// 是否有过期时间
    pub fn has_expire_time(&self) -> bool {
        self.expire_time.is_some()
    }

    /// 获取创建时间格式化字符串
    pub fn create_time_formatted(&self) -> Option<String> {
        self.create_time
            .map(|timestamp| format!("创建时间: {timestamp}"))
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

    /// 获取密码信息摘要
    pub fn password_summary(&self) -> String {
        format!(
            "密码长度: {}, 强度: {}, 类型: {}",
            self.password_length(),
            self.password_strength(),
            if self.is_numeric_password() {
                "纯数字"
            } else {
                "混合字符"
            }
        )
    }
}

impl CreatePasswordResponse {
    /// 获取密码信息
    pub fn password_info(&self) -> &PasswordResult {
        &self.password
    }

    /// 获取实际密码
    pub fn password_value(&self) -> &str {
        &self.password.password
    }

    /// 是否创建成功
    pub fn is_created(&self) -> bool {
        !self.password.password.is_empty()
    }

    /// 获取创建摘要
    pub fn creation_summary(&self) -> String {
        if self.is_created() {
            format!("密码保护已开启 - {}", self.password.password_summary())
        } else {
            "密码保护开启失败".to_string()
        }
    }

    /// 安全提醒
    pub fn security_tips(&self) -> Vec<String> {
        let mut tips = Vec::new();

        if self.password.is_numeric_password() {
            tips.push("建议使用包含字母和数字的混合密码".to_string());
        }

        if self.password.password_length() < 8 {
            tips.push("建议使用8位以上的密码".to_string());
        }

        tips.push("请妥善保管密码，遗失后需要重新设置".to_string());

        if self.password.has_expire_time() {
            tips.push("密码有过期时间，请注意及时更新".to_string());
        }

        tips
    }

    /// 获取操作建议
    pub fn operation_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        recommendations.push("建议定期更换密码".to_string());
        recommendations.push("不要在不安全的环境中输入密码".to_string());
        recommendations.push("可以设置更复杂的密码提高安全性".to_string());

        if self.password.password_strength() == "弱" {
            recommendations.push("当前密码强度较弱，建议立即更换".to_string());
        }

        recommendations
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_create_password_request_builder() {
        let request = CreatePasswordRequest::builder()
            .token("doccnxxxxxx")
            .as_doc()
            .password("123456")
            .build();

        assert_eq!(request.token, "doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");
        assert_eq!(request.password, "123456");
    }

    #[test]
    fn test_convenience_methods() {
        let request = CreatePasswordRequest::for_doc("doccnxxxxxx", "password123");
        assert_eq!(request.obj_type, "doc");
        assert_eq!(request.password, "password123");

        let request = CreatePasswordRequest::for_sheet("shtcnxxxxxx", "sheet456");
        assert_eq!(request.obj_type, "sheet");
        assert_eq!(request.password, "sheet456");
    }

    #[test]
    fn test_password_builder_methods() {
        let request = CreatePasswordRequest::builder()
            .token("doccnxxxxxx")
            .as_doc()
            .simple_password(123456)
            .build();

        assert_eq!(request.password, "123456");
    }

    #[test]
    fn test_password_result_methods() {
        let result = PasswordResult {
            password: "password123".to_string(),
            create_time: Some(1234567890),
            expire_time: None,
        };

        assert!(result.has_create_time());
        assert!(!result.has_expire_time());
        assert!(!result.is_numeric_password());
        assert_eq!(result.password_length(), 11);
        assert_eq!(result.password_strength(), "强");

        let numeric_result = PasswordResult {
            password: "123456".to_string(),
            create_time: None,
            expire_time: None,
        };

        assert!(numeric_result.is_numeric_password());
        assert_eq!(numeric_result.password_strength(), "中等");
    }
}
