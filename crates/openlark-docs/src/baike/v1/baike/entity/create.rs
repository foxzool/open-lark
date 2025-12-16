/// 创建免审词条
///
/// API文档: https://open.feishu.cn/document/server-docs/baike-v1/entity/create
///
/// 通过此接口创建的词条，无需经过词典管理员审核，直接写入词库。
/// 因此，调用此接口时，应当慎重操作。

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

/// 创建免审词条请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEntityRequest {
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

/// 创建免审词条响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEntityResponse {
    /// 词条ID
    pub entity_id: String,
    /// 创建时间
    pub create_time: String,
}

/// 创建免审词条构建器
pub struct CreateEntityBuilder<'a> {
    config: &'a Config,
    request: CreateEntityRequest,
}

impl<'a> CreateEntityBuilder<'a> {
    /// 创建新的创建免审词条构建器
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            request: CreateEntityRequest {
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

    /// 执行创建免审词条操作
    pub async fn execute(self) -> SDKResult<CreateEntityResponse> {
        let mut api_request: ApiRequest<CreateEntityResponse> = ApiRequest::post("/open-apis/baike/v1/entities")
            .body(serde_json::to_value(&self.request)?);

        let http_request = UnifiedRequestBuilder::build(
            &mut api_request,
            AccessTokenType::App,
            self.config,
            &RequestOption::default(),
        ).await?;

        let response = http_request.send().await?;
        let resp: Response<_> = response.json().await?;
        resp.into_result()
    }
}