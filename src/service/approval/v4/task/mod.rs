use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{BaseResponse, EmptyResponse},
        config::Config,
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::approval::models::UserIdType,
};

/// 原生审批任务服务
pub struct TaskService {
    pub config: Config,
}

/// 同意审批任务请求
#[derive(Debug, Serialize, Deserialize)]
pub struct ApproveTaskRequest {
    /// 审批意见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// 表单数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<serde_json::Value>,
}

/// 拒绝审批任务请求
#[derive(Debug, Serialize, Deserialize)]
pub struct RejectTaskRequest {
    /// 拒绝原因
    pub comment: String,
    /// 表单数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<serde_json::Value>,
}

/// 转交审批任务请求
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferTaskRequest {
    /// 转交给的用户ID
    pub transfer_user_id: String,
    /// 转交原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// 退回审批任务请求
#[derive(Debug, Serialize, Deserialize)]
pub struct RollbackTaskRequest {
    /// 退回到的节点ID
    pub node_id: String,
    /// 退回原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// 审批任务加签请求
#[derive(Debug, Serialize, Deserialize)]
pub struct AddSignTaskRequest {
    /// 加签类型
    pub add_sign_type: String,
    /// 加签用户ID列表
    pub add_sign_user_ids: Vec<String>,
    /// 加签原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// 重新提交审批任务请求
#[derive(Debug, Serialize, Deserialize)]
pub struct ResubmitTaskRequest {
    /// 表单数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<serde_json::Value>,
    /// 提交意见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl TaskService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 同意审批任务
    pub async fn approve(
        &self,
        task_id: &str,
        request: ApproveTaskRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: format!("/open-apis/approval/v4/tasks/{task_id}/approve"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 拒绝审批任务
    pub async fn reject(
        &self,
        task_id: &str,
        request: RejectTaskRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: format!("/open-apis/approval/v4/tasks/{task_id}/reject"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 转交审批任务
    pub async fn transfer(
        &self,
        task_id: &str,
        request: TransferTaskRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: format!("/open-apis/approval/v4/tasks/{task_id}/transfer"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 退回审批任务
    pub async fn rollback(
        &self,
        task_id: &str,
        request: RollbackTaskRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: format!("/open-apis/approval/v4/tasks/{task_id}/specified_rollback"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 审批任务加签
    pub async fn add_sign(
        &self,
        task_id: &str,
        request: AddSignTaskRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: format!("/open-apis/approval/v4/tasks/{task_id}/add_sign"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 重新提交审批任务
    pub async fn resubmit(
        &self,
        task_id: &str,
        request: ResubmitTaskRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: format!("/open-apis/approval/v4/tasks/{task_id}/resubmit"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
