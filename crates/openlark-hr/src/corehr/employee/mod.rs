use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api::ApiRequest,
        api::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::corehr::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::corehr::models::{
        Employee, EmployeeBatchGetRequest, EmployeeSearchRequest, PageResponse,
    },
};

/// 员工信息服务
pub struct EmployeeService {
    pub config: Config,
}

/// 批量查询员工信息响应
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeBatchGetResponse {
    /// 员工信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Employee>>,
}

impl ApiResponseTrait for EmployeeBatchGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索员工信息响应
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeSearchResponse {
    /// 员工信息列表
    #[serde(flatten)]
    pub employees: PageResponse<Employee>,
}

impl ApiResponseTrait for EmployeeSearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl EmployeeService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 批量查询员工信息
    ///
    /// 该接口用于通过员工ID列表批量获取员工的详细信息，包括个人信息、
    /// 雇佣信息、任职信息等完整的员工档案数据。
    ///
    /// # 参数
    ///
    /// - `request`: 批量查询请求参数，包括：
    ///   - `employee_ids`: 员工ID列表
    ///   - `user_id_type`: 用户ID类型
    ///   - `employee_id_type`: 员工ID类型
    ///   - `department_id_type`: 部门ID类型
    ///   - `fields`: 查询字段列表
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回员工信息列表，包括：
    /// - 员工基本信息（工号、状态等）
    /// - 个人信息（姓名、性别、身份证等）
    /// - 雇佣信息（入职日期、雇佣类型等）
    /// - 任职信息（职位、部门、汇报关系等）
    /// - 自定义字段
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::corehr::models::EmployeeBatchGetRequest;
    ///
    /// let request = EmployeeBatchGetRequest {
    ///     employee_ids: vec!["emp_123".to_string(), "emp_456".to_string()],
    ///     user_id_type: Some("open_id".to_string()),
    ///     employee_id_type: Some("employee_id".to_string()),
    ///     department_id_type: Some("open_department_id".to_string()),
    ///     fields: Some(vec![
    ///         "person".to_string(),
    ///         "employment".to_string(),
    ///         "job_datas".to_string(),
    ///     ]),
    /// };
    ///
    /// let response = client.corehr.employee.batch_get(request, None).await?;
    /// ```
    pub async fn batch_get(
        &self,
        request: EmployeeBatchGetRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<EmployeeBatchGetResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: COREHR_EMPLOYEES_BATCH_GET.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        // Content-Type 由 Transport 层自动设置

        Transport::request(api_req, &self.config, option).await
    }

    /// 搜索员工信息
    ///
    /// 该接口用于根据多种条件搜索员工信息，支持按员工状态、部门、
    /// 工号等条件进行筛选，并支持分页查询。
    ///
    /// # 参数
    ///
    /// - `request`: 搜索请求参数，包括：
    ///   - `page_size`: 分页大小
    ///   - `page_token`: 分页标记
    ///   - `employment_status`: 员工状态列表
    ///   - `department_ids`: 部门ID列表
    ///   - `employee_number`: 员工工号
    ///   - `fields`: 查询字段列表
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的员工信息列表，包括：
    /// - 符合条件的员工列表
    /// - 分页信息（是否有更多数据、下一页标记）
    /// - 员工完整档案信息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::corehr::models::EmployeeSearchRequest;
    ///
    /// let request = EmployeeSearchRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     employment_status: Some(vec!["active".to_string()]),
    ///     department_ids: Some(vec!["dept_123".to_string()]),
    ///     employee_number: None,
    ///     user_id_type: Some("open_id".to_string()),
    ///     employee_id_type: Some("employee_id".to_string()),
    ///     department_id_type: Some("open_department_id".to_string()),
    ///     fields: Some(vec![
    ///         "person".to_string(),
    ///         "employment".to_string(),
    ///         "job_datas".to_string(),
    ///     ]),
    /// };
    ///
    /// let response = client.corehr.employee.search(request, None).await?;
    /// ```
    pub async fn search(
        &self,
        request: EmployeeSearchRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<EmployeeSearchResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: COREHR_EMPLOYEES_SEARCH.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        // Content-Type 由 Transport 层自动设置

        Transport::request(api_req, &self.config, option).await
    }
}
