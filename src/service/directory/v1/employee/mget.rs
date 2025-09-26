use reqwest::Method;
use serde::Deserialize;
use serde_json::json;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::directory::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
    service::directory::v1::models::{DepartmentIdType, Employee, UserIdType},
};

use super::EmployeeService;

/// 批量获取员工信息请求
#[derive(Default, Clone)]
pub struct MgetEmployeeRequest {
    pub api_req: ApiRequest,
    /// 员工ID列表
    pub employee_ids: Vec<String>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
    /// 部门ID类型
    pub department_id_type: Option<DepartmentIdType>,
}

impl MgetEmployeeRequest {
    /// 创建批量获取员工信息请求的构建器
    pub fn builder(employee_ids: Vec<String>) -> MgetEmployeeRequestBuilder {
        MgetEmployeeRequestBuilder {
            request: MgetEmployeeRequest {
                employee_ids,
                ..Default::default()
            },
        }
    }
}

/// 批量获取员工信息请求构建器
#[derive(Default)]
pub struct MgetEmployeeRequestBuilder {
    request: MgetEmployeeRequest,
}

impl MgetEmployeeRequestBuilder {
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
    pub fn build(mut self) -> MgetEmployeeRequest {
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
        let body = json!({
            "employee_ids": self.request.employee_ids
        });

        self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request
    }
}

/// 批量获取员工信息响应数据
#[derive(Debug, Deserialize)]
pub struct MgetEmployeeResponseData {
    /// 员工信息列表
    pub employees: Vec<Employee>,
}

/// 批量获取员工信息响应
#[derive(Debug, Deserialize)]
pub struct MgetEmployeeResponse {
    /// 响应数据
    pub data: MgetEmployeeResponseData,
}

impl ApiResponseTrait for MgetEmployeeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl EmployeeService {
    /// 批量获取员工信息
    ///
    /// 根据员工ID列表批量获取员工详细信息。身份验证方式为应用身份验证
    ///
    /// # 参数
    /// - `request`: 批量获取员工信息请求
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// 员工信息列表
    ///
    /// # 示例
    /// ```rust,ignore
    /// let response = client.directory.v1.employee.mget(
    ///     MgetEmployeeRequest::builder(vec![
    ///         "employee_id_1".to_string(),
    ///         "employee_id_2".to_string(),
    ///     ])
    ///     .user_id_type(UserIdType::UserId)
    ///     .build(),
    ///     None
    /// ).await?;
    /// ```
    ///
    /// 参考: <https://open.feishu.cn/document/directory-v1/employee/mget>
    pub async fn mget(
        &self,
        request: MgetEmployeeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<MgetEmployeeResponse>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = DIRECTORY_V1_EMPLOYEES_MGET.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

// 应用ExecutableBuilder宏
impl_executable_builder_owned!(
    MgetEmployeeRequestBuilder,
    EmployeeService,
    MgetEmployeeRequest,
    BaseResponse<MgetEmployeeResponse>,
    mget
);
