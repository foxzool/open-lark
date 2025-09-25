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
    service::approval::models::{Approval, DepartmentIdType, UserIdType},
};

/// 原生审批定义服务
pub struct ApprovalService {
    pub config: Config,
}

/// 创建审批定义请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateApprovalRequest {
    /// 审批定义名称
    pub approval_name: String,
    /// 审批定义描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 表单字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<serde_json::Value>,
    /// 审批流程
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<serde_json::Value>,
    /// 审批设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}

/// 创建审批定义响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApprovalResponse {
    /// 审批定义编码
    pub approval_code: String,
}

impl ApiResponseTrait for CreateApprovalResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取审批定义响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetApprovalResponse {
    /// 审批定义信息
    pub approval: Approval,
}

impl ApiResponseTrait for GetApprovalResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApprovalService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建审批定义
    pub async fn create(
        &self,
        request: CreateApprovalRequest,
        user_id_type: Option<UserIdType>,
        department_id_type: Option<DepartmentIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateApprovalResponse>> {
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
            api_path: APPROVAL_V4_APPROVALS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查看指定审批定义
    pub async fn get(
        &self,
        approval_code: &str,
        user_id_type: Option<UserIdType>,
        department_id_type: Option<DepartmentIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetApprovalResponse>> {
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
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                APPROVAL_V4_APPROVAL_GET,
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
