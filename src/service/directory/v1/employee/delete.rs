use reqwest::Method;
use serde::Deserialize;
use serde_json::json;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::directory::*,
        endpoints::EndpointBuilder,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
    service::directory::v1::models::{DepartmentIdType, Employee, UserIdType},
};

use super::EmployeeService;

/// 离职员工请求
#[derive(Default, Clone)]
pub struct DeleteEmployeeRequest {
    pub api_req: ApiRequest,
    /// 员工ID
    pub employee_id: String,
    /// 离职时间 (格式: YYYY-MM-DD)
    pub leave_time: Option<String>,
    /// 离职原因
    pub leave_reason: Option<String>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
    /// 部门ID类型
    pub department_id_type: Option<DepartmentIdType>,
}

impl DeleteEmployeeRequest {
    /// 创建离职员工请求的构建器
    pub fn builder(employee_id: impl ToString) -> DeleteEmployeeRequestBuilder {
        DeleteEmployeeRequestBuilder {
            request: DeleteEmployeeRequest {
                employee_id: employee_id.to_string(),
                ..Default::default()
            },
        }
    }
}

/// 离职员工请求构建器
#[derive(Default)]
pub struct DeleteEmployeeRequestBuilder {
    request: DeleteEmployeeRequest,
}

impl DeleteEmployeeRequestBuilder {
    /// 设置离职时间 (格式: YYYY-MM-DD)
    pub fn leave_time(mut self, leave_time: impl ToString) -> Self {
        self.request.leave_time = Some(leave_time.to_string());
        self
    }

    /// 设置离职原因
    pub fn leave_reason(mut self, leave_reason: impl ToString) -> Self {
        self.request.leave_reason = Some(leave_reason.to_string());
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.request.user_id_type = Some(user_id_type);
        self
    }

    /// 设置部门ID类型
    pub fn department_id_type(mut self, department_id_type: DepartmentIdType) -> Self {
        self.request.department_id_type = Some(department_id_type);
        self
    }

    /// 构建请求
    pub fn build(mut self) -> DeleteEmployeeRequest {
        // 构建查询参数
        if let Some(user_id_type) = &self.request.user_id_type {
            self.request
                .api_req
                .query_params
                .insert("user_id_type", user_id_type.to_string());
        }

        if let Some(department_id_type) = &self.request.department_id_type {
            self.request
                .api_req
                .query_params
                .insert("department_id_type", department_id_type.to_string());
        }

        // 构建请求体
        let mut body = json!({});

        if let Some(ref leave_time) = self.request.leave_time {
            body["leave_time"] = json!(leave_time);
        }

        if let Some(ref leave_reason) = self.request.leave_reason {
            body["leave_reason"] = json!(leave_reason);
        }

        self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request
    }
}

/// 离职员工响应数据
#[derive(Debug, Deserialize)]
pub struct DeleteEmployeeResponseData {
    /// 员工信息
    pub employee: Employee,
}

/// 离职员工响应
#[derive(Debug, Deserialize)]
pub struct DeleteEmployeeResponse {
    /// 响应数据
    pub data: DeleteEmployeeResponseData,
}

impl ApiResponseTrait for DeleteEmployeeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl EmployeeService {
    /// 离职员工
    ///
    /// 将员工状态变更为离职状态。身份验证方式为应用身份验证
    ///
    /// # 参数
    /// - `request`: 离职员工请求
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// 员工信息
    ///
    /// # 示例
    /// ```rust,ignore
    /// let response = client.directory.v1.employee.delete(
    ///     DeleteEmployeeRequest::builder("employee_id")
    ///         .leave_time("2024-01-01")
    ///         .leave_reason("个人原因")
    ///         .build(),
    ///     None
    /// ).await?;
    /// ```
    ///
    /// 参考: <https://open.feishu.cn/document/directory-v1/employee/delete>
    pub async fn delete(
        &self,
        request: DeleteEmployeeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DeleteEmployeeResponse>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::DELETE;
        api_req.api_path = EndpointBuilder::replace_param(
            DIRECTORY_V1_EMPLOYEE_GET,
            "employee_id",
            &request.employee_id,
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

// 应用ExecutableBuilder宏
impl_executable_builder_owned!(
    DeleteEmployeeRequestBuilder,
    EmployeeService,
    DeleteEmployeeRequest,
    BaseResponse<DeleteEmployeeResponse>,
    delete
);
