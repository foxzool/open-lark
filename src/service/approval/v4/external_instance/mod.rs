use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{approval::*, EndpointBuilder},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::approval::models::{DepartmentIdType, UserIdType},
};

/// 三方审批实例服务
pub struct ExternalInstanceService {
    pub config: Config,
}

/// 同步三方审批实例请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateExternalInstanceRequest {
    /// 三方审批定义编码
    pub approval_code: String,
    /// 审批实例状态
    pub status: String,
    /// 表单数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<serde_json::Value>,
    /// 发起人用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 发起人部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 外部实例ID
    pub external_instance_id: String,
    /// 外部实例链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_link: Option<String>,
    /// 审批流程详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_details: Option<Vec<serde_json::Value>>,
}

/// 同步三方审批实例响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExternalInstanceResponse {
    /// 审批实例编码
    pub instance_code: String,
}

impl ApiResponseTrait for CreateExternalInstanceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 校验三方审批实例请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckExternalInstanceRequest {
    /// 审批实例状态
    pub status: String,
    /// 外部实例ID
    pub external_instance_id: String,
    /// 校验信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_details: Option<serde_json::Value>,
}

impl ExternalInstanceService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 同步三方审批实例
    pub async fn create(
        &self,
        request: CreateExternalInstanceRequest,
        user_id_type: Option<UserIdType>,
        department_id_type: Option<DepartmentIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateExternalInstanceResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }
        if let Some(department_id_type) = department_id_type {
            query_params.insert(
                "department_id_type",
                department_id_type.as_str().to_string(),
            );
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: APPROVAL_V4_EXTERNAL_INSTANCES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 校验三方审批实例
    pub async fn check(
        &self,
        instance_code: &str,
        request: CheckExternalInstanceRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                APPROVAL_V4_EXTERNAL_INSTANCE_CHECK,
                "instance_code",
                instance_code,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
