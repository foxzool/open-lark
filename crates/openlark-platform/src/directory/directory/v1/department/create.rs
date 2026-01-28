//! 创建部门
//!
//! 文档: https://open.feishu.cn/document/directory-v1/department/create

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 创建部门 Builder
#[derive(Debug, Clone)]
pub struct DepartmentCreateBuilder {
    config: Config,
    /// 部门名称
    name: String,
    /// 父部门 ID
    parent_id: Option<String>,
    /// 部门负责人 ID
    leader_user_id: Option<String>,
}

impl DepartmentCreateBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, name: impl Into<String>) -> Self {
        Self {
            config,
            name: name.into(),
            parent_id: None,
            leader_user_id: None,
        }
    }

    /// 设置父部门 ID
    pub fn parent_id(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }

    /// 设置部门负责人 ID
    pub fn leader_user_id(mut self, leader_user_id: impl Into<String>) -> Self {
        self.leader_user_id = Some(leader_user_id.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DepartmentCreateResponse> {
        let url = "/open-apis/directory/v1/departments".to_string();

        let request = DepartmentCreateRequest {
            name: self.name,
            parent_id: self.parent_id,
            leader_user_id: self.leader_user_id,
        };

        let transport = Transport::new(self.config);
        transport.post(url, request, None::<&()>).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<DepartmentCreateResponse> {
        let url = "/open-apis/directory/v1/departments".to_string();

        let request = DepartmentCreateRequest {
            name: self.name,
            parent_id: self.parent_id,
            leader_user_id: self.leader_user_id,
        };

        let transport = Transport::new(self.config);
        transport.post(url, request, Some(option)).await
    }
}

/// 创建部门请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct DepartmentCreateRequest {
    /// 部门名称
    #[serde(rename = "name")]
    name: String,
    /// 父部门 ID
    #[serde(rename = "parent_id", skip_serializing_if = "Option::is_none")]
    parent_id: Option<String>,
    /// 部门负责人 ID
    #[serde(rename = "leader_user_id", skip_serializing_if = "Option::is_none")]
    leader_user_id: Option<String>,
}

/// 创建部门响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentCreateResponse {
    /// 部门 ID
    #[serde(rename = "department_id")]
    department_id: String,
    /// 部门名称
    #[serde(rename = "name")]
    name: String,
    /// 创建时间
    #[serde(rename = "created_at")]
    created_at: i64,
}

impl ApiResponseTrait for DepartmentCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
