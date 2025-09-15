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
    service::apass::models::{
        FlowExecuteRequest, FlowExecuteResult, PageResponse, RollbackPoint, UserTask,
        UserTaskActionRequest, UserTaskAddAssigneeRequest, UserTaskCcRequest,
        UserTaskChatGroupRequest, UserTaskQueryRequest, UserTaskRollbackRequest,
        UserTaskTransferRequest,
    },
};

/// 流程管理服务
pub struct FlowService {
    pub config: Config,
}

/// 流程执行响应
#[derive(Debug, Serialize, Deserialize)]
pub struct FlowExecuteResponse {
    /// 流程执行结果
    #[serde(flatten)]
    pub execute_result: FlowExecuteResult,
}

impl ApiResponseTrait for FlowExecuteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 人工任务查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UserTaskQueryResponse {
    /// 分页响应数据
    #[serde(flatten)]
    pub page_response: PageResponse<UserTask>,
}

impl ApiResponseTrait for UserTaskQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 人工任务操作成功响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UserTaskActionResponse {
    /// 操作是否成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 操作消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ApiResponseTrait for UserTaskActionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 退回位置查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UserTaskRollbackPointsResponse {
    /// 可退回的位置列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_points: Option<Vec<RollbackPoint>>,
}

impl ApiResponseTrait for UserTaskRollbackPointsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 群聊创建响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UserTaskChatGroupResponse {
    /// 创建的群聊ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    /// 操作是否成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl ApiResponseTrait for UserTaskChatGroupResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl FlowService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 发起流程
    ///
    /// 该接口用于发起一个新的流程实例。
    ///
    /// # 参数
    ///
    /// - `request`: 流程执行请求参数
    /// - `option`: 可选的请求配置
    pub async fn execute_flow(
        &self,
        request: FlowExecuteRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<FlowExecuteResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_FLOW_EXECUTE,
                &[
                    ("app_id", &request.app_id),
                    ("flow_api_name", &request.flow_api_name),
                ],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&serde_json::json!({
                "parameters": request.parameters
            }))?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询人工任务
    ///
    /// 该接口用于查询人工任务列表。
    ///
    /// # 参数
    ///
    /// - `request`: 人工任务查询请求参数
    /// - `option`: 可选的请求配置
    pub async fn query_user_tasks(
        &self,
        request: UserTaskQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UserTaskQueryResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::APASS_V1_FLOW_USER_TASK_QUERY,
                "app_id",
                &request.app_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(status) = request.status {
            api_req.query_params.insert("status", status);
        }
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 同意人工任务
    ///
    /// 该接口用于同意指定的人工任务。
    ///
    /// # 参数
    ///
    /// - `request`: 人工任务操作请求参数
    /// - `option`: 可选的请求配置
    pub async fn agree_user_task(
        &self,
        request: UserTaskActionRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UserTaskActionResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_FLOW_USER_TASK_AGREE,
                &[("app_id", &request.app_id), ("task_id", &request.task_id)],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&serde_json::json!({
                "comment": request.comment,
                "data": request.data
            }))?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 拒绝人工任务
    ///
    /// 该接口用于拒绝指定的人工任务。
    ///
    /// # 参数
    ///
    /// - `request`: 人工任务操作请求参数
    /// - `option`: 可选的请求配置
    pub async fn reject_user_task(
        &self,
        request: UserTaskActionRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UserTaskActionResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_FLOW_USER_TASK_REJECT,
                &[("app_id", &request.app_id), ("task_id", &request.task_id)],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&serde_json::json!({
                "comment": request.comment,
                "data": request.data
            }))?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 转交人工任务
    ///
    /// 该接口用于转交指定的人工任务给其他用户。
    ///
    /// # 参数
    ///
    /// - `request`: 人工任务转交请求参数
    /// - `option`: 可选的请求配置
    pub async fn transfer_user_task(
        &self,
        request: UserTaskTransferRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UserTaskActionResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_FLOW_USER_TASK_TRANSFER,
                &[("app_id", &request.app_id), ("task_id", &request.task_id)],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&serde_json::json!({
                "target_user_id": request.target_user_id,
                "comment": request.comment
            }))?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 人工任务加签
    ///
    /// 该接口用于为人工任务加签处理人。
    ///
    /// # 参数
    ///
    /// - `request`: 人工任务加签请求参数
    /// - `option`: 可选的请求配置
    pub async fn add_assignee_user_task(
        &self,
        request: UserTaskAddAssigneeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UserTaskActionResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_FLOW_USER_TASK_ADD_ASSIGNEE,
                &[("app_id", &request.app_id), ("task_id", &request.task_id)],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&serde_json::json!({
                "assignee_user_ids": request.assignee_user_ids,
                "comment": request.comment
            }))?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 抄送人工任务
    ///
    /// 该接口用于抄送人工任务给其他用户。
    ///
    /// # 参数
    ///
    /// - `request`: 人工任务抄送请求参数
    /// - `option`: 可选的请求配置
    pub async fn cc_user_task(
        &self,
        request: UserTaskCcRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UserTaskActionResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_FLOW_USER_TASK_CC,
                &[("app_id", &request.app_id), ("task_id", &request.task_id)],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&serde_json::json!({
                "cc_user_ids": request.cc_user_ids,
                "comment": request.comment
            }))?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 催办人工任务
    ///
    /// 该接口用于催办指定的人工任务。
    ///
    /// # 参数
    ///
    /// - `request`: 人工任务操作请求参数
    /// - `option`: 可选的请求配置
    pub async fn expedite_user_task(
        &self,
        request: UserTaskActionRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UserTaskActionResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_FLOW_USER_TASK_EXPEDITING,
                &[("app_id", &request.app_id), ("task_id", &request.task_id)],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&serde_json::json!({
                "comment": request.comment
            }))?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 撤销人工任务
    ///
    /// 该接口用于撤销指定的人工任务。
    ///
    /// # 参数
    ///
    /// - `request`: 人工任务操作请求参数
    /// - `option`: 可选的请求配置
    pub async fn cancel_user_task(
        &self,
        request: UserTaskActionRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UserTaskActionResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_FLOW_USER_TASK_CANCEL,
                &[("app_id", &request.app_id), ("task_id", &request.task_id)],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&serde_json::json!({
                "comment": request.comment
            }))?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询人工任务可退回的位置
    ///
    /// 该接口用于查询指定人工任务可退回的位置列表。
    ///
    /// # 参数
    ///
    /// - `app_id`: 应用ID
    /// - `task_id`: 任务ID
    /// - `option`: 可选的请求配置
    pub async fn get_user_task_rollback_points(
        &self,
        app_id: String,
        task_id: String,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UserTaskRollbackPointsResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_FLOW_USER_TASK_ROLLBACK_POINTS,
                &[("app_id", &app_id), ("task_id", &task_id)],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 退回人工任务
    ///
    /// 该接口用于退回指定的人工任务到指定节点。
    ///
    /// # 参数
    ///
    /// - `request`: 人工任务退回请求参数
    /// - `option`: 可选的请求配置
    pub async fn rollback_user_task(
        &self,
        request: UserTaskRollbackRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UserTaskActionResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_FLOW_USER_TASK_ROLLBACK,
                &[("app_id", &request.app_id), ("task_id", &request.task_id)],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&serde_json::json!({
                "target_node_id": request.target_node_id,
                "comment": request.comment
            }))?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 基于人工任务发起群聊
    ///
    /// 该接口用于基于人工任务创建群聊。
    ///
    /// # 参数
    ///
    /// - `request`: 人工任务群聊请求参数
    /// - `option`: 可选的请求配置
    pub async fn create_user_task_chat_group(
        &self,
        request: UserTaskChatGroupRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UserTaskChatGroupResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_FLOW_USER_TASK_CHAT_GROUP,
                &[("app_id", &request.app_id), ("task_id", &request.task_id)],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&serde_json::json!({
                "group_name": request.group_name
            }))?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
