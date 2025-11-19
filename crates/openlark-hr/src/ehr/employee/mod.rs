use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api::ApiRequest,
        api::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::ehr::models::{Employee, EmployeeListRequest, PageResponse},
};

/// 员工花名册服务
pub struct EmployeeService {
    pub config: Config,
}

/// 批量获取员工花名册信息响应
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeListResponse {
    /// 员工列表
    #[serde(flatten)]
    pub employees: PageResponse<Employee>,
}

impl ApiResponseTrait for EmployeeListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl EmployeeService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 批量获取员工花名册信息
    ///
    /// 该接口用于分页获取企业内员工的花名册信息，支持多种筛选条件
    /// 和自定义字段查询。可以获取员工的基本信息、职位信息、部门信息、
    /// 个人信息、教育经历、工作经历等完整的人事档案数据。
    ///
    /// # 参数
    ///
    /// - `request`: 员工列表查询请求参数，包括：
    ///   - `page_size`: 分页大小，最大值100
    ///   - `page_token`: 分页标记
    ///   - `status`: 员工状态筛选
    ///   - `department_id`: 部门ID筛选
    ///   - `user_id_type`: 用户ID类型
    ///   - `department_id_type`: 部门ID类型
    ///   - `include_resigned`: 是否包含离职员工
    ///   - `fields`: 查询的字段列表
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的员工信息列表，包括：
    /// - 员工基本信息（姓名、工号、邮箱等）
    /// - 职位信息（职位、职级、上级等）
    /// - 部门信息（部门名称、部门路径等）
    /// - 个人信息（性别、生日、身份证等）
    /// - 教育经历和工作经历
    /// - 紧急联系人和银行卡信息
    /// - 社保信息
    /// - 自定义字段
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::ehr::models::EmployeeListRequest;
    ///
    /// let request = EmployeeListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     status: Some("active".to_string()),
    ///     department_id: Some("dept_123".to_string()),
    ///     user_id_type: Some("open_id".to_string()),
    ///     department_id_type: Some("open_department_id".to_string()),
    ///     include_resigned: Some(false),
    ///     fields: Some(vec![
    ///         "name".to_string(),
    ///         "employee_number".to_string(),
    ///         "email".to_string(),
    ///         "department_info".to_string(),
    ///         "job_info".to_string(),
    ///     ]),
    /// };
    ///
    /// let response = client.ehr.employee.list_employees(request, None).await?;
    /// ```
    pub async fn list_employees(
        &self,
        request: EmployeeListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<EmployeeListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: "/open-apis/ehr/v1/employees".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        if let Some(status) = request.status {
            api_req.query_params.insert("status", status);
        }

        if let Some(department_id) = request.department_id {
            api_req.query_params.insert("department_id", department_id);
        }

        if let Some(user_id_type) = request.user_id_type {
            api_req.query_params.insert("user_id_type", user_id_type);
        }

        if let Some(department_id_type) = request.department_id_type {
            api_req
                .query_params
                .insert("department_id_type", department_id_type);
        }

        if let Some(include_resigned) = request.include_resigned {
            api_req
                .query_params
                .insert("include_resigned", include_resigned.to_string());
        }

        if let Some(fields) = request.fields {
            if !fields.is_empty() {
                api_req.query_params.insert("fields", fields.join(","));
            }
        }

        Transport::request(api_req, &self.config, option).await
    }
}
