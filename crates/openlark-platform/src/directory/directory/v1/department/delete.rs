//! 删除部门
//!
//! 文档: https://open.feishu.cn/document/directory-v1/department/delete

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 删除部门 Builder
#[derive(Debug, Clone)]
pub struct DepartmentDeleteBuilder {
    config: Config,
    /// 部门 ID
    department_id: String,
}

impl DepartmentDeleteBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, department_id: impl Into<String>) -> Self {
        Self {
            config,
            department_id: department_id.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DepartmentDeleteResponse> {
        let url = format!(
            "/open-apis/directory/v1/departments/{}",
            self.department_id
        );

        let transport = Transport::new(self.config);
        transport.delete::<DepartmentDeleteRequest>(url, DepartmentDeleteRequest {}).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<DepartmentDeleteResponse> {
        let url = format!(
            "/open-apis/directory/v1/departments/{}",
            self.department_id
        );

        let transport = Transport::new(self.config);
        transport.delete_with_option::<DepartmentDeleteRequest>(url, DepartmentDeleteRequest {}, option).await
    }
}

/// 删除部门请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct DepartmentDeleteRequest {}

/// 删除部门响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentDeleteResponse {
    /// 部门 ID
    #[serde(rename = "department_id")]
    department_id: String,
    /// 结果消息
    #[serde(rename = "message")]
    message: String,
}

impl ApiResponseTrait for DepartmentDeleteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
