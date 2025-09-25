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
    service::approval::models::{ApprovalInstance, DepartmentIdType, UserIdType},
};

/// 原生审批实例服务
pub struct InstanceService {
    pub config: Config,
}

/// 创建审批实例请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInstanceRequest {
    /// 审批定义编码
    pub approval_code: String,
    /// 表单数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<serde_json::Value>,
    /// 发起人用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 发起人部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 审批实例UUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

/// 创建审批实例响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInstanceResponse {
    /// 审批实例编码
    pub instance_code: String,
}

impl ApiResponseTrait for CreateInstanceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取审批实例响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInstanceResponse {
    /// 审批实例信息
    pub instance: ApprovalInstance,
}

impl ApiResponseTrait for GetInstanceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 审批实例列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListInstanceResponse {
    /// 审批实例ID列表
    pub instance_code_list: Vec<String>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否还有更多数据
    pub has_more: bool,
}

impl ApiResponseTrait for ListInstanceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 抄送审批实例请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CcInstanceRequest {
    /// 抄送用户ID列表
    pub user_id_list: Vec<String>,
    /// 抄送消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_message: Option<String>,
}

/// 预览审批流程请求
#[derive(Debug, Serialize, Deserialize)]
pub struct PreviewInstanceRequest {
    /// 审批定义编码
    pub approval_code: String,
    /// 发起人用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 发起人部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 表单数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<serde_json::Value>,
}

/// 预览审批流程响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewInstanceResponse {
    /// 预览流程信息
    pub preview_nodes: Vec<PreviewNode>,
}

/// 预览节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewNode {
    /// 节点ID
    pub node_id: String,
    /// 节点名称
    pub node_name: Option<String>,
    /// 节点类型
    pub node_type: String,
    /// 审批人信息
    pub approvers: Option<Vec<serde_json::Value>>,
}

impl ApiResponseTrait for PreviewInstanceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 实例列表查询参数
#[derive(Debug, Default)]
pub struct ListInstanceParams {
    pub page_size: Option<i32>,
    pub page_token: Option<String>,
    pub approval_code: Option<String>,
    pub instance_status: Option<String>,
    pub user_id: Option<String>,
    pub user_id_type: Option<UserIdType>,
}

impl InstanceService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建审批实例
    pub async fn create(
        &self,
        request: CreateInstanceRequest,
        user_id_type: Option<UserIdType>,
        department_id_type: Option<DepartmentIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateInstanceResponse>> {
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
            api_path: APPROVAL_V4_INSTANCES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 撤回审批实例
    pub async fn cancel(
        &self,
        instance_code: &str,
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
                APPROVAL_V4_INSTANCE_CANCEL,
                "instance_code",
                instance_code,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 抄送审批实例
    pub async fn cc(
        &self,
        instance_code: &str,
        request: CcInstanceRequest,
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
                APPROVAL_V4_INSTANCE_CC,
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

    /// 预览审批流程
    pub async fn preview(
        &self,
        request: PreviewInstanceRequest,
        user_id_type: Option<UserIdType>,
        department_id_type: Option<DepartmentIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<PreviewInstanceResponse>> {
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
            api_path: APPROVAL_V4_INSTANCE_PREVIEW.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取单个审批实例详情
    pub async fn get(
        &self,
        instance_code: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetInstanceResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                APPROVAL_V4_INSTANCE_GET,
                "instance_code",
                instance_code,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 批量获取审批实例ID
    pub async fn list(
        &self,
        params: Option<ListInstanceParams>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListInstanceResponse>> {
        let mut query_params = HashMap::new();
        if let Some(params) = params {
            if let Some(page_size) = params.page_size {
                query_params.insert("page_size", page_size.to_string());
            }
            if let Some(page_token) = params.page_token {
                query_params.insert("page_token", page_token);
            }
            if let Some(approval_code) = params.approval_code {
                query_params.insert("approval_code", approval_code);
            }
            if let Some(instance_status) = params.instance_status {
                query_params.insert("instance_status", instance_status);
            }
            if let Some(user_id) = params.user_id {
                query_params.insert("user_id", user_id);
            }
            if let Some(user_id_type) = params.user_id_type {
                query_params.insert("user_id_type", user_id_type.as_str().to_string());
            }
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: APPROVAL_V4_INSTANCES_LIST.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
