use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::approval::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::approval::models::{ApprovalTask, UserIdType},
};

/// 三方审批任务服务
pub struct ExternalTaskService {
    pub config: Config,
}

/// 获取三方审批任务状态响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListExternalTaskResponse {
    /// 任务列表
    pub tasks: Vec<ApprovalTask>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListExternalTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 外部任务查询参数
#[derive(Debug, Default)]
pub struct ListExternalTaskParams {
    pub page_size: Option<i32>,
    pub page_token: Option<String>,
    pub approval_code: Option<String>,
    pub instance_code: Option<String>,
    pub user_id: Option<String>,
    pub task_status: Option<String>,
    pub user_id_type: Option<UserIdType>,
}

impl ExternalTaskService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取三方审批任务状态
    pub async fn list(
        &self,
        params: Option<ListExternalTaskParams>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListExternalTaskResponse>> {
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
            if let Some(instance_code) = params.instance_code {
                query_params.insert("instance_code", instance_code);
            }
            if let Some(user_id) = params.user_id {
                query_params.insert("user_id", user_id);
            }
            if let Some(task_status) = params.task_status {
                query_params.insert("task_status", task_status);
            }
            if let Some(user_id_type) = params.user_id_type {
                query_params.insert("user_id_type", user_id_type.as_str().to_string());
            }
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: APPROVAL_V4_EXTERNAL_TASKS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
