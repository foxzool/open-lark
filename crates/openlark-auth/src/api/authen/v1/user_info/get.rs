//! 获取用户信息 API
use crate::models::authen::UserInfoResponse;
///
/// API文档: https://open.feishu.cn/document/server-docs/user-authentication/access-token/user_info
///
/// 通过 `user_access_token` 获取登录用户的信息。
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取用户信息请求
pub struct UserInfoBuilder {
    user_access_token: String,
    user_id_type: Option<String>,
    /// 配置信息
    config: Config,
}

/// 获取用户信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserInfoResponseData {
    /// 用户信息响应
    pub data: UserInfoResponse,
}

impl ApiResponseTrait for UserInfoResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UserInfoBuilder {
    /// 创建 user_info 请求
    pub fn new(config: Config) -> Self {
        Self {
            user_access_token: String::new(),
            user_id_type: None,
            config,
        }
    }

    /// 设置用户访问令牌
    pub fn user_access_token(mut self, user_access_token: impl Into<String>) -> Self {
        self.user_access_token = user_access_token.into();
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UserInfoResponseData> {
        // 验证必填字段
        validate_required!(self.user_access_token, "用户访问令牌不能为空");

        // 🚀 使用新的enum+builder系统生成API端点
        use crate::common::api_endpoints::AuthenApiV1;
        let api_endpoint = AuthenApiV1::UserInfo;

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<UserInfoResponseData> =
            ApiRequest::get(api_endpoint.to_url());

        // 添加Authorization头
        api_request.headers.insert(
            "Authorization".to_string(),
            format!("Bearer {}", self.user_access_token),
        );

        // 添加查询参数
        if let Some(ref user_id_type) = self.user_id_type {
            api_request
                .query
                .insert("user_id_type".to_string(), user_id_type.clone());
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// 用户信息API服务
#[derive(Debug)]
pub struct UserInfoService {
    config: Config,
}

impl UserInfoService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取用户信息
    pub fn get(&self) -> UserInfoBuilder {
        UserInfoBuilder::new(self.config.clone())
    }
}
