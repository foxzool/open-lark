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

/// 恢复离职员工请求
#[derive(Default, Clone)]
pub struct ResurrectEmployeeRequest {
    pub api_req: ApiRequest,
    /// 员工ID
    pub employee_id: String,
    /// 上级ID
    pub leader_id: Option<String>,
    /// 部门ID列表
    pub department_ids: Option<Vec<String>>,
    /// 工作地点
    pub work_location: Option<String>,
    /// 职级
    pub job_level: Option<String>,
    /// 职位
    pub job_title: Option<String>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
    /// 部门ID类型
    pub department_id_type: Option<DepartmentIdType>,
}

impl ResurrectEmployeeRequest {
    /// 创建恢复离职员工请求的构建器
    pub fn builder(employee_id: impl ToString) -> ResurrectEmployeeRequestBuilder {
        ResurrectEmployeeRequestBuilder {
            request: ResurrectEmployeeRequest {
                employee_id: employee_id.to_string(),
                ..Default::default()
            },
        }
    }
}

/// 恢复离职员工请求构建器
#[derive(Default)]
pub struct ResurrectEmployeeRequestBuilder {
    request: ResurrectEmployeeRequest,
}

impl ResurrectEmployeeRequestBuilder {
    /// 设置上级ID
    pub fn leader_id(mut self, leader_id: impl ToString) -> Self {
        self.request.leader_id = Some(leader_id.to_string());
        self
    }

    /// 设置部门ID列表
    pub fn department_ids(mut self, department_ids: Vec<String>) -> Self {
        self.request.department_ids = Some(department_ids);
        self
    }

    /// 设置工作地点
    pub fn work_location(mut self, work_location: impl ToString) -> Self {
        self.request.work_location = Some(work_location.to_string());
        self
    }

    /// 设置职级
    pub fn job_level(mut self, job_level: impl ToString) -> Self {
        self.request.job_level = Some(job_level.to_string());
        self
    }

    /// 设置职位
    pub fn job_title(mut self, job_title: impl ToString) -> Self {
        self.request.job_title = Some(job_title.to_string());
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
    pub fn build(mut self) -> ResurrectEmployeeRequest {
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

        if let Some(ref leader_id) = self.request.leader_id {
            body["leader_id"] = json!(leader_id);
        }

        if let Some(ref department_ids) = self.request.department_ids {
            body["department_ids"] = json!(department_ids);
        }

        if let Some(ref work_location) = self.request.work_location {
            body["work_location"] = json!(work_location);
        }

        if let Some(ref job_level) = self.request.job_level {
            body["job_level"] = json!(job_level);
        }

        if let Some(ref job_title) = self.request.job_title {
            body["job_title"] = json!(job_title);
        }

        self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request
    }
}

/// 恢复离职员工响应数据
#[derive(Debug, Deserialize)]
pub struct ResurrectEmployeeResponseData {
    /// 恢复的员工信息
    pub employee: Employee,
}

/// 恢复离职员工响应
#[derive(Debug, Deserialize)]
pub struct ResurrectEmployeeResponse {
    /// 响应数据
    pub data: ResurrectEmployeeResponseData,
}

impl ApiResponseTrait for ResurrectEmployeeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl EmployeeService {
    /// 恢复离职员工
    ///
    /// 将已离职的员工恢复为在职状态。身份验证方式为应用身份验证
    ///
    /// # 参数
    /// - `request`: 恢复离职员工请求
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// 恢复的员工信息
    ///
    /// # 示例
    /// ```rust,ignore
    /// let response = client.directory.v1.employee.resurrect(
    ///     ResurrectEmployeeRequest::builder("employee_id")
    ///         .leader_id("leader_id")
    ///         .department_ids(vec!["department_id"])
    ///         .build(),
    ///     None
    /// ).await?;
    /// ```
    ///
    /// 参考: <https://open.feishu.cn/document/directory-v1/employee/resurrect>
    pub async fn resurrect(
        &self,
        request: ResurrectEmployeeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ResurrectEmployeeResponse>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::PUT;
        api_req.api_path = EndpointBuilder::replace_param(
            DIRECTORY_V1_EMPLOYEE_RESURRECT,
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
    ResurrectEmployeeRequestBuilder,
    EmployeeService,
    ResurrectEmployeeRequest,
    BaseResponse<ResurrectEmployeeResponse>,
    resurrect
);
