use reqwest::Method;
use serde::Deserialize;

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
    service::directory::v1::models::{DepartmentIdType, Employee, EmployeeStatus, UserIdType},
};

use super::EmployeeService;

/// 批量获取员工列表请求
#[derive(Default, Clone)]
pub struct FilterEmployeeRequest {
    pub api_req: ApiRequest,
    /// 分页大小，最大值 50
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 员工状态过滤
    pub status: Option<EmployeeStatus>,
    /// 部门ID过滤
    pub department_ids: Option<Vec<String>>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
    /// 部门ID类型
    pub department_id_type: Option<DepartmentIdType>,
}

impl FilterEmployeeRequest {
    /// 创建批量获取员工列表请求的构建器
    pub fn builder() -> FilterEmployeeRequestBuilder {
        FilterEmployeeRequestBuilder::default()
    }
}

/// 批量获取员工列表请求构建器
#[derive(Default)]
pub struct FilterEmployeeRequestBuilder {
    request: FilterEmployeeRequest,
}

impl FilterEmployeeRequestBuilder {
    /// 设置分页大小，最大值 50
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 设置员工状态过滤
    pub fn status(mut self, status: EmployeeStatus) -> Self {
        self.request.status = Some(status);
        self
    }

    /// 设置部门ID过滤
    pub fn department_ids(mut self, department_ids: Vec<String>) -> Self {
        self.request.department_ids = Some(department_ids);
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
    pub fn build(mut self) -> FilterEmployeeRequest {
        // 构建查询参数
        if let Some(page_size) = self.request.page_size {
            self.request
                .api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        if let Some(ref page_token) = self.request.page_token {
            self.request
                .api_req
                .query_params
                .insert("page_token", page_token.clone());
        }

        if let Some(ref status) = self.request.status {
            self.request.api_req.query_params.insert(
                "status",
                serde_json::to_string(status)
                    .unwrap_or_default()
                    .trim_matches('"')
                    .to_string(),
            );
        }

        if let Some(ref department_ids) = self.request.department_ids {
            self.request
                .api_req
                .query_params
                .insert("department_ids", department_ids.join(","));
        }

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

        self.request
    }
}

/// 批量获取员工列表响应数据
#[derive(Debug, Deserialize)]
pub struct FilterEmployeeResponseData {
    /// 是否还有更多项
    pub has_more: bool,
    /// 下次遍历的分页标记
    pub page_token: Option<String>,
    /// 员工列表
    pub employees: Vec<Employee>,
}

/// 批量获取员工列表响应
#[derive(Debug, Deserialize)]
pub struct FilterEmployeeResponse {
    /// 响应数据
    pub data: FilterEmployeeResponseData,
}

impl ApiResponseTrait for FilterEmployeeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl EmployeeService {
    /// 批量获取员工列表
    ///
    /// 根据过滤条件批量获取员工列表。身份验证方式为应用身份验证
    ///
    /// # 参数
    /// - `request`: 批量获取员工列表请求
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// 员工列表
    ///
    /// # 示例
    /// ```rust,ignore
    /// let response = client.directory.v1.employee.filter(
    ///     FilterEmployeeRequest::builder()
    ///         .page_size(20)
    ///         .status(EmployeeStatus::Active)
    ///         .build(),
    ///     None
    /// ).await?;
    /// ```
    ///
    /// 参考: <https://open.feishu.cn/document/directory-v1/employee/filter>
    pub async fn filter(
        &self,
        request: FilterEmployeeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<FilterEmployeeResponse>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = DIRECTORY_V1_EMPLOYEES_FILTER.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

// 应用ExecutableBuilder宏
impl_executable_builder_owned!(
    FilterEmployeeRequestBuilder,
    EmployeeService,
    FilterEmployeeRequest,
    BaseResponse<FilterEmployeeResponse>,
    filter
);
