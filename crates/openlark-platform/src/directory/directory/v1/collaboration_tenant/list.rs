//! 管理员获取所有关联组织列表
//!
//! 文档: https://open.feishu.cn/document/trust_party-v1/-collaboraiton-organization/list-2

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 管理员获取所有关联组织列表 Builder
#[derive(Debug, Clone)]
pub struct CollaborationTenantListBuilder {
    config: Config,
}

impl CollaborationTenantListBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CollaborationTenantListResponse> {
        let url = "/open-apis/directory/v1/collaboration_tenants".to_string();

        let transport = Transport::new(self.config);
        transport.get(url, None::<&()>).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<CollaborationTenantListResponse> {
        let url = "/open-apis/directory/v1/collaboration_tenants".to_string();

        let transport = Transport::new(self.config);
        transport.get_with_option(url, option).await
    }
}

/// 关联组织信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollaborationTenantInfo {
    /// 租户 ID
    #[serde(rename = "tenant_id")]
    tenant_id: String,
    /// 租户名称
    #[serde(rename = "tenant_name")]
    tenant_name: String,
    /// 租户类型
    #[serde(rename = "tenant_type")]
    tenant_type: String,
    /// 创建时间
    #[serde(rename = "created_at")]
    created_at: i64,
}

/// 管理员获取所有关联组织列表响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollaborationTenantListResponse {
    /// 关联组织列表
    #[serde(rename = "items")]
    items: Vec<CollaborationTenantInfo>,
}

impl ApiResponseTrait for CollaborationTenantListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
