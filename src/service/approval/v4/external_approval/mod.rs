use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{approval::*, EndpointBuilder},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::approval::models::{DepartmentIdType, UserIdType},
};

/// 三方审批定义服务
pub struct ExternalApprovalService {
    pub config: Config,
}

/// 创建三方审批定义请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateExternalApprovalRequest {
    /// 审批名称
    pub approval_name: String,
    /// 审批描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 外部审批系统URL
    pub external_url: String,
    /// 回调URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    /// 审批定义配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
}

/// 创建三方审批定义响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExternalApprovalResponse {
    /// 三方审批定义编码
    pub approval_code: String,
}

impl ApiResponseTrait for CreateExternalApprovalResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取三方审批定义响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetExternalApprovalResponse {
    /// 审批名称
    pub approval_name: String,
    /// 审批描述
    pub description: Option<String>,
    /// 外部审批系统URL
    pub external_url: String,
    /// 回调URL
    pub callback_url: Option<String>,
    /// 审批定义配置
    pub config: Option<serde_json::Value>,
    /// 状态
    pub status: String,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
}

impl ApiResponseTrait for GetExternalApprovalResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ExternalApprovalService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建三方审批定义
    pub async fn create(
        &self,
        request: CreateExternalApprovalRequest,
        user_id_type: Option<UserIdType>,
        department_id_type: Option<DepartmentIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateExternalApprovalResponse>> {
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
            api_path: APPROVAL_V4_EXTERNAL_APPROVALS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查看指定三方审批定义
    pub async fn get(
        &self,
        approval_code: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetExternalApprovalResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                APPROVAL_V4_EXTERNAL_APPROVAL_GET,
                "approval_code",
                approval_code,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
