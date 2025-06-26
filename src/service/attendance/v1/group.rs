use reqwest::Method;
use serde_json::json;

use crate::core::{
    api_resp::BaseResponse, config::Config, constants::AccessTokenType, http::Transport,
    req_option::RequestOption, SDKResult,
};

use super::models::{
    CreateGroupRequest, CreateGroupRespData, DeleteGroupRequest, EmptyResponse, GetGroupRequest,
    Group, ListGroupRequest, ListGroupRespData, ListGroupUserRequest, ListGroupUserRespData,
    SearchGroupRequest, SearchGroupRespData,
};

/// 考勤组服务
pub struct GroupService {
    pub config: Config,
}

impl GroupService {
    /// 查询考勤组下所有成员
    ///
    /// 该接口用于查询考勤组下所有成员的信息，支持分页查询。
    ///
    /// <https://open.feishu.cn/document/attendance-v1/group/list_user>
    pub async fn list_user(
        &self,
        request: ListGroupUserRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListGroupUserRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = format!("/open-apis/attendance/v1/groups/{}/users", request.group_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type".to_string(), request.employee_type.clone());

        if let Some(dept_type) = &request.dept_type {
            api_req
                .query_params
                .insert("dept_type".to_string(), dept_type.clone());
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

    /// 创建或修改考勤组
    ///
    /// 该接口用于创建或修改考勤组，支持设置考勤规则、工作日安排、成员管理等。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/group/create>
    pub async fn create(
        &self,
        request: CreateGroupRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateGroupRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = "/open-apis/attendance/v1/groups".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type".to_string(), request.employee_type.clone());

        if let Some(dept_type) = &request.dept_type {
            api_req
                .query_params
                .insert("dept_type".to_string(), dept_type.clone());
        }

        // 构建请求体
        let mut body = json!({
            "group_name": request.group_name.clone()
        });

        if let Some(time_zone) = &request.time_zone {
            body["time_zone"] = json!(time_zone.clone());
        }
        if let Some(bind_dept_ids) = &request.bind_dept_ids {
            body["bind_dept_ids"] = json!(bind_dept_ids.clone());
        }
        if let Some(except_date_rule) = &request.except_date_rule {
            body["except_date_rule"] = json!(except_date_rule.clone());
        }
        if let Some(attendance_type) = &request.attendance_type {
            body["attendance_type"] = json!(attendance_type.clone());
        }
        if let Some(punch_type) = &request.punch_type {
            body["punch_type"] = json!(punch_type.clone());
        }
        if let Some(allow_late_minutes) = &request.allow_late_minutes {
            body["allow_late_minutes"] = json!(allow_late_minutes.clone());
        }
        if let Some(allow_early_leave_minutes) = &request.allow_early_leave_minutes {
            body["allow_early_leave_minutes"] = json!(allow_early_leave_minutes.clone());
        }
        if let Some(work_day_rule) = &request.work_day_rule {
            body["work_day_rule"] = json!(work_day_rule.clone());
        }
        if let Some(shift_rule) = &request.shift_rule {
            body["shift_rule"] = json!(shift_rule.clone());
        }
        if let Some(member_rule) = &request.member_rule {
            body["member_rule"] = json!(member_rule.clone());
        }

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 删除考勤组
    ///
    /// 删除指定的考勤组。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/group/delete>
    pub async fn delete(
        &self,
        request: DeleteGroupRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::DELETE;
        api_req.api_path = format!("/open-apis/attendance/v1/groups/{}", request.group_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 按 ID 查询考勤组
    ///
    /// 根据考勤组 ID 查询考勤组的详细信息。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/group/get>
    pub async fn get(
        &self,
        request: GetGroupRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<Group>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = format!("/open-apis/attendance/v1/groups/{}", request.group_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type".to_string(), request.employee_type);

        if let Some(dept_type) = request.dept_type {
            api_req
                .query_params
                .insert("dept_type".to_string(), dept_type);
        }

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 按名称查询考勤组
    ///
    /// 根据考勤组名称查询考勤组信息。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/group/search>
    pub async fn search(
        &self,
        request: SearchGroupRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SearchGroupRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = "/open-apis/attendance/v1/groups/search".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type".to_string(), request.employee_type);

        if let Some(dept_type) = request.dept_type {
            api_req
                .query_params
                .insert("dept_type".to_string(), dept_type);
        }

        // 构建请求体
        let body = json!({
            "group_name": request.group_name
        });

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 查询所有考勤组
    ///
    /// 查询企业内所有考勤组的信息，支持分页查询。
    ///
    /// <https://open.feishu.cn/document/server-docs/attendance-v1/group/list>
    pub async fn list(
        &self,
        request: ListGroupRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListGroupRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = "/open-apis/attendance/v1/groups".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type".to_string(), request.employee_type);

        if let Some(dept_type) = request.dept_type {
            api_req
                .query_params
                .insert("dept_type".to_string(), dept_type);
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
