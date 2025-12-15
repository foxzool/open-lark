/// 更新免审词条
///
/// API文档: https://open.feishu.cn/document/lingo-v1/entity/update
///
/// 通过此接口更新已有的词条，无需经过词典管理员审核，直接写入词库。
/// 因此，调用该接口时应当慎重操作。

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

/// 更新免审词条请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEntityRequest {
    /// 词条ID
    pub entity_id: String,
    /// 词条名称
    pub name: String,
    /// 词条释义
    pub definition: String,
    /// 词条别名
    pub aliases: Option<Vec<String>>,
    /// 分类ID列表
    pub classification_ids: Option<Vec<String>>,
    /// 词条封面
    pub cover: Option<EntityCover>,
    /// 扩展属性
    pub extra: Option<serde_json::Value>,
}

/// 更新免审词条响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEntityResponse {
    /// 词条ID
    pub entity_id: String,
    /// 更新时间
    pub update_time: String,
}

/// 更新免审词条构建器
pub struct UpdateEntityBuilder<'a> {
    config: &'a Config,
    request: UpdateEntityRequest,
}

impl<'a> UpdateEntityBuilder<'a> {
    /// 创建新的更新免审词条构建器
    pub fn new(config: &'a Config, entity_id: String) -> Self {
        Self {
            config,
            request: UpdateEntityRequest {
                entity_id,
                name: String::new(),
                definition: String::new(),
                aliases: None,
                classification_ids: None,
                cover: None,
                extra: None,
            },
        }
    }

    /// 设置词条名称
    pub fn name(mut self, name: String) -> Self {
        self.request.name = name;
        self
    }

    /// 设置词条释义
    pub fn definition(mut self, definition: String) -> Self {
        self.request.definition = definition;
        self
    }

    /// 设置词条别名
    pub fn aliases(mut self, aliases: Vec<String>) -> Self {
        self.request.aliases = Some(aliases);
        self
    }

    /// 设置分类ID列表
    pub fn classification_ids(mut self, classification_ids: Vec<String>) -> Self {
        self.request.classification_ids = Some(classification_ids);
        self
    }

    /// 设置词条封面
    pub fn cover(mut self, cover: EntityCover) -> Self {
        self.request.cover = Some(cover);
        self
    }

    /// 设置扩展属性
    pub fn extra(mut self, extra: serde_json::Value) -> Self {
        self.request.extra = Some(extra);
        self
    }

    /// 执行更新免审词条操作
    pub async fn execute(self) -> SDKResult<UpdateEntityResponse> {
        let path = format!("/open-apis/lingo/v1/entities/{}", self.request.entity_id);
        let mut api_request = ApiRequest::put(&path)
            .body(serde_json::to_value(&self.request)?);

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