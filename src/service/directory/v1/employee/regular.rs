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
    service::directory::v1::models::{Employee, UserIdType},
};

use super::EmployeeService;

/// 更新待离职成员为在职请求
#[derive(Default, Clone)]
pub struct RegularEmployeeRequest {
    pub api_req: ApiRequest,
    /// 员工ID
    pub employee_id: String,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
}

impl RegularEmployeeRequest {
    /// 创建更新待离职成员为在职请求的构建器
    pub fn builder(employee_id: impl ToString) -> RegularEmployeeRequestBuilder {
        RegularEmployeeRequestBuilder {
            request: RegularEmployeeRequest {
                employee_id: employee_id.to_string(),
                ..Default::default()
            },
        }
    }
}

/// 更新待离职成员为在职请求构建器
#[derive(Default)]
pub struct RegularEmployeeRequestBuilder {
    request: RegularEmployeeRequest,
}

impl RegularEmployeeRequestBuilder {
    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.request.user_id_type = Some(user_id_type);
        self
    }

    /// 构建请求
    pub fn build(mut self) -> RegularEmployeeRequest {
        // 构建查询参数
        if let Some(user_id_type) = &self.request.user_id_type {
            self.request
                .api_req
                .query_params
                .insert("user_id_type", user_id_type.to_string());
        }

        // 构建空的请求体
        let body = json!({});
        self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request
    }
}

/// 更新待离职成员为在职响应数据
#[derive(Debug, Deserialize)]
pub struct RegularEmployeeResponseData {
    /// 更新的员工信息
    pub employee: Employee,
}

/// 更新待离职成员为在职响应
#[derive(Debug, Deserialize)]
pub struct RegularEmployeeResponse {
    /// 响应数据
    pub data: RegularEmployeeResponseData,
}

impl ApiResponseTrait for RegularEmployeeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl EmployeeService {
    /// 更新待离职成员为在职
    ///
    /// 将待离职员工状态恢复为在职状态。身份验证方式为应用身份验证
    ///
    /// # 参数
    /// - `request`: 更新待离职成员为在职请求
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// 更新的员工信息
    ///
    /// # 示例
    /// ```rust,ignore
    /// let response = client.directory.v1.employee.regular(
    ///     RegularEmployeeRequest::builder("employee_id")
    ///         .user_id_type(UserIdType::UserId)
    ///         .build(),
    ///     None
    /// ).await?;
    /// ```
    ///
    /// 参考: <https://open.feishu.cn/document/directory-v1/employee/regular>
    pub async fn regular(
        &self,
        request: RegularEmployeeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RegularEmployeeResponse>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = EndpointBuilder::replace_param(
            DIRECTORY_V1_EMPLOYEE_REGULAR,
            "employee_id",
            &request.employee_id,
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

// 应用ExecutableBuilder宏
impl_executable_builder_owned!(
    RegularEmployeeRequestBuilder,
    EmployeeService,
    RegularEmployeeRequest,
    BaseResponse<RegularEmployeeResponse>,
    regular
);
