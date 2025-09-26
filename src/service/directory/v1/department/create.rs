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

/// 创建部门请求
#[derive(Default, Clone)]
pub struct CreateDepartmentRequest {
    pub api_req: ApiRequest,
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

impl CreateDepartmentRequest {
    /// 创建创建部门请求的构建器
    pub fn builder() -> CreateDepartmentRequestBuilder {
        CreateDepartmentRequestBuilder::default()
    }
}

/// 创建部门请求构建器
#[derive(Default)]
pub struct CreateDepartmentRequestBuilder {
    request: CreateDepartmentRequest,
}

impl CreateDepartmentRequestBuilder {
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
    pub fn build(mut self) -> CreateDepartmentRequest {
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

/// 创建部门响应数据
#[derive(Debug, Deserialize)]
pub struct CreateDepartmentResponseData {
    /// 创建的部门信息
    pub department: Department,
}

/// 创建部门响应
#[derive(Debug, Deserialize)]
pub struct CreateDepartmentResponse {
    /// 响应数据
    pub data: CreateDepartmentResponseData,
}

impl ApiResponseTrait for CreateDepartmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DepartmentService {
    /// 创建部门
    ///
    /// 创建一个新的部门。身份验证方式为应用身份验证
    ///
    /// # 参数
    /// - `request`: 创建部门请求
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// 创建的部门信息
    ///
    /// # 示例
    /// ```rust,ignore
    /// let response = client.directory.v1.department.create(
    ///     CreateDepartmentRequest::builder()
    ///         .name("技术部")
    ///         .en_name("Technology Department")
    ///         .build(),
    ///     None
    /// ).await?;
    /// ```
    ///
    /// 参考: <https://open.feishu.cn/document/directory-v1/department/create>
    pub async fn create(
        &self,
        request: CreateDepartmentRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateDepartmentResponse>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = DIRECTORY_V1_DEPARTMENTS.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

// 应用ExecutableBuilder宏
impl_executable_builder_owned!(
    CreateDepartmentRequestBuilder,
    DepartmentService,
    CreateDepartmentRequest,
    BaseResponse<CreateDepartmentResponse>,
    create
);
