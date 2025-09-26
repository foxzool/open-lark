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

/// 更新在职员工为待离职请求
#[derive(Default, Clone)]
pub struct ToBeResignedEmployeeRequest {
    pub api_req: ApiRequest,
    /// 员工ID
    pub employee_id: String,
    /// 离职时间（毫秒时间戳）
    pub resign_time: Option<i64>,
    /// 离职原因
    pub resign_reason: Option<String>,
    /// 离职类型
    pub resign_type: Option<String>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
}

impl ToBeResignedEmployeeRequest {
    /// 创建更新在职员工为待离职请求的构建器
    pub fn builder(employee_id: impl ToString) -> ToBeResignedEmployeeRequestBuilder {
        ToBeResignedEmployeeRequestBuilder {
            request: ToBeResignedEmployeeRequest {
                employee_id: employee_id.to_string(),
                ..Default::default()
            },
        }
    }
}

/// 更新在职员工为待离职请求构建器
#[derive(Default)]
pub struct ToBeResignedEmployeeRequestBuilder {
    request: ToBeResignedEmployeeRequest,
}

impl ToBeResignedEmployeeRequestBuilder {
    /// 设置离职时间（毫秒时间戳）
    pub fn resign_time(mut self, resign_time: i64) -> Self {
        self.request.resign_time = Some(resign_time);
        self
    }

    /// 设置离职原因
    pub fn resign_reason(mut self, resign_reason: impl ToString) -> Self {
        self.request.resign_reason = Some(resign_reason.to_string());
        self
    }

    /// 设置离职类型
    pub fn resign_type(mut self, resign_type: impl ToString) -> Self {
        self.request.resign_type = Some(resign_type.to_string());
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.request.user_id_type = Some(user_id_type);
        self
    }

    /// 构建请求
    pub fn build(mut self) -> ToBeResignedEmployeeRequest {
        // 构建查询参数
        if let Some(user_id_type) = &self.request.user_id_type {
            self.request
                .api_req
                .query_params
                .insert("user_id_type", user_id_type.to_string());
        }

        // 构建请求体
        let mut body = json!({});

        if let Some(ref resign_time) = self.request.resign_time {
            body["resign_time"] = json!(resign_time);
        }

        if let Some(ref resign_reason) = self.request.resign_reason {
            body["resign_reason"] = json!(resign_reason);
        }

        if let Some(ref resign_type) = self.request.resign_type {
            body["resign_type"] = json!(resign_type);
        }

        self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request
    }
}

/// 更新在职员工为待离职响应数据
#[derive(Debug, Deserialize)]
pub struct ToBeResignedEmployeeResponseData {
    /// 更新的员工信息
    pub employee: Employee,
}

/// 更新在职员工为待离职响应
#[derive(Debug, Deserialize)]
pub struct ToBeResignedEmployeeResponse {
    /// 响应数据
    pub data: ToBeResignedEmployeeResponseData,
}

impl ApiResponseTrait for ToBeResignedEmployeeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl EmployeeService {
    /// 更新在职员工为待离职
    ///
    /// 将在职员工状态更新为待离职。身份验证方式为应用身份验证
    ///
    /// # 参数
    /// - `request`: 更新在职员工为待离职请求
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// 更新的员工信息
    ///
    /// # 示例
    /// ```rust,ignore
    /// let response = client.directory.v1.employee.to_be_resigned(
    ///     ToBeResignedEmployeeRequest::builder("employee_id")
    ///         .resign_time(1640995200000) // 2022-01-01 00:00:00
    ///         .resign_reason("个人原因")
    ///         .resign_type("主动离职")
    ///         .build(),
    ///     None
    /// ).await?;
    /// ```
    ///
    /// 参考: <https://open.feishu.cn/document/directory-v1/employee/to_be_resigned>
    pub async fn to_be_resigned(
        &self,
        request: ToBeResignedEmployeeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ToBeResignedEmployeeResponse>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = EndpointBuilder::replace_param(
            DIRECTORY_V1_EMPLOYEE_TO_BE_RESIGNED,
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
    ToBeResignedEmployeeRequestBuilder,
    EmployeeService,
    ToBeResignedEmployeeRequest,
    BaseResponse<ToBeResignedEmployeeResponse>,
    to_be_resigned
);
