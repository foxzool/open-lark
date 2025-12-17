/// 删除免审词条
///
/// API文档: https://open.feishu.cn/document/lingo-v1/entity/delete
use openlark_core::{
    api::{ApiRequest, Response},
    config::Config,
    constants::AccessTokenType,
    error::SDKResult,
    req_option::RequestOption,
    request_builder::UnifiedRequestBuilder,
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
        let mut api_request: ApiRequest<DeleteEntityResponse> = ApiRequest::delete(&path);

        let http_request = UnifiedRequestBuilder::build(
            &mut api_request,
            AccessTokenType::App,
            self.config,
            &RequestOption::default(),
        )
        .await?;

        let response = http_request.send().await?;
        let resp: Response<_> = response.json().await?;
        resp.into_result()
    }
}
