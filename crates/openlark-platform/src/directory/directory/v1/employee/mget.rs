//! 批量获取员工信息
//!
//! 文档: https://open.feishu.cn/document/directory-v1/employee/mget

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 批量获取员工信息 Builder
#[derive(Debug, Clone)]
pub struct EmployeeMgetBuilder {
    config: Config,
    /// 员工 ID 列表
    employee_ids: Vec<String>,
}

impl EmployeeMgetBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            employee_ids: Vec::new(),
        }
    }

    /// 添加员工 ID
    pub fn employee_id(mut self, employee_id: impl Into<String>) -> Self {
        self.employee_ids.push(employee_id.into());
        self
    }

    /// 添加多个员工 ID
    pub fn employee_ids(
        mut self,
        employee_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.employee_ids.extend(employee_ids.into_iter().map(Into::into));
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EmployeeMgetResponse> {
        let url = "/open-apis/directory/v1/employees/mget".to_string();

        let request = EmployeeMgetRequest {
            employee_ids: self.employee_ids,
        };

        let transport = Transport::new(self.config);
        transport.post(url, request, None::<&()>).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<EmployeeMgetResponse> {
        let url = "/open-apis/directory/v1/employees/mget".to_string();

        let request = EmployeeMgetRequest {
            employee_ids: self.employee_ids,
        };

        let transport = Transport::new(self.config);
        transport.post(url, request, Some(option)).await
    }
}

/// 批量获取员工信息请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct EmployeeMgetRequest {
    /// 员工 ID 列表
    #[serde(rename = "employee_ids")]
    employee_ids: Vec<String>,
}

/// 员工信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeInfo {
    /// 员工 ID
    #[serde(rename = "employee_id")]
    employee_id: String,
    /// 员工名称
    #[serde(rename = "name")]
    name: String,
    /// 手机号
    #[serde(rename = "mobile")]
    mobile: String,
    /// 邮箱
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    /// 部门 ID 列表
    #[serde(rename = "department_ids")]
    department_ids: Vec<String>,
    /// 状态
    #[serde(rename = "status")]
    status: String,
}

/// 批量获取员工信息响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeMgetResponse {
    /// 员工信息列表
    #[serde(rename = "items")]
    items: Vec<EmployeeInfo>,
}

impl ApiResponseTrait for EmployeeMgetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
