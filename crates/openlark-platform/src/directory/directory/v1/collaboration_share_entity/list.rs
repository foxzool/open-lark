//! 获取关联组织双方共享成员范围
//!
//! 文档: https://open.feishu.cn/document/trust_party-v1/-collaboraiton-organization/list-3

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取关联组织双方共享成员范围 Builder
#[derive(Debug, Clone)]
pub struct CollaborationShareEntityListBuilder {
    config: Config,
    /// 租户 ID
    tenant_id: Option<String>,
}

impl CollaborationShareEntityListBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self { config, tenant_id: None }
    }

    /// 设置租户 ID
    pub fn tenant_id(mut self, tenant_id: impl Into<String>) -> Self {
        self.tenant_id = Some(tenant_id.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CollaborationShareEntityListResponse> {
        let url = "/open-apis/directory/v1/share_entities".to_string();

        let transport = Transport::new(self.config);
        transport
            .get_with_query(url, vec![("tenant_id", self.tenant_id)])
            .await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CollaborationShareEntityListResponse> {
        let url = "/open-apis/directory/v1/share_entities".to_string();

        let transport = Transport::new(self.config);
        transport
            .get_with_query_and_option(url, vec![("tenant_id", self.tenant_id)], option)
            .await
    }
}

/// 共享成员信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ShareEntityInfo {
    /// 实体 ID
    #[serde(rename = "entity_id")]
    pub entity_id: String,
    /// 实体类型
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// 实体名称
    #[serde(rename = "entity_name")]
    pub entity_name: String,
}

/// 获取关联组织双方共享成员范围响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollaborationShareEntityListResponse {
    /// 共享成员列表
    #[serde(rename = "items")]
    pub items: Vec<ShareEntityInfo>,
}

impl ApiResponseTrait for CollaborationShareEntityListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
