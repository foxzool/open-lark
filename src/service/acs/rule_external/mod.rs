use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpoints},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::acs::models::{RuleExternal, RuleStatus},
};

/// 权限组管理服务
pub struct RuleExternalService {
    pub config: Config,
}

impl RuleExternalService {
    /// 创建权限组管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建或更新权限组
    ///
    /// 创建新的权限组或更新现有权限组的信息。
    ///
    /// # Arguments
    ///
    /// * `request` - 权限组创建或更新请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回权限组信息
    pub async fn create_or_update_rule(
        &self,
        request: RuleCreateOrUpdateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RuleCreateOrUpdateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::ACS_V1_RULE_EXTERNAL.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取权限组信息
    ///
    /// 根据权限组ID获取权限组的详细信息。
    ///
    /// # Arguments
    ///
    /// * `rule_id` - 权限组ID
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回权限组详细信息
    pub async fn get_rule(
        &self,
        rule_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RuleGetResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::ACS_V1_RULE_EXTERNAL_OPERATION,
                "rule_id",
                rule_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除权限组
    ///
    /// 删除指定的权限组。
    ///
    /// # Arguments
    ///
    /// * `rule_id` - 权限组ID
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回删除结果
    pub async fn delete_rule(
        &self,
        rule_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RuleDeleteResponse>> {
        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                Endpoints::ACS_V1_RULE_EXTERNAL_OPERATION,
                "rule_id",
                rule_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 设备绑定权限组
    ///
    /// 将指定设备绑定到权限组，或修改设备的权限组绑定关系。
    ///
    /// # Arguments
    ///
    /// * `request` - 设备绑定请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回绑定结果
    pub async fn bind_device(
        &self,
        request: DeviceBindRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DeviceBindResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::ACS_V1_RULE_EXTERNAL_DEVICE_BIND.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

/// 权限组创建或更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCreateOrUpdateRequest {
    /// 权限组ID（更新时必须提供）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    /// 权限组名称
    pub name: String,
    /// 权限组描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 权限组状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RuleStatus>,
    /// 关联的用户ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 生效开始时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 生效结束时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

/// 权限组创建或更新响应
#[derive(Debug, Serialize, Deserialize)]
pub struct RuleCreateOrUpdateResponse {
    /// 权限组信息
    pub rule_external: RuleExternal,
}

impl ApiResponseTrait for RuleCreateOrUpdateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 权限组详情查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct RuleGetResponse {
    /// 权限组详细信息
    pub rule_external: RuleExternal,
}

impl ApiResponseTrait for RuleGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 权限组删除响应
#[derive(Debug, Serialize, Deserialize)]
pub struct RuleDeleteResponse {
    /// 删除成功标识
    pub success: bool,
}

impl ApiResponseTrait for RuleDeleteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 设备绑定权限组请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceBindRequest {
    /// 设备ID
    pub device_id: String,
    /// 权限组ID列表
    pub rule_ids: Vec<String>,
    /// 绑定操作类型 (bind: 绑定, unbind: 解绑)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
}

/// 设备绑定权限组响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceBindResponse {
    /// 绑定成功标识
    pub success: bool,
    /// 绑定的设备ID
    pub device_id: String,
    /// 绑定的权限组ID列表
    pub rule_ids: Vec<String>,
}

impl ApiResponseTrait for DeviceBindResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
