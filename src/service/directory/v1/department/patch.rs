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
    service::directory::v1::models::{Department, DepartmentIdType, UserIdType},
};

use super::DepartmentService;

/// 更新部门请求
#[derive(Default, Clone)]
pub struct PatchDepartmentRequest {
    pub api_req: ApiRequest,
    /// 部门ID
    pub department_id: String,
    /// 部门名称
    pub name: Option<String>,
    /// 英文名称
    pub en_name: Option<String>,
    /// 父部门ID
    pub parent_department_id: Option<String>,
    /// 部门主管ID
    pub leader_id: Option<String>,
    /// 部门顺序
    pub order: Option<i32>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
    /// 部门ID类型
    pub department_id_type: Option<DepartmentIdType>,
}

impl PatchDepartmentRequest {
    /// 创建更新部门请求的构建器
    pub fn builder(department_id: impl ToString) -> PatchDepartmentRequestBuilder {
        PatchDepartmentRequestBuilder::new(department_id)
    }
}

/// 更新部门请求构建器
#[derive(Default)]
pub struct PatchDepartmentRequestBuilder {
    request: PatchDepartmentRequest,
}

impl PatchDepartmentRequestBuilder {
    /// 创建新的构建器
    pub fn new(department_id: impl ToString) -> Self {
        Self {
            request: PatchDepartmentRequest {
                department_id: department_id.to_string(),
                ..Default::default()
            },
        }
    }

    /// 设置部门名称
    pub fn name(mut self, name: impl ToString) -> Self {
        self.request.name = Some(name.to_string());
        self
    }

    /// 设置英文名称
    pub fn en_name(mut self, en_name: impl ToString) -> Self {
        self.request.en_name = Some(en_name.to_string());
        self
    }

    /// 设置父部门ID
    pub fn parent_department_id(mut self, parent_department_id: impl ToString) -> Self {
        self.request.parent_department_id = Some(parent_department_id.to_string());
        self
    }

    /// 设置部门主管ID
    pub fn leader_id(mut self, leader_id: impl ToString) -> Self {
        self.request.leader_id = Some(leader_id.to_string());
        self
    }

    /// 设置部门顺序
    pub fn order(mut self, order: i32) -> Self {
        self.request.order = Some(order);
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
    pub fn build(mut self) -> PatchDepartmentRequest {
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

        if let Some(ref name) = self.request.name {
            body["name"] = json!(name);
        }

        if let Some(ref en_name) = self.request.en_name {
            body["en_name"] = json!(en_name);
        }

        if let Some(ref parent_department_id) = self.request.parent_department_id {
            body["parent_department_id"] = json!(parent_department_id);
        }

        if let Some(ref leader_id) = self.request.leader_id {
            body["leader_id"] = json!(leader_id);
        }

        if let Some(order) = self.request.order {
            body["order"] = json!(order);
        }

        self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request
    }
}

/// 更新部门响应数据
#[derive(Debug, Deserialize)]
pub struct PatchDepartmentResponseData {
    /// 更新后的部门信息
    pub department: Department,
}

/// 更新部门响应
#[derive(Debug, Deserialize)]
pub struct PatchDepartmentResponse {
    /// 响应数据
    pub data: PatchDepartmentResponseData,
}

impl ApiResponseTrait for PatchDepartmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DepartmentService {
    /// 更新部门
    ///
    /// 更新一个已存在的部门信息。身份验证方式为应用身份验证
    ///
    /// # 参数
    /// - `request`: 更新部门请求
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// 更新后的部门信息
    ///
    /// # 示例
    /// ```rust,ignore
    /// let response = client.directory.v1.department.patch(
    ///     PatchDepartmentRequest::builder("department_id")
    ///         .name("新技术部")
    ///         .en_name("New Technology Department")
    ///         .build(),
    ///     None
    /// ).await?;
    /// ```
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/department/patch>
    pub async fn patch(
        &self,
        request: PatchDepartmentRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<PatchDepartmentResponse>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::PATCH);
        api_req.set_api_path(EndpointBuilder::replace_param(
            DIRECTORY_V1_DEPARTMENT_GET,
            "department_id",
            &request.department_id,
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

// 应用ExecutableBuilder宏
impl_executable_builder_owned!(
    PatchDepartmentRequestBuilder,
    DepartmentService,
    PatchDepartmentRequest,
    BaseResponse<PatchDepartmentResponse>,
    patch
);
