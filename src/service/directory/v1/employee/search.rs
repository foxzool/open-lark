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

/// 搜索员工请求
#[derive(Default, Clone)]
pub struct SearchEmployeeRequest {
    pub api_req: ApiRequest,
    /// 搜索查询词
    pub query: String,
    /// 搜索范围限制在指定部门
    pub department_id: Option<String>,
    /// 页码，从1开始
    pub page_token: Option<String>,
    /// 页面大小，默认10，最大100
    pub page_size: Option<i32>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
    /// 部门ID类型
    pub department_id_type: Option<DepartmentIdType>,
}

impl SearchEmployeeRequest {
    /// 创建搜索员工请求的构建器
    pub fn builder(query: impl ToString) -> SearchEmployeeRequestBuilder {
        SearchEmployeeRequestBuilder {
            request: SearchEmployeeRequest {
                query: query.to_string(),
                ..Default::default()
            },
        }
    }
}

/// 搜索员工请求构建器
#[derive(Default)]
pub struct SearchEmployeeRequestBuilder {
    request: SearchEmployeeRequest,
}

impl SearchEmployeeRequestBuilder {
    /// 设置搜索范围限制在指定部门
    pub fn department_id(mut self, department_id: impl ToString) -> Self {
        self.request.department_id = Some(department_id.to_string());
        self
    }

    /// 设置页码
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 设置页面大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
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
    pub fn build(mut self) -> SearchEmployeeRequest {
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

        if let Some(page_token) = &self.request.page_token {
            self.request
                .api_req
                .query_params
                .insert("page_token", page_token.clone());
        }

        if let Some(page_size) = &self.request.page_size {
            self.request
                .api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        // 构建请求体
        let mut body = json!({
            "query": self.request.query
        });

        if let Some(ref department_id) = self.request.department_id {
            body["department_id"] = json!(department_id);
        }

        self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request
    }
}

/// 搜索员工响应数据
#[derive(Debug, Deserialize)]
pub struct SearchEmployeeResponseData {
    /// 员工信息列表
    pub employees: Vec<Employee>,
    /// 下一页页码
    pub page_token: Option<String>,
    /// 是否还有更多数据
    pub has_more: bool,
}

/// 搜索员工响应
#[derive(Debug, Deserialize)]
pub struct SearchEmployeeResponse {
    /// 响应数据
    pub data: SearchEmployeeResponseData,
}

impl ApiResponseTrait for SearchEmployeeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl EmployeeService {
    /// 搜索员工
    ///
    /// 根据关键词搜索员工信息，支持按姓名、邮箱、工号等字段搜索。身份验证方式为应用身份验证
    ///
    /// # 参数
    /// - `request`: 搜索员工请求
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// 搜索到的员工信息列表
    ///
    /// # 示例
    /// ```rust,ignore
    /// let response = client.directory.v1.employee.search(
    ///     SearchEmployeeRequest::builder("张三")
    ///         .department_id("department_id")
    ///         .page_size(20)
    ///         .build(),
    ///     None
    /// ).await?;
    /// ```
    ///
    /// 参考: <https://open.feishu.cn/document/directory-v1/employee/search>
    pub async fn search(
        &self,
        request: SearchEmployeeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SearchEmployeeResponse>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = DIRECTORY_V1_EMPLOYEES_SEARCH.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

// 应用ExecutableBuilder宏
impl_executable_builder_owned!(
    SearchEmployeeRequestBuilder,
    EmployeeService,
    SearchEmployeeRequest,
    BaseResponse<SearchEmployeeResponse>,
    search
);
