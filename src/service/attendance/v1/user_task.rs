use reqwest::Method;
use serde_json::json;

use crate::core::{
    api_resp::BaseResponse, config::Config, constants::AccessTokenType, http::Transport,
    req_option::RequestOption, SDKResult,
};

use super::models::{
    BatchCreateUserTaskRequest, BatchCreateUserTaskRespData, BatchDelUserTaskRequest,
    BatchDelUserTaskRespData, GetUserTaskRequest, GetUserTaskRespData, QueryUserTaskRequest,
    QueryUserTaskResultRequest, QueryUserTaskResultRespData, QueryUserTaskRespData,
};

/// 用户打卡任务服务
pub struct UserTaskService {
    pub config: Config,
}

impl UserTaskService {
    /// 导入打卡流水
    ///
    /// 该接口用于批量导入员工的打卡记录，支持补录历史打卡数据。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/user_task/batch_create>
    pub async fn batch_create(
        &self,
        request: BatchCreateUserTaskRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BatchCreateUserTaskRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = "/open-apis/attendance/v1/user_tasks/batch_create".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type".to_string(), request.employee_type);

        // 构建请求体
        let body = json!({
            "user_tasks": request.user_tasks
        });

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 查询打卡流水
    ///
    /// 该接口用于查询指定用户在特定日期的打卡流水记录。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/user_task/get>
    pub async fn get(
        &self,
        request: GetUserTaskRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetUserTaskRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = format!(
            "/open-apis/attendance/v1/user_tasks/{}/get",
            request.user_id
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type".to_string(), request.employee_type);
        api_req
            .query_params
            .insert("check_date".to_string(), request.check_date);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 批量查询打卡流水
    ///
    /// 该接口用于批量查询多个用户的打卡流水记录，支持按时间范围和打卡类型筛选。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/user_task/query-2>
    pub async fn query(
        &self,
        request: QueryUserTaskRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<QueryUserTaskRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = "/open-apis/attendance/v1/user_tasks/query".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type".to_string(), request.employee_type);

        if let Some(user_ids) = request.user_ids {
            api_req
                .query_params
                .insert("user_ids".to_string(), user_ids.join(","));
        }

        if let Some(check_date_from) = request.check_date_from {
            api_req
                .query_params
                .insert("check_date_from".to_string(), check_date_from);
        }

        if let Some(check_date_to) = request.check_date_to {
            api_req
                .query_params
                .insert("check_date_to".to_string(), check_date_to);
        }

        if let Some(check_type) = request.check_type {
            api_req
                .query_params
                .insert("check_type".to_string(), check_type.to_string());
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size".to_string(), page_size.to_string());
        }

        if let Some(page_token) = request.page_token {
            api_req
                .query_params
                .insert("page_token".to_string(), page_token);
        }

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 删除打卡流水
    ///
    /// 该接口用于批量删除指定的打卡流水记录。
    ///
    /// <https://open.feishu.cn/document/attendance-v1/user_task/batch_del>
    pub async fn batch_del(
        &self,
        request: BatchDelUserTaskRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BatchDelUserTaskRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = "/open-apis/attendance/v1/user_tasks/batch_del".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type".to_string(), request.employee_type);

        // 构建请求体
        let body = json!({
            "task_ids": request.task_ids
        });

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 查询打卡结果
    ///
    /// 该接口用于查询员工的考勤结果，包括工作时长、加班时长、考勤状态等汇总信息。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/user_task/query>
    pub async fn query_result(
        &self,
        request: QueryUserTaskResultRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<QueryUserTaskResultRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = "/open-apis/attendance/v1/user_task_results/query".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type".to_string(), request.employee_type);

        if let Some(user_ids) = request.user_ids {
            api_req
                .query_params
                .insert("user_ids".to_string(), user_ids.join(","));
        }

        if let Some(check_date_from) = request.check_date_from {
            api_req
                .query_params
                .insert("check_date_from".to_string(), check_date_from);
        }

        if let Some(check_date_to) = request.check_date_to {
            api_req
                .query_params
                .insert("check_date_to".to_string(), check_date_to);
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size".to_string(), page_size.to_string());
        }

        if let Some(page_token) = request.page_token {
            api_req
                .query_params
                .insert("page_token".to_string(), page_token);
        }

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}