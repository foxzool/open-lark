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
    service::directory::v1::models::{Department, DepartmentIdType, UserIdType},
};

use super::DepartmentService;

/// 批量获取部门信息请求
#[derive(Default, Clone)]
pub struct MgetDepartmentRequest {
    pub api_req: ApiRequest,
    /// 部门ID列表
    pub department_ids: Vec<String>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
    /// 部门ID类型
    pub department_id_type: Option<DepartmentIdType>,
}

impl MgetDepartmentRequest {
    /// 创建批量获取部门信息请求的构建器
    pub fn builder() -> MgetDepartmentRequestBuilder {
        MgetDepartmentRequestBuilder::default()
    }
}

/// 批量获取部门信息请求构建器
#[derive(Default)]
pub struct MgetDepartmentRequestBuilder {
    request: MgetDepartmentRequest,
}

impl MgetDepartmentRequestBuilder {
    /// 设置部门ID列表
    pub fn department_ids(mut self, department_ids: Vec<String>) -> Self {
        self.request.department_ids = department_ids;
        self
    }

    /// 添加部门ID
    pub fn add_department_id(mut self, department_id: impl ToString) -> Self {
        self.request.department_ids.push(department_id.to_string());
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
    pub fn build(mut self) -> MgetDepartmentRequest {
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
            "department_ids": self.request.department_ids
        });

        self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request
    }
}

/// 批量获取部门信息响应数据
#[derive(Debug, Deserialize)]
pub struct MgetDepartmentResponseData {
    /// 部门信息列表
    pub departments: Vec<Department>,
}

/// 批量获取部门信息响应
#[derive(Debug, Deserialize)]
pub struct MgetDepartmentResponse {
    /// 响应数据
    pub data: MgetDepartmentResponseData,
}

impl ApiResponseTrait for MgetDepartmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DepartmentService {
    /// 批量获取部门信息
    ///
    /// 通过部门ID列表批量获取多个部门的详细信息。身份验证方式为应用身份验证
    ///
    /// # 参数
    /// - `request`: 批量获取部门信息请求
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// 部门信息列表
    ///
    /// # 示例
    /// ```rust,ignore
    /// let response = client.directory.v1.department.mget(
    ///     MgetDepartmentRequest::builder()
    ///         .department_ids(vec!["dept_id_1".to_string(), "dept_id_2".to_string()])
    ///         .department_id_type(DepartmentIdType::DepartmentId)
    ///         .build(),
    ///     None
    /// ).await?;
    /// ```
    ///
    /// 参考: <https://open.feishu.cn/document/directory-v1/department/mget>
    pub async fn mget(
        &self,
        request: MgetDepartmentRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<MgetDepartmentResponse>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = DIRECTORY_V1_DEPARTMENTS_MGET.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

// 应用ExecutableBuilder宏
impl_executable_builder_owned!(
    MgetDepartmentRequestBuilder,
    DepartmentService,
    MgetDepartmentRequest,
    BaseResponse<MgetDepartmentResponse>,
    mget
);
