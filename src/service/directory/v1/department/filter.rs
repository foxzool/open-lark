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
    service::directory::v1::models::{Department, DepartmentIdType, UserIdType},
};

use super::DepartmentService;

/// 批量获取部门列表请求
#[derive(Default, Clone)]
pub struct FilterDepartmentRequest {
    pub api_req: ApiRequest,
    /// 分页大小，最大值为50
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 父部门ID，获取指定父部门下的子部门列表
    pub parent_department_id: Option<String>,
    /// 部门状态过滤
    pub fetch_deleted: Option<bool>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
    /// 部门ID类型
    pub department_id_type: Option<DepartmentIdType>,
}

impl FilterDepartmentRequest {
    /// 创建批量获取部门列表请求的构建器
    pub fn builder() -> FilterDepartmentRequestBuilder {
        FilterDepartmentRequestBuilder::default()
    }
}

/// 批量获取部门列表请求构建器
#[derive(Default)]
pub struct FilterDepartmentRequestBuilder {
    request: FilterDepartmentRequest,
}

impl FilterDepartmentRequestBuilder {
    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 设置父部门ID
    pub fn parent_department_id(mut self, parent_department_id: impl ToString) -> Self {
        self.request.parent_department_id = Some(parent_department_id.to_string());
        self
    }

    /// 设置是否获取已删除部门
    pub fn fetch_deleted(mut self, fetch_deleted: bool) -> Self {
        self.request.fetch_deleted = Some(fetch_deleted);
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
    pub fn build(mut self) -> FilterDepartmentRequest {
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

        if let Some(ref parent_department_id) = self.request.parent_department_id {
            self.request
                .api_req
                .query_params
                .insert("parent_department_id", parent_department_id.clone());
        }

        if let Some(fetch_deleted) = self.request.fetch_deleted {
            self.request
                .api_req
                .query_params
                .insert("fetch_deleted", fetch_deleted.to_string());
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

/// 批量获取部门列表响应数据
#[derive(Debug, Deserialize)]
pub struct FilterDepartmentResponseData {
    /// 部门信息列表
    pub departments: Vec<Department>,
    /// 下一页分页标记
    pub page_token: Option<String>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
}

/// 批量获取部门列表响应
#[derive(Debug, Deserialize)]
pub struct FilterDepartmentResponse {
    /// 响应数据
    pub data: FilterDepartmentResponseData,
}

impl ApiResponseTrait for FilterDepartmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DepartmentService {
    /// 批量获取部门列表
    ///
    /// 获取部门列表，支持分页和过滤条件。身份验证方式为应用身份验证
    ///
    /// # 参数
    /// - `request`: 批量获取部门列表请求
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// 部门列表和分页信息
    ///
    /// # 示例
    /// ```rust,ignore
    /// let response = client.directory.v1.department.filter(
    ///     FilterDepartmentRequest::builder()
    ///         .page_size(20)
    ///         .parent_department_id("root_department_id")
    ///         .department_id_type(DepartmentIdType::DepartmentId)
    ///         .build(),
    ///     None
    /// ).await?;
    /// ```
    ///
    /// 参考: <https://open.feishu.cn/document/directory-v1/department/filter>
    pub async fn filter(
        &self,
        request: FilterDepartmentRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<FilterDepartmentResponse>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = DIRECTORY_V1_DEPARTMENTS_FILTER.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

// 应用ExecutableBuilder宏
impl_executable_builder_owned!(
    FilterDepartmentRequestBuilder,
    DepartmentService,
    FilterDepartmentRequest,
    BaseResponse<FilterDepartmentResponse>,
    filter
);
