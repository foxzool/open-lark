/// 删除免审词条
///
/// API文档: https://open.feishu.cn/document/lingo-v1/entity/delete

use openlark_core::{
    error::SDKResult,
    config::Config,
    request_builder::UnifiedRequestBuilder,
    constants::AccessTokenType,
    api::{ApiRequest, Response},
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 删除免审词条响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteEntityResponse {
    /// 词条ID
    pub entity_id: String,
    /// 删除时间
    pub delete_time: String,
}

/// 删除免审词条构建器
pub struct DeleteEntityBuilder<'a> {
    config: &'a Config,
    entity_id: String,
}

impl<'a> DeleteEntityBuilder<'a> {
    /// 创建新的删除免审词条构建器
    pub fn new(config: &'a Config, entity_id: String) -> Self {
        Self { config, entity_id }
    }

    /// 执行删除免审词条操作
    pub async fn execute(self) -> SDKResult<DeleteEntityResponse> {
        let path = format!("/open-apis/lingo/v1/entities/{}", self.entity_id);
        let mut api_request = ApiRequest::delete(&path);

        let http_request = UnifiedRequestBuilder::build(
            &mut api_request,
            AccessTokenType::App,
            self.config,
            &RequestOption::default(),
        ).await?;

        let response = self.config.http_client().execute(http_request).await?;
        let raw_response = Response::from_reqwest_response(response).await?;

        raw_response.into_result()
    }
}