//! 恢复离职员工
//!
//! 文档: https://open.feishu.cn/document/directory-v1/employee/resurrect

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 恢复离职员工 Builder
#[derive(Debug, Clone)]
pub struct EmployeeResurrectBuilder {
    config: Config,
    /// 员工 ID
    employee_id: String,
}

impl EmployeeResurrectBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, employee_id: impl Into<String>) -> Self {
        Self {
            config,
            employee_id: employee_id.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EmployeeResurrectResponse> {
        let url = format!(
            "/open-apis/directory/v1/employees/{}/resurrect",
            self.employee_id
        );

        let transport = Transport::new(self.config);
        transport.post::<EmployeeResurrectRequest>(url, EmployeeResurrectRequest {}, None::<&()>).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<EmployeeResurrectResponse> {
        let url = format!(
            "/open-apis/directory/v1/employees/{}/resurrect",
            self.employee_id
        );

        let transport = Transport::new(self.config);
        transport.post_with_option::<EmployeeResurrectRequest>(url, EmployeeResurrectRequest {}, option).await
    }
}

/// 恢复离职员工请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct EmployeeResurrectRequest {}

/// 恢复离职员工响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeResurrectResponse {
    /// 员工 ID
    #[serde(rename = "employee_id")]
    employee_id: String,
    /// 结果消息
    #[serde(rename = "message")]
    message: String,
}

impl ApiResponseTrait for EmployeeResurrectResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
