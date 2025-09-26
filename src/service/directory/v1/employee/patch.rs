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

/// 更新员工请求
#[derive(Default, Clone)]
pub struct PatchEmployeeRequest {
    pub api_req: ApiRequest,
    /// 员工ID
    pub employee_id: String,
    /// 员工工号
    pub employee_no: Option<String>,
    /// 姓名
    pub name: Option<String>,
    /// 英文名
    pub en_name: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 手机号
    pub mobile: Option<String>,
    /// 部门ID列表
    pub department_ids: Option<Vec<String>>,
    /// 工作地点
    pub work_location: Option<String>,
    /// 职级
    pub job_level: Option<String>,
    /// 职位
    pub job_title: Option<String>,
    /// 上级ID
    pub leader_id: Option<String>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
    /// 部门ID类型
    pub department_id_type: Option<DepartmentIdType>,
}

impl PatchEmployeeRequest {
    /// 创建更新员工请求的构建器
    pub fn builder(employee_id: impl ToString) -> PatchEmployeeRequestBuilder {
        PatchEmployeeRequestBuilder {
            request: PatchEmployeeRequest {
                employee_id: employee_id.to_string(),
                ..Default::default()
            },
        }
    }
}

/// 更新员工请求构建器
#[derive(Default)]
pub struct PatchEmployeeRequestBuilder {
    request: PatchEmployeeRequest,
}

impl PatchEmployeeRequestBuilder {
    /// 设置员工工号
    pub fn employee_no(mut self, employee_no: impl ToString) -> Self {
        self.request.employee_no = Some(employee_no.to_string());
        self
    }

    /// 设置姓名
    pub fn name(mut self, name: impl ToString) -> Self {
        self.request.name = Some(name.to_string());
        self
    }

    /// 设置英文名
    pub fn en_name(mut self, en_name: impl ToString) -> Self {
        self.request.en_name = Some(en_name.to_string());
        self
    }

    /// 设置邮箱
    pub fn email(mut self, email: impl ToString) -> Self {
        self.request.email = Some(email.to_string());
        self
    }

    /// 设置手机号
    pub fn mobile(mut self, mobile: impl ToString) -> Self {
        self.request.mobile = Some(mobile.to_string());
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

    /// 设置上级ID
    pub fn leader_id(mut self, leader_id: impl ToString) -> Self {
        self.request.leader_id = Some(leader_id.to_string());
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
    pub fn build(mut self) -> PatchEmployeeRequest {
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

        if let Some(ref employee_no) = self.request.employee_no {
            body["employee_no"] = json!(employee_no);
        }

        if let Some(ref name) = self.request.name {
            body["name"] = json!(name);
        }

        if let Some(ref en_name) = self.request.en_name {
            body["en_name"] = json!(en_name);
        }

        if let Some(ref email) = self.request.email {
            body["email"] = json!(email);
        }

        if let Some(ref mobile) = self.request.mobile {
            body["mobile"] = json!(mobile);
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

        if let Some(ref leader_id) = self.request.leader_id {
            body["leader_id"] = json!(leader_id);
        }

        self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request
    }
}

/// 更新员工响应数据
#[derive(Debug, Deserialize)]
pub struct PatchEmployeeResponseData {
    /// 更新的员工信息
    pub employee: Employee,
}

/// 更新员工响应
#[derive(Debug, Deserialize)]
pub struct PatchEmployeeResponse {
    /// 响应数据
    pub data: PatchEmployeeResponseData,
}

impl ApiResponseTrait for PatchEmployeeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl EmployeeService {
    /// 更新员工
    ///
    /// 更新员工信息。身份验证方式为应用身份验证
    ///
    /// # 参数
    /// - `request`: 更新员工请求
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// 更新的员工信息
    ///
    /// # 示例
    /// ```rust,ignore
    /// let response = client.directory.v1.employee.patch(
    ///     PatchEmployeeRequest::builder("employee_id")
    ///         .name("李四")
    ///         .email("lisi@example.com")
    ///         .build(),
    ///     None
    /// ).await?;
    /// ```
    ///
    /// 参考: <https://open.feishu.cn/document/directory-v1/employee/patch>
    pub async fn patch(
        &self,
        request: PatchEmployeeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<PatchEmployeeResponse>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::PATCH;
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
    PatchEmployeeRequestBuilder,
    EmployeeService,
    PatchEmployeeRequest,
    BaseResponse<PatchEmployeeResponse>,
    patch
);
