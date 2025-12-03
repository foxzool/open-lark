//! 用户信息API实现
//!
//! 本模块提供用户信息获取的API实现，根据meta.resource=user_info组织

use openlark_core::{
    config::Config,
    api::ApiRequest,
    prelude::Transport,
    error::{SDKResult, api_error},
};
use crate::models::authen::{UserInfoRequest, UserInfoResponse};

// 类型别名
pub type AuthenResult<T> = SDKResult<T>;

/// 用户信息获取构建器
#[derive(Debug)]
pub struct UserInfoBuilder {
    config: Config,
    request: UserInfoRequest,
}

impl UserInfoBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: UserInfoRequest {
                user_access_token: String::new(),
                user_id_type: None,
            },
        }
    }

    /// 设置用户访问令牌
    pub fn user_access_token(mut self, user_access_token: impl Into<String>) -> Self {
        self.request.user_access_token = user_access_token.into();
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    /// 发送请求获取用户信息
    pub async fn send(self) -> AuthenResult<UserInfoResponse> {
        // 构建API请求
        let url = format!("{}/open-apis/authen/v1/user_info", self.config.base_url);

        let request: ApiRequest<UserInfoResponse> = ApiRequest::get(&url)
            .header("Authorization", format!("Bearer {}", self.request.user_access_token));

        // 使用Transport发送请求
        let response = Transport::request(request, &self.config, None).await?;

        // 处理响应
        if response.raw_response.code == 0 {
            Ok(response.data.unwrap())
        } else {
            // 映射飞书错误码
            let error_code = response.raw_response.code;
            let error_message = response.raw_response.msg.clone();

            match error_code {
                99991663 => Err(api_error(401, "/open-apis/authen/v1/user_info", "用户访问令牌无效", None::<String>)),
                99991677 => Err(api_error(401, "/open-apis/authen/v1/user_info", "用户访问令牌已过期", None::<String>)),
                99991641 | 99991642 | 99991645 => Err(api_error(401, "/open-apis/authen/v1/user_info", "用户会话失效", None::<String>)),
                _ => Err(api_error(error_code as u16, "/open-apis/authen/v1/user_info", error_message, None::<String>)),
            }
        }
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