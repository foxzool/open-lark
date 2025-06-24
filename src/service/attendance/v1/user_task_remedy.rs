use reqwest::Method;
use serde_json::json;

use crate::{
    core::{
        api_resp::BaseResponse, config::Config, constants::AccessTokenType, http::Transport,
        req_option::RequestOption, SDKResult,
    },
    impl_executable_builder,
};

use super::models::{
    CreateUserTaskRemedyRequest, CreateUserTaskRemedyRespData, QueryUserAllowedRemedysRequest,
    QueryUserAllowedRemedysRespData, QueryUserTaskRemedyRequest, QueryUserTaskRemedyRespData,
};

/// 用户补卡服务
pub struct UserTaskRemedyService {
    pub config: Config,
}

impl UserTaskRemedyService {
    /// 通知补卡审批发起
    ///
    /// 该接口用于提交员工的补卡申请，启动审批流程。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/user_task_remedy/create>
    pub async fn create(
        &self,
        request: &CreateUserTaskRemedyRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateUserTaskRemedyRespData>> {
        let mut api_req = request.api_req.clone();
        api_req.http_method = Method::POST;
        api_req.api_path = "/open-apis/attendance/v1/user_task_remedys".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type".to_string(), request.employee_type.clone());

        // 构建请求体
        let body = json!({
            "user_id": request.remedy_application.user_id.clone(),
            "remedy_date": request.remedy_application.remedy_date.clone(),
            "remedy_time": request.remedy_application.remedy_time.clone(),
            "remedy_type": request.remedy_application.remedy_type.clone(),
            "reason": request.remedy_application.reason.clone(),
            "comment": request.remedy_application.comment.clone()
        });

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 获取可补卡时间
    ///
    /// 该接口用于查询指定用户在特定时间范围内的可补卡时间段。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/user_task_remedy/query_user_allowed_remedys>
    pub async fn query_user_allowed_remedys(
        &self,
        request: &QueryUserAllowedRemedysRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<QueryUserAllowedRemedysRespData>> {
        let mut api_req = request.api_req.clone();
        api_req.http_method = Method::GET;
        api_req.api_path =
            "/open-apis/attendance/v1/user_task_remedys/query_user_allowed_remedys".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type".to_string(), request.employee_type.clone());
        api_req
            .query_params
            .insert("user_id".to_string(), request.user_id.clone());

        if let Some(date_from) = &request.date_from {
            api_req
                .query_params
                .insert("date_from".to_string(), date_from.clone());
        }

        if let Some(date_to) = &request.date_to {
            api_req.query_params.insert("date_to".to_string(), date_to.clone());
        }

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 获取补卡记录
    ///
    /// 该接口用于查询员工的补卡申请记录，支持按状态、时间范围等条件筛选。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/user_task_remedy/query>
    pub async fn query(
        &self,
        request: &QueryUserTaskRemedyRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<QueryUserTaskRemedyRespData>> {
        let mut api_req = request.api_req.clone();
        api_req.http_method = Method::GET;
        api_req.api_path = "/open-apis/attendance/v1/user_task_remedys".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type".to_string(), request.employee_type.clone());

        if let Some(user_ids) = &request.user_ids {
            api_req
                .query_params
                .insert("user_ids".to_string(), user_ids.join(","));
        }

        if let Some(date_from) = &request.date_from {
            api_req
                .query_params
                .insert("date_from".to_string(), date_from.clone());
        }

        if let Some(date_to) = &request.date_to {
            api_req.query_params.insert("date_to".to_string(), date_to.clone());
        }

        if let Some(status) = &request.status {
            api_req
                .query_params
                .insert("status".to_string(), status.to_string());
        }

        if let Some(page_size) = &request.page_size {
            api_req
                .query_params
                .insert("page_size".to_string(), page_size.to_string());
        }

        if let Some(page_token) = &request.page_token {
            api_req
                .query_params
                .insert("page_token".to_string(), page_token.clone());
        }

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

// Builder implementations
impl_executable_builder!(
    CreateUserTaskRemedyRequest,
    UserTaskRemedyService,
    CreateUserTaskRemedyRequest,
    BaseResponse<CreateUserTaskRemedyRespData>,
    create
);

impl_executable_builder!(
    QueryUserAllowedRemedysRequest,
    UserTaskRemedyService,
    QueryUserAllowedRemedysRequest,
    BaseResponse<QueryUserAllowedRemedysRespData>,
    query_user_allowed_remedys
);

impl_executable_builder!(
    QueryUserTaskRemedyRequest,
    UserTaskRemedyService,
    QueryUserTaskRemedyRequest,
    BaseResponse<QueryUserTaskRemedyRespData>,
    query
);
