use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::mdm::*,
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    },
    service::mdm::models::UserAuthDataRelation,
};

/// 用户数据维度管理服务
pub struct UserAuthDataRelationService {
    pub config: Config,
}

impl UserAuthDataRelationService {
    /// 创建用户数据维度管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 用户数据维度绑定
    ///
    /// 将用户与指定的数据维度建立绑定关系，授予用户对该数据维度的访问权限。
    ///
    /// # Arguments
    ///
    /// * `request` - 数据维度绑定请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回绑定结果
    pub async fn bind(
        &self,
        request: UserDataRelationBindRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UserDataRelationBindResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: MDM_V1_USER_AUTH_DATA_RELATIONS_BIND.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 用户数据维度解绑
    ///
    /// 解除用户与指定数据维度的绑定关系，取消用户对该数据维度的访问权限。
    ///
    /// # Arguments
    ///
    /// * `request` - 数据维度解绑请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回解绑结果
    pub async fn unbind(
        &self,
        request: UserDataRelationUnbindRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UserDataRelationUnbindResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: MDM_V1_USER_AUTH_DATA_RELATIONS_UNBIND.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

impl Service for UserAuthDataRelationService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "user_auth_data_relation"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

/// 用户数据维度绑定请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDataRelationBindRequest {
    /// 用户ID
    pub user_id: String,
    /// 数据维度ID
    pub data_dimension_id: String,
    /// 绑定类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_type: Option<String>,
    /// 权限级别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_level: Option<String>,
    /// 生效开始时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_start_time: Option<i64>,
    /// 生效结束时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_end_time: Option<i64>,
    /// 备注说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
}

/// 用户数据维度绑定响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UserDataRelationBindResponse {
    /// 绑定关系信息
    pub relation: UserAuthDataRelation,
    /// 操作是否成功
    pub success: bool,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl ApiResponseTrait for UserDataRelationBindResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 用户数据维度解绑请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDataRelationUnbindRequest {
    /// 用户ID
    pub user_id: String,
    /// 数据维度ID
    pub data_dimension_id: String,
    /// 解绑原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unbind_reason: Option<String>,
}

/// 用户数据维度解绑响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UserDataRelationUnbindResponse {
    /// 操作是否成功
    pub success: bool,
    /// 解绑的关系ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_id: Option<String>,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl ApiResponseTrait for UserDataRelationUnbindResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
