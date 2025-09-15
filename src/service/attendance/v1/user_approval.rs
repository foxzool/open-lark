use reqwest::Method;
use serde_json::json;

use crate::core::{
    api_resp::BaseResponse,
    config::Config,
    constants::AccessTokenType,
    endpoints::{EndpointBuilder, Endpoints},
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

use super::models::{
    CreateUserApprovalRequest, CreateUserApprovalRespData, ProcessUserApprovalRequest,
    ProcessUserApprovalRespData, QueryUserApprovalRequest, QueryUserApprovalRespData,
};

/// 用户审批服务
pub struct UserApprovalService {
    pub config: Config,
}

impl UserApprovalService {
    /// 获取审批数据
    ///
    /// 该接口用于查询假勤审批数据，支持按状态、时间范围、用户等条件筛选。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/user_approval/query>
    pub async fn query(
        &self,
        request: QueryUserApprovalRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<QueryUserApprovalRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = Endpoints::ATTENDANCE_V1_USER_APPROVALS.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        if let Some(status) = request.status {
            api_req.query_params.insert("status", status.to_string());
        }

        if let Some(date_from) = request.date_from {
            api_req.query_params.insert("date_from", date_from);
        }

        if let Some(date_to) = request.date_to {
            api_req.query_params.insert("date_to", date_to);
        }

        if let Some(user_ids) = request.user_ids {
            api_req.query_params.insert("user_ids", user_ids.join(","));
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 写入审批结果
    ///
    /// 该接口用于写入假勤审批的处理结果，包括通过或拒绝审批。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/user_approval/create>
    pub async fn create(
        &self,
        request: CreateUserApprovalRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateUserApprovalRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = Endpoints::ATTENDANCE_V1_USER_APPROVALS.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        // 构建请求体
        let mut body = json!({
            "approval_id": request.approval_id,
            "status": request.status
        });

        if let Some(approval_note) = request.approval_note {
            body["approval_note"] = json!(approval_note);
        }

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 通知审批状态更新
    ///
    /// 该接口用于通知假勤审批状态的更新，支持审批通过、拒绝、撤回等操作。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/user_approval/process>
    pub async fn process(
        &self,
        request: ProcessUserApprovalRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ProcessUserApprovalRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = EndpointBuilder::replace_param(
            Endpoints::ATTENDANCE_V1_USER_APPROVAL_PROCESS,
            "approval_id",
            &request.approval_id,
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        // 构建请求体
        let mut body = json!({
            "action": request.action
        });

        if let Some(message) = request.message {
            body["message"] = json!(message);
        }

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}
