/// 获取词条详情
///
/// API文档: https://open.feishu.cn/document/server-docs/baike-v1/entity/get
///
/// 通过词条 ID 拉取对应的词条详情信息。

use openlark_core::{
    error::SDKResult,
    config::Config,
    request_builder::UnifiedRequestBuilder,
    constants::AccessTokenType,
    api::{ApiRequest, Response},
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use crate::baike::models::*;

/// 获取词条详情响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEntityResponse {
    /// 词条ID
    pub entity_id: String,
    /// 词条名称
    pub name: String,
    /// 词条别名列表
    pub aliases: Vec<String>,
    /// 词条分类
    pub classifications: Vec<Classification>,
    /// 词条释义
    pub definition: String,
    /// 词条状态
    pub status: String,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
    /// 创建者
    pub creator: String,
    /// 词典ID
    pub repo_id: String,
    /// 词条封面图片
    pub cover: Option<EntityCover>,
    /// 词条扩展属性
    pub extra: Option<serde_json::Value>,
}

/// 获取词条详情构建器
pub struct GetEntityBuilder<'a> {
    config: &'a Config,
    entity_id: String,
}

impl<'a> GetEntityBuilder<'a> {
    /// 创建新的获取词条详情构建器
    pub fn new(config: &'a Config, entity_id: String) -> Self {
        Self { config, entity_id }
    }

    /// 执行获取词条详情操作
    pub async fn execute(self) -> SDKResult<GetEntityResponse> {
        let path = format!("/open-apis/baike/v1/entities/{}", self.entity_id);
        let mut api_request = ApiRequest::get(&path);

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