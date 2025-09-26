use reqwest::Method;
use serde::Deserialize;

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
    service::directory::v1::models::DepartmentIdType,
};

use super::DepartmentService;

/// 删除部门请求
#[derive(Default, Clone)]
pub struct DeleteDepartmentRequest {
    pub api_req: ApiRequest,
    /// 部门ID
    pub department_id: String,
    /// 部门ID类型
    pub department_id_type: Option<DepartmentIdType>,
}

impl DeleteDepartmentRequest {
    /// 创建删除部门请求的构建器
    pub fn builder(department_id: impl ToString) -> DeleteDepartmentRequestBuilder {
        DeleteDepartmentRequestBuilder::new(department_id)
    }
}

/// 删除部门请求构建器
#[derive(Default)]
pub struct DeleteDepartmentRequestBuilder {
    request: DeleteDepartmentRequest,
}

impl DeleteDepartmentRequestBuilder {
    /// 创建新的构建器
    pub fn new(department_id: impl ToString) -> Self {
        Self {
            request: DeleteDepartmentRequest {
                department_id: department_id.to_string(),
                ..Default::default()
            },
        }
    }

    /// 设置部门ID类型
    pub fn department_id_type(mut self, department_id_type: DepartmentIdType) -> Self {
        self.request.department_id_type = Some(department_id_type);
        self
    }

    /// 构建请求
    pub fn build(mut self) -> DeleteDepartmentRequest {
        // 构建查询参数
        if let Some(department_id_type) = &self.request.department_id_type {
            self.request
                .api_req
                .query_params
                .insert("department_id_type", department_id_type.to_string());
        }

        self.request
    }
}

/// 删除部门响应
#[derive(Debug, Deserialize)]
pub struct DeleteDepartmentResponse {
    /// 是否删除成功
    pub deleted: Option<bool>,
}

impl ApiResponseTrait for DeleteDepartmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DepartmentService {
    /// 删除部门
    ///
    /// 删除一个已存在的部门。身份验证方式为应用身份验证
    ///
    /// # 参数
    /// - `request`: 删除部门请求
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// 删除操作结果
    ///
    /// # 示例
    /// ```rust,ignore
    /// let response = client.directory.v1.department.delete(
    ///     DeleteDepartmentRequest::builder("department_id")
    ///         .department_id_type(DepartmentIdType::DepartmentId)
    ///         .build(),
    ///     None
    /// ).await?;
    /// ```
    ///
    /// 参考: <https://open.feishu.cn/document/directory-v1/department/delete>
    pub async fn delete(
        &self,
        request: DeleteDepartmentRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DeleteDepartmentResponse>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::DELETE;
        api_req.api_path = EndpointBuilder::replace_param(
            DIRECTORY_V1_DEPARTMENT_GET,
            "department_id",
            &request.department_id,
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

// 应用ExecutableBuilder宏
impl_executable_builder_owned!(
    DeleteDepartmentRequestBuilder,
    DepartmentService,
    DeleteDepartmentRequest,
    BaseResponse<DeleteDepartmentResponse>,
    delete
);
