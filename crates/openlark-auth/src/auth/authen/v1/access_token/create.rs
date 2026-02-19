//! 获取 user_access_token（v1版本） API
use crate::models::authen::{UserAccessTokenResponse, UserAccessTokenV1Request};
///
/// API文档: https://open.feishu.cn/document/server-docs/user-authentication/access-token/access_token
///
/// 根据登录预授权码获取 user_access_token
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 用户访问令牌请求（v1版本）
pub struct UserAccessTokenV1Builder {
    grant_code: String,
    app_id: String,
    app_secret: String,
    /// 配置信息
    config: Config,
}

/// 用户访问令牌响应（v1版本）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserAccessTokenV1ResponseData {
    /// 用户访问令牌响应
    pub data: UserAccessTokenResponse,
}

impl ApiResponseTrait for UserAccessTokenV1ResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UserAccessTokenV1Builder {
    /// 创建 access_token 请求
    pub fn new(config: Config) -> Self {
        Self {
            grant_code: String::new(),
            app_id: String::new(),
            app_secret: String::new(),
            config,
        }
    }

    /// 设置授权码
    pub fn grant_code(mut self, grant_code: impl Into<String>) -> Self {
        self.grant_code = grant_code.into();
        self
    }

    /// 设置应用ID
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
    }

    /// 设置应用密钥
    pub fn app_secret(mut self, app_secret: impl Into<String>) -> Self {
        self.app_secret = app_secret.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UserAccessTokenV1ResponseData> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UserAccessTokenV1ResponseData> {
        // 验证必填字段
        validate_required!(self.grant_code, "授权码不能为空");
        validate_required!(self.app_id, "应用ID不能为空");
        validate_required!(self.app_secret, "应用密钥不能为空");

        // 🚀 使用新的enum+builder系统生成API端点
        use crate::common::api_endpoints::AuthenApiV1;
        let api_endpoint = AuthenApiV1::AccessToken;

        // 构建请求体
        let request_body = UserAccessTokenV1Request {
            grant_code: self.grant_code.clone(),
            app_id: self.app_id.clone(),
            app_secret: self.app_secret.clone(),
        };

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<UserAccessTokenV1ResponseData> =
            ApiRequest::post(api_endpoint.path()).body(serde_json::to_value(&request_body)?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取 user_access_token v1", "响应数据为空")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
    }

    #[test]
    fn test_user_access_token_v1_builder_new() {
        let config = create_test_config();
        let builder = UserAccessTokenV1Builder::new(config);
        assert!(builder.grant_code.is_empty());
        assert!(builder.app_id.is_empty());
        assert!(builder.app_secret.is_empty());
    }

    #[test]
    fn test_user_access_token_v1_builder_chain() {
        let config = create_test_config();
        let builder = UserAccessTokenV1Builder::new(config)
            .grant_code("my_grant_code")
            .app_id("my_app_id")
            .app_secret("my_app_secret");
        assert_eq!(builder.grant_code, "my_grant_code");
        assert_eq!(builder.app_id, "my_app_id");
        assert_eq!(builder.app_secret, "my_app_secret");
    }

    #[test]
    fn test_user_access_token_v1_builder_grant_code_chained() {
        let config = create_test_config();
        let builder = UserAccessTokenV1Builder::new(config).grant_code("chained_grant_code");
        assert_eq!(builder.grant_code, "chained_grant_code");
    }

    #[test]
    fn test_user_access_token_v1_builder_app_id_chained() {
        let config = create_test_config();
        let builder = UserAccessTokenV1Builder::new(config).app_id("chained_app_id");
        assert_eq!(builder.app_id, "chained_app_id");
    }

    #[test]
    fn test_user_access_token_v1_builder_app_secret_chained() {
        let config = create_test_config();
        let builder = UserAccessTokenV1Builder::new(config).app_secret("chained_secret");
        assert_eq!(builder.app_secret, "chained_secret");
    }

    #[test]
    fn test_user_access_token_v1_response_data_deserialization() {
        let json = r#"{"data":{"user_access_token":"token123","expires_in":7200,"refresh_token":"refresh456"}}"#;
        let response: UserAccessTokenV1ResponseData = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.user_access_token, "token123");
        assert_eq!(response.data.expires_in, 7200);
        assert_eq!(response.data.refresh_token, Some("refresh456".to_string()));
    }

    #[test]
    fn test_user_access_token_v1_response_data_format() {
        assert_eq!(
            UserAccessTokenV1ResponseData::data_format(),
            ResponseFormat::Data
        );
    }
}
